use crate::api::observability::{ActivityKind, AgentObserver};
use crate::api::types::*;
use crate::agent::{Agent, AgentGoal, AgentKind, AgentMessage, AgentRegistry, AgentState};
use crate::db::Database;
use crate::error::ApiError;
use crate::world::pathfind::find_path;
use crate::world::{Grid, Position, Tile, WorldEvent};
use axum::extract::{Path, Query, State};
use axum::response::Json;
use axum::response::sse::{Event, KeepAlive, Sse};
use chrono::Utc;
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Clone)]
pub struct ApiState {
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub grid: Arc<RwLock<Grid>>,
    pub event_tx: mpsc::Sender<WorldEvent>,
    pub event_broadcast: broadcast::Sender<WorldEvent>,
    pub api_key: String,
    pub tick_count: Arc<std::sync::atomic::AtomicU64>,
    pub observer: Arc<RwLock<AgentObserver>>,
    pub db: Arc<Mutex<Database>>,
}

// --- Health (no auth) ---

pub async fn health(State(state): State<ApiState>) -> Result<Json<HealthResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: crate::VERSION.to_string(),
        agents: reg.count(),
    }))
}

// --- Agent CRUD ---

pub async fn list_agents(State(state): State<ApiState>) -> Result<Json<Vec<ApiAgent>>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agents: Vec<ApiAgent> = reg
        .agents()
        .map(|a| ApiAgent {
            id: a.id.to_string(),
            name: a.name.clone(),
            state: a.state.label().to_string(),
            position: (a.position.x, a.position.y),
            task_count: a.task_count,
            speech: a.speech.clone(),
        })
        .collect();
    Ok(Json(agents))
}

pub async fn connect_agent(
    State(state): State<ApiState>,
    Json(req): Json<ConnectRequest>,
) -> Result<Json<ConnectResponse>, ApiError> {
    let position = {
        let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
        grid.find_empty_floor().ok_or(ApiError::ServiceUnavailable(
            "no empty floor available".into(),
        ))?
    };

    let kind = match req.endpoint {
        Some(ep) => AgentKind::External { endpoint: ep },
        None => AgentKind::Local,
    };

    let agent = Agent::new(&req.name, kind, position);
    let agent_id = agent.id;

    {
        let mut grid = state.grid.write().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
        grid.place_agent(position, agent_id);
    }

    {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        reg.register(agent);
    }

    if let Err(e) = state.event_tx.try_send(WorldEvent::AgentSpawned { agent_id, position }) {
        tracing::error!("Failed to send AgentSpawned event: {}", e);
    }

    persist_activity(
        &state.observer,
        &state.db,
        agent_id,
        ActivityKind::Spawned,
        &format!(
            "Agent '{}' connected at ({},{})",
            req.name, position.x, position.y
        ),
    );

    // Persist to database
    {
        let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        if let Some(agent) = reg.get(&agent_id)
            && let Ok(db) = state.db.lock()
        {
            if let Err(e) = db.save_agent(agent) {
                tracing::error!("Failed to save agent to DB: {}", e);
            }
        }
    }

    Ok(Json(ConnectResponse {
        agent_id: agent_id.to_string(),
        position: (position.x, position.y),
    }))
}

pub async fn delete_agent(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
) -> Result<Json<DeleteResponse>, ApiError> {
    let (target_id, position) = {
        let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id(&reg, &agent_id_str)?;
        (agent.id, agent.position)
    };

    {
        let mut grid = state.grid.write().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
        grid.remove_agent(position);
    }

    {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        reg.remove(&target_id);
    }

    if let Err(e) = state.event_tx.try_send(WorldEvent::AgentRemoved { agent_id: target_id }) {
        tracing::error!("Failed to send AgentRemoved event: {}", e);
    }

    persist_activity(
        &state.observer,
        &state.db,
        target_id,
        ActivityKind::Removed,
        "Agent disconnected",
    );
    {
        let mut obs = state.observer.write().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
        obs.remove_agent(&target_id);
    }

    // Remove from database
    if let Ok(db) = state.db.lock() {
        if let Err(e) = db.purge_agent(&target_id) {
            tracing::error!("Failed to purge agent from DB: {}", e);
        }
    }

    Ok(Json(DeleteResponse {
        status: "removed".to_string(),
        agent_id: agent_id_str,
    }))
}

// --- Messaging ---

pub async fn send_agent_message(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(msg): Json<ApiMessage>,
) -> Result<Json<MessageResponse>, ApiError> {
    let (sender_id, target_info, webhook_url) = {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;

        let sender = find_agent_by_id(&reg, &agent_id_str)?;
        let sender_id = sender.id;

        if let Some(ref to_str) = msg.to {
            // Agent-to-agent messaging
            let target = find_agent_by_id(&reg, to_str)?;
            let target_id = target.id;

            let mut webhook_url = None;
            if let Some(target_agent) = reg.get_mut(&target_id) {
                target_agent.say(&msg.text);
                target_agent.set_state(AgentState::Messaging);
                target_agent.anim.activity_ticks = 0;

                // Push to inbox
                let inbox_msg = AgentMessage::new(sender_id, target_id, &msg.text);
                target_agent.inbox.push_back(inbox_msg);

                // Cap inbox at 500 messages
                while target_agent.inbox.len() > 500 {
                    target_agent.inbox.pop_front();
                }

                // Get webhook endpoint for push delivery
                webhook_url = match &target_agent.kind {
                    AgentKind::External { endpoint } => Some(endpoint.clone()),
                    AgentKind::OpenCrabs { endpoint } => Some(endpoint.clone()),
                    AgentKind::Local => None,
                };
            }

            (sender_id, Some((target_id, to_str.clone())), webhook_url)
        } else {
            // Self-message (speech bubble)
            if let Some(agent) = reg.get_mut(&sender_id) {
                agent.say(&msg.text);
                agent.set_state(AgentState::Messaging);
                agent.anim.activity_ticks = 0;
            }
            (sender_id, None, None)
        }
    };

    if let Some((target_id, to_str)) = target_info {
        if let Err(e) = state.event_tx.try_send(WorldEvent::MessageSent {
            from: sender_id,
            to: target_id,
            text: msg.text.clone(),
        }) {
            tracing::error!("Failed to send MessageSent event: {}", e);
        }

        persist_activity(
            &state.observer,
            &state.db,
            sender_id,
            ActivityKind::MessageSent,
            &format!("Sent to {}: {}", &to_str[..8.min(to_str.len())], &msg.text),
        );
        persist_activity(
            &state.observer,
            &state.db,
            target_id,
            ActivityKind::MessageReceived,
            &format!("From {}: {}", &sender_id.to_string()[..8], &msg.text),
        );

        // Persist message to database
        if let Ok(db) = state.db.lock() {
            let msg = AgentMessage::new(sender_id, target_id, &msg.text);
            if let Err(e) = db.save_message(&msg) {
                tracing::error!("Failed to save message to DB: {}", e);
            }
        }

        // Push via webhook if target has an endpoint
        if let Some(url) = webhook_url {
            let payload = serde_json::json!({
                "from": sender_id.0.to_string(),
                "text": msg.text,
                "timestamp": Utc::now().to_rfc3339(),
            });
            let api_key = state.api_key.clone();
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let res = client
                    .post(format!("{}/messages", url))
                    .header("X-API-Key", api_key)
                    .json(&payload)
                    .send()
                    .await;
                if let Err(e) = res {
                    tracing::warn!("Webhook delivery to {} failed: {}", url, e);
                }
            });
        }

        return Ok(Json(MessageResponse {
            status: "delivered".to_string(),
            delivered_to: Some(to_str),
        }));
    }

    persist_activity(
        &state.observer,
        &state.db,
        sender_id,
        ActivityKind::MessageSent,
        &format!("Speech: {}", &msg.text),
    );

    Ok(Json(MessageResponse {
        status: "delivered".to_string(),
        delivered_to: None,
    }))
}

// --- Agent Actions ---

pub async fn move_agent(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<MoveRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let target = Position::new(req.x, req.y);

    // Validate target is within bounds and walkable
    {
        let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
        let cell = grid
            .get(target)
            .ok_or(ApiError::BadRequest("position out of bounds".into()))?;
        if cell.tile.is_solid() {
            return Err(ApiError::BadRequest(
                "target position is not walkable".into(),
            ));
        }
    }

    let agent_id = {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;

        let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
        let path = find_path(&grid, agent.position, target)
            .ok_or(ApiError::BadRequest("no path to target position".into()))?;

        agent.path = path;
        agent.goal = Some(AgentGoal::Wander(target));
        agent.set_state(AgentState::Walking);
        agent.anim.activity_ticks = 0;
        agent.id
    };

    persist_activity(
        &state.observer,
        &state.db,
        agent_id,
        ActivityKind::Movement,
        &format!("Moving to ({},{})", req.x, req.y),
    );

    Ok(Json(serde_json::json!({
        "status": "moving",
        "target": { "x": req.x, "y": req.y }
    })))
}

pub async fn set_agent_goal(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<GoalRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let tile_type = match req.goal.as_str() {
        "desk" => Tile::Desk,
        "vending" => Tile::VendingMachine,
        "coffee" => Tile::CoffeeMachine,
        "pinball" => Tile::PinballMachine,
        "gym" | "treadmill" => Tile::GymTreadmill,
        "weights" => Tile::WeightBench,
        "yoga" => Tile::YogaMat,
        "meeting" => Tile::MeetingTable,
        "server" | "archive" => Tile::ServerRack,
        "couch" => Tile::Couch,
        "wander" => {
            // Wander to random floor
            let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
            let target = grid
                .find_empty_floor()
                .ok_or(ApiError::ServiceUnavailable("no empty floor".into()))?;
            let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
            let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;
            let agent_id = agent.id;
            if let Some(path) = find_path(&grid, agent.position, target) {
                agent.goal = Some(AgentGoal::Wander(target));
                agent.path = path;
                agent.set_state(AgentState::Walking);
                agent.anim.activity_ticks = 0;
            }
            drop(reg);
            persist_activity(
                &state.observer,
                &state.db,
                agent_id,
                ActivityKind::GoalAssigned,
                "Goal: wander",
            );
            return Ok(Json(serde_json::json!({
                "status": "wandering",
                "goal": "wander"
            })));
        }
        other => {
            return Err(ApiError::BadRequest(format!(
                "unknown goal '{}'. Valid: desk, vending, coffee, pinball, gym, treadmill, weights, yoga, meeting, server, couch, wander",
                other
            )));
        }
    };

    let goal_fn: fn(Position) -> AgentGoal = match req.goal.as_str() {
        "desk" => AgentGoal::GoToDesk,
        "vending" => AgentGoal::GoToVending,
        "coffee" => AgentGoal::GoToCoffee,
        "pinball" => AgentGoal::GoToPinball,
        "meeting" => AgentGoal::GoToMeeting,
        "server" | "archive" => AgentGoal::GoToServer,
        "gym" | "treadmill" | "weights" | "yoga" => AgentGoal::GoToGym,
        "couch" => AgentGoal::GoToCouch,
        _ => unreachable!(),
    };

    let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
    let targets = grid.find_tiles(&tile_type);
    if targets.is_empty() {
        return Err(ApiError::NotFound(format!(
            "no {} found in world",
            req.goal
        )));
    }

    // Pick random available target
    use rand::RngExt;
    let target = targets[rand::rng().random_range(0..targets.len())];

    let adj = grid
        .find_adjacent_floor(target)
        .ok_or(ApiError::ServiceUnavailable(
            "no adjacent floor available".into(),
        ))?;

    let agent_id = {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;

        let path = find_path(&grid, agent.position, adj)
            .ok_or(ApiError::BadRequest("no path to target".into()))?;

        agent.goal = Some(goal_fn(target));
        agent.path = path;
        agent.set_state(AgentState::Walking);
        agent.anim.activity_ticks = 0;
        agent.id
    };

    persist_activity(
        &state.observer,
        &state.db,
        agent_id,
        ActivityKind::GoalAssigned,
        &format!("Goal: {}", req.goal),
    );

    Ok(Json(serde_json::json!({
        "status": "heading_to_goal",
        "goal": req.goal,
        "target": { "x": target.x, "y": target.y }
    })))
}

pub async fn set_agent_state(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<StateRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let new_state = parse_agent_state(&req.state)?;

    let agent_id = {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;
        agent.set_state(new_state.clone());
        agent.anim.activity_ticks = 0;

        // Clear path/goal if setting to idle
        if new_state == AgentState::Idle {
            agent.path.clear();
            agent.goal = None;
        }
        agent.id
    };

    persist_activity(
        &state.observer,
        &state.db,
        agent_id,
        ActivityKind::StateChange,
        &format!("State → {}", req.state),
    );

    Ok(Json(serde_json::json!({
        "status": "state_changed",
        "state": req.state
    })))
}

pub async fn rename_agent(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<RenameRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(ApiError::BadRequest("name cannot be empty".into()));
    }

    let agent_id = {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;
        agent.name = name.clone();
        agent.id
    };

    // Persist name change to DB
    if let Ok(db) = state.db.lock() {
        let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        if let Some(agent) = reg.get(&agent_id) {
            if let Err(e) = db.save_agent(agent) {
                tracing::error!("Failed to save agent to DB: {}", e);
            }
        }
    }

    persist_activity(
        &state.observer,
        &state.db,
        agent_id,
        ActivityKind::StateChange,
        &format!("Renamed to '{}'", name),
    );

    Ok(Json(serde_json::json!({
        "status": "renamed",
        "name": name
    })))
}

// --- Task Reporting ---

pub async fn report_task(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<TaskReportRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let task_id = req.task_id.trim().to_string();
    if task_id.is_empty() {
        return Err(ApiError::BadRequest("task_id cannot be empty".into()));
    }

    let valid_states = ["submitted", "running", "completed", "failed"];
    if !valid_states.contains(&req.state.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "invalid task state '{}'. Valid: submitted, running, completed, failed",
            req.state
        )));
    }

    // Record in observer FIRST so we can check for other active tasks
    let agent_id;
    {
        let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        let agent = find_agent_by_id_mut(&mut reg, &agent_id_str)?;
        if req.state == "completed" || req.state == "submitted" {
            agent.task_count += 1;
        }
        agent_id = agent.id;
    }

    {
        let mut obs = state.observer.write().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
        obs.record_task(agent_id, &task_id, &req.state, req.summary.clone());

        // Auto-sync visual state with task lifecycle.
        // api_locked prevents the simulation from overriding this state.
        let is_terminal = req.state == "completed" || req.state == "failed";
        let has_other_active = is_terminal && obs.has_active_tasks(&agent_id);

        let new_state = match req.state.as_str() {
            "submitted" => Some(AgentState::Thinking),
            "running" => Some(AgentState::Working),
            // Only go Idle if no other tasks are still active
            "completed" | "failed" if !has_other_active => Some(AgentState::Idle),
            "completed" | "failed" => None, // keep current state, other tasks still active
            _ => None,
        };

        if let Some(s) = new_state {
            let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
            if let Some(agent) = reg.get_mut(&agent_id) {
                agent.set_state(s.clone());
                agent.anim.activity_ticks = 0;
                agent.api_locked = s != AgentState::Idle;
            }
        }
    }

    // Build activity kind based on task state
    let activity_kind = match req.state.as_str() {
        "submitted" => ActivityKind::TaskSubmitted,
        "completed" => ActivityKind::TaskCompleted,
        "failed" => ActivityKind::TaskFailed,
        _ => ActivityKind::TaskSubmitted,
    };
    let detail = match &req.summary {
        Some(s) => format!("Task {}: {} — {}", task_id, req.state, s),
        None => format!("Task {}: {}", task_id, req.state),
    };
    persist_activity(&state.observer, &state.db, agent_id, activity_kind, &detail);

    // Persist task to database
    if let Ok(db) = state.db.lock() {
        let task = crate::api::observability::TaskRecord {
            task_id: task_id.clone(),
            submitted_at: Utc::now(),
            state: req.state.clone(),
            last_updated: Utc::now(),
            response_summary: req.summary.clone(),
        };
        if let Err(e) = db.save_task(agent_id, &task) {
                tracing::error!("Failed to save task to DB: {}", e);
            }
    }

    // Persist updated agent (task_count)
    if let Ok(db) = state.db.lock() {
        let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        if let Some(agent) = reg.get(&agent_id) {
            if let Err(e) = db.save_agent(agent) {
                tracing::error!("Failed to save agent to DB: {}", e);
            }
        }
    }

    Ok(Json(serde_json::json!({
        "status": "recorded",
        "task_id": task_id,
        "state": req.state
    })))
}

// --- Task Deletion ---

pub async fn delete_task(
    State(state): State<ApiState>,
    Path((agent_id_str, task_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let task_id = task_id.trim().to_string();
    if task_id.is_empty() {
        return Err(ApiError::BadRequest("task_id cannot be empty".into()));
    }

    let agent_id = {
        let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
        find_agent_by_id(&reg, &agent_id_str)?.id
    };

    // Remove from observer
    let found = {
        let mut obs = state.observer.write().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
        obs.delete_task(&agent_id, &task_id)
    };

    if !found {
        return Err(ApiError::NotFound(format!("task '{}' not found", task_id)));
    }

    // Remove from database
    if let Ok(db) = state.db.lock() {
        if let Err(e) = db.delete_task(agent_id, &task_id) {
            tracing::error!("Failed to delete task from DB: {}", e);
        }
    }

    persist_activity(&state.observer, &state.db, agent_id, ActivityKind::TaskFailed, &format!("Task {} deleted", task_id));

    Ok(Json(serde_json::json!({
        "status": "deleted",
        "task_id": task_id
    })))
}

// --- World ---

pub async fn world_snapshot(State(state): State<ApiState>) -> Result<Json<WorldSnapshot>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
    let tick = state.tick_count.load(std::sync::atomic::Ordering::Relaxed);

    let agents: Vec<ApiAgent> = reg
        .agents()
        .map(|a| ApiAgent {
            id: a.id.to_string(),
            name: a.name.clone(),
            state: a.state.label().to_string(),
            position: (a.position.x, a.position.y),
            task_count: a.task_count,
            speech: a.speech.clone(),
        })
        .collect();

    Ok(Json(WorldSnapshot {
        width: grid.width,
        height: grid.height,
        agents,
        tick,
    }))
}

pub async fn world_tiles(State(state): State<ApiState>) -> Result<Json<TileMapResponse>, ApiError> {
    let grid = state.grid.read().map_err(|_| ApiError::ServiceUnavailable("grid lock poisoned".into()))?;
    let mut tiles = Vec::with_capacity(grid.height as usize);
    for y in 0..grid.height {
        let mut row = Vec::with_capacity(grid.width as usize);
        for x in 0..grid.width {
            let pos = Position::new(x, y);
            let cell = grid.get(pos).ok_or(ApiError::BadRequest(format!("position ({},{}) out of bounds", x, y)))?;
            row.push(ApiCell {
                tile: format!("{:?}", cell.tile),
                occupant: cell.occupant.map(|id| id.to_string()),
            });
        }
        tiles.push(row);
    }
    Ok(Json(TileMapResponse {
        width: grid.width,
        height: grid.height,
        tiles,
    }))
}

// --- Agent Inbox ---

pub async fn get_agent_messages(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<InboxResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;

    let limit = params.limit.unwrap_or(50);
    let messages: Vec<InboxMessage> = agent
        .inbox
        .iter()
        .rev()
        .take(limit)
        .map(|m| {
            let from_name = reg
                .get(&m.from)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| m.from.to_string());
            InboxMessage {
                from: m.from.0.to_string(),
                from_name,
                text: m.text.clone(),
                timestamp: m.timestamp.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(InboxResponse {
        agent_id: agent_id.0.to_string(),
        count: messages.len(),
        messages,
    }))
}

pub async fn ack_agent_messages(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut reg = state.registry.write().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;

    let count = if let Some(agent) = reg.get_mut(&agent_id) {
        let c = agent.inbox.len();
        agent.inbox.clear();
        c
    } else {
        0
    };

    // Clear messages from database
    if let Ok(db) = state.db.lock() {
        if let Err(e) = db.clear_messages_for(&agent_id) {
                tracing::error!("Failed to clear messages from DB: {}", e);
            }
    }

    Ok(Json(serde_json::json!({
        "status": "cleared",
        "cleared": count,
    })))
}

// --- SSE Event Stream ---

pub async fn event_stream(
    State(state): State<ApiState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_broadcast.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(event) => {
            let json = serde_json::to_string(&event).ok()?;
            Some(Ok(Event::default().data(json)))
        }
        Err(_) => None,
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

// --- Observability Endpoints ---

pub async fn get_agent(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
) -> Result<Json<ApiAgentDetail>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;

    let obs = state.observer.read().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    let last_activity_secs_ago = obs
        .get_activity(&agent_id, 1)
        .first()
        .map(|e| (Utc::now() - e.timestamp).num_seconds().max(0) as u64)
        .unwrap_or(0);
    let connection_health = obs.connection_health(&agent_id).to_string();
    let goal = agent.goal.as_ref().map(|g| format!("{:?}", g));

    Ok(Json(ApiAgentDetail {
        id: agent.id.to_string(),
        name: agent.name.clone(),
        kind: format!("{:?}", agent.kind),
        state: agent.state.label().to_string(),
        position: (agent.position.x, agent.position.y),
        task_count: agent.task_count,
        speech: agent.speech.clone(),
        goal,
        last_activity_secs_ago,
        connection_health,
    }))
}

pub async fn get_agent_activity(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<ActivityResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;
    drop(reg);

    let limit = query.limit.unwrap_or(50);
    let obs = state.observer.read().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    let entries: Vec<_> = obs
        .get_activity(&agent_id, limit)
        .into_iter()
        .cloned()
        .collect();

    Ok(Json(ActivityResponse {
        agent_id: agent_id.to_string(),
        count: entries.len(),
        entries,
    }))
}

pub async fn post_agent_heartbeat(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Json(req): Json<HeartbeatRequest>,
) -> Result<Json<HeartbeatResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;
    drop(reg);

    let mut obs = state.observer.write().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    obs.update_heartbeat(agent_id, &req.status, req.metadata);
    obs.record_activity(
        agent_id,
        ActivityKind::Heartbeat,
        format!("Heartbeat: {}", req.status),
    );

    let hb = obs.get_heartbeat(&agent_id).ok_or(ApiError::NotFound("heartbeat not found after update".into()))?;
    let hb_clone = hb.clone();
    drop(obs);

    // Persist heartbeat to database
    if let Ok(db) = state.db.lock() {
        if let Err(e) = db.save_heartbeat(agent_id, &hb_clone) {
                tracing::error!("Failed to save heartbeat to DB: {}", e);
            }
    }

    Ok(Json(HeartbeatResponse {
        status: "ok".to_string(),
        last_seen: hb_clone.last_seen.to_rfc3339(),
    }))
}

pub async fn get_agent_status(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
) -> Result<Json<AgentStatusResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;
    let name = agent.name.clone();
    let agent_state = agent.state.label().to_string();
    drop(reg);

    let obs = state.observer.read().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    let connection_health = obs.connection_health(&agent_id).to_string();
    let heartbeat = obs.get_heartbeat(&agent_id).cloned();

    Ok(Json(AgentStatusResponse {
        agent_id: agent_id.to_string(),
        name,
        state: agent_state,
        connection_health,
        heartbeat,
    }))
}

pub async fn get_agent_tasks(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<TaskHistoryResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;
    drop(reg);

    let limit = query.limit.unwrap_or(50);
    let obs = state.observer.read().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    let tasks: Vec<_> = obs
        .get_tasks(&agent_id, limit)
        .into_iter()
        .cloned()
        .collect();

    Ok(Json(TaskHistoryResponse {
        agent_id: agent_id.to_string(),
        count: tasks.len(),
        tasks,
    }))
}

pub async fn get_agent_dashboard(
    State(state): State<ApiState>,
    Path(agent_id_str): Path<String>,
) -> Result<Json<DashboardResponse>, ApiError> {
    let reg = state.registry.read().map_err(|_| ApiError::ServiceUnavailable("registry lock poisoned".into()))?;
    let agent = find_agent_by_id(&reg, &agent_id_str)?;
    let agent_id = agent.id;
    let goal = agent.goal.as_ref().map(|g| format!("{:?}", g));
    let detail = ApiAgentDetail {
        id: agent.id.to_string(),
        name: agent.name.clone(),
        kind: format!("{:?}", agent.kind),
        state: agent.state.label().to_string(),
        position: (agent.position.x, agent.position.y),
        task_count: agent.task_count,
        speech: agent.speech.clone(),
        goal,
        last_activity_secs_ago: 0,        // filled below
        connection_health: String::new(), // filled below
    };
    drop(reg);

    let obs = state.observer.read().map_err(|_| ApiError::ServiceUnavailable("observer lock poisoned".into()))?;
    let recent_activity: Vec<_> = obs
        .get_activity(&agent_id, 20)
        .into_iter()
        .cloned()
        .collect();
    let task_history: Vec<_> = obs.get_tasks(&agent_id, 20).into_iter().cloned().collect();
    let heartbeat = obs.get_heartbeat(&agent_id).cloned();
    let connection_health = obs.connection_health(&agent_id).to_string();

    let last_activity_secs_ago = recent_activity
        .last()
        .map(|e| (Utc::now() - e.timestamp).num_seconds().max(0) as u64)
        .unwrap_or(0);

    Ok(Json(DashboardResponse {
        agent: ApiAgentDetail {
            last_activity_secs_ago,
            connection_health: connection_health.clone(),
            ..detail
        },
        recent_activity,
        task_history,
        heartbeat,
        connection_health,
    }))
}

// --- Auth Middleware ---

pub async fn auth_middleware(
    State(state): State<ApiState>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, ApiError> {
    let provided = req.headers().get("X-API-Key").and_then(|v| v.to_str().ok());
    if provided != Some(state.api_key.as_str()) {
        return Err(ApiError::Unauthorized);
    }
    Ok(next.run(req).await)
}

// --- Helpers ---

fn find_agent_by_id<'a>(
    reg: &'a AgentRegistry,
    id_str: &str,
) -> Result<&'a crate::agent::Agent, ApiError> {
    // Try full UUID first, then prefix match
    reg.agents()
        .find(|a| {
            let full = a.id.0.to_string();
            full == id_str || full.starts_with(id_str)
        })
        .ok_or_else(|| ApiError::NotFound(format!("agent '{}' not found", id_str)))
}

fn find_agent_by_id_mut<'a>(
    reg: &'a mut AgentRegistry,
    id_str: &str,
) -> Result<&'a mut crate::agent::Agent, ApiError> {
    // Find the ID first, then get mutable ref
    let target_id = {
        reg.agents()
            .find(|a| {
                let full = a.id.0.to_string();
                full == id_str || full.starts_with(id_str)
            })
            .map(|a| a.id)
            .ok_or_else(|| ApiError::NotFound(format!("agent '{}' not found", id_str)))?
    };
    reg.get_mut(&target_id)
        .ok_or_else(|| ApiError::NotFound(format!("agent '{}' not found", id_str)))
}

/// Record activity in both in-memory observer AND SQLite database.
fn persist_activity(
    observer: &Arc<RwLock<AgentObserver>>,
    db: &Arc<Mutex<crate::db::Database>>,
    agent_id: crate::agent::AgentId,
    kind: ActivityKind,
    detail: &str,
) {
    let entry = if let Ok(mut obs) = observer.write() {
        obs.record_activity(agent_id, kind.clone(), detail);
        crate::api::observability::ActivityEntry {
            timestamp: Utc::now(),
            kind,
            detail: detail.to_string(),
        }
    } else {
        tracing::error!("Failed to acquire observer write lock");
        return;
    };
    if let Ok(db) = db.lock() {
        if let Err(e) = db.save_activity(agent_id, &entry) {
            tracing::error!("Failed to save activity to DB: {}", e);
        }
    }
}

fn parse_agent_state(s: &str) -> Result<AgentState, ApiError> {
    match s {
        "idle" => Ok(AgentState::Idle),
        "walking" => Ok(AgentState::Walking),
        "thinking" => Ok(AgentState::Thinking),
        "working" => Ok(AgentState::Working),
        "messaging" => Ok(AgentState::Messaging),
        "eating" => Ok(AgentState::Eating),
        "exercising" => Ok(AgentState::Exercising),
        "playing" => Ok(AgentState::Playing),
        "error" => Ok(AgentState::Error),
        "offline" => Ok(AgentState::Offline),
        other => Err(ApiError::BadRequest(format!(
            "unknown state '{}'. Valid: idle, walking, thinking, working, messaging, eating, exercising, playing, error, offline",
            other
        ))),
    }
}
