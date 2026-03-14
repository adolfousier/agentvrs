use super::types::*;
use crate::agent::{Agent, AgentKind, AgentRegistry, AgentState};
use crate::world::{Grid, WorldEvent};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ApiState {
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub grid: Arc<RwLock<Grid>>,
    pub event_tx: mpsc::Sender<WorldEvent>,
}

pub async fn health(State(state): State<ApiState>) -> Json<HealthResponse> {
    let reg = state.registry.read().unwrap();
    Json(HealthResponse {
        status: "ok".to_string(),
        version: crate::VERSION.to_string(),
        agents: reg.count(),
    })
}

pub async fn list_agents(State(state): State<ApiState>) -> Json<Vec<ApiAgent>> {
    let reg = state.registry.read().unwrap();
    let agents: Vec<ApiAgent> = reg
        .agents()
        .map(|a| ApiAgent {
            id: a.id.to_string(),
            name: a.name.clone(),
            state: a.state.label().to_string(),
            position: (a.position.x, a.position.y),
            task_count: a.task_count,
        })
        .collect();
    Json(agents)
}

pub async fn connect_agent(
    State(state): State<ApiState>,
    Json(req): Json<ConnectRequest>,
) -> Result<Json<ConnectResponse>, StatusCode> {
    let position = {
        let grid = state.grid.read().unwrap();
        grid.find_empty_floor()
            .ok_or(StatusCode::SERVICE_UNAVAILABLE)?
    };

    let kind = match req.endpoint {
        Some(ep) => AgentKind::External { endpoint: ep },
        None => AgentKind::Local,
    };

    let agent = Agent::new(&req.name, kind, position);
    let agent_id = agent.id;

    {
        let mut grid = state.grid.write().unwrap();
        grid.place_agent(position, agent_id);
    }

    {
        let mut reg = state.registry.write().unwrap();
        reg.register(agent);
    }

    let _ = state
        .event_tx
        .send(WorldEvent::AgentSpawned { agent_id, position })
        .await;

    Ok(Json(ConnectResponse {
        agent_id: agent_id.to_string(),
        position: (position.x, position.y),
    }))
}

pub async fn send_agent_message(
    State(state): State<ApiState>,
    Path(agent_id): Path<String>,
    Json(msg): Json<ApiMessage>,
) -> Result<StatusCode, StatusCode> {
    let mut reg = state.registry.write().unwrap();

    let target = reg
        .agents()
        .find(|a| a.id.to_string() == agent_id)
        .map(|a| a.id);

    let target_id = target.ok_or(StatusCode::NOT_FOUND)?;

    if let Some(agent) = reg.get_mut(&target_id) {
        agent.say(&msg.text);
        agent.set_state(AgentState::Messaging);
    }

    Ok(StatusCode::OK)
}

pub async fn world_snapshot(State(state): State<ApiState>) -> Json<WorldSnapshot> {
    let reg = state.registry.read().unwrap();
    let grid = state.grid.read().unwrap();

    let agents: Vec<ApiAgent> = reg
        .agents()
        .map(|a| ApiAgent {
            id: a.id.to_string(),
            name: a.name.clone(),
            state: a.state.label().to_string(),
            position: (a.position.x, a.position.y),
            task_count: a.task_count,
        })
        .collect();

    Json(WorldSnapshot {
        width: grid.width,
        height: grid.height,
        agents,
        tick: 0,
    })
}
