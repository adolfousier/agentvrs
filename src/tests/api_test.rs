use crate::agent::AgentRegistry;
use crate::api::server::build_router;
use crate::api::types::*;
use crate::error::ErrorBody;
use crate::world::{Grid, Position, WorldEvent};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::{Arc, RwLock};
use tokio::sync::{broadcast, mpsc};
use tower::ServiceExt;

fn test_state() -> (axum::Router, Arc<RwLock<AgentRegistry>>, Arc<RwLock<Grid>>) {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(64);
    let tick_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let router = build_router(
        Arc::clone(&registry),
        Arc::clone(&grid),
        event_tx,
        broadcast_tx,
        None,
        tick_count,
    );
    (router, registry, grid)
}

fn test_state_with_auth(
    api_key: &str,
) -> (axum::Router, Arc<RwLock<AgentRegistry>>, Arc<RwLock<Grid>>) {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(64);
    let tick_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let router = build_router(
        Arc::clone(&registry),
        Arc::clone(&grid),
        event_tx,
        broadcast_tx,
        Some(api_key.to_string()),
        tick_count,
    );
    (router, registry, grid)
}

/// Helper: connect an agent and return its ID
async fn connect_helper(router: &axum::Router, name: &str) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/agents/connect")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ConnectRequest {
                name: name.to_string(),
                endpoint: None,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096)
        .await
        .unwrap();
    let connect: ConnectResponse = serde_json::from_slice(&body).unwrap();
    connect.agent_id
}

// ─── Health ─────────────────────────────────────────────────

#[tokio::test]
async fn test_health_endpoint() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let health: HealthResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(health.status, "ok");
    assert_eq!(health.agents, 0);
}

// ─── Agent CRUD ─────────────────────────────────────────────

#[tokio::test]
async fn test_list_agents_empty() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let agents: Vec<ApiAgent> = serde_json::from_slice(&body).unwrap();
    assert!(agents.is_empty());
}

#[tokio::test]
async fn test_connect_agent() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "test-bot").await;
    assert!(!agent_id.is_empty());
    assert_eq!(registry.read().unwrap().count(), 1);
}

#[tokio::test]
async fn test_connect_agent_with_endpoint() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .method("POST")
        .uri("/agents/connect")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ConnectRequest {
                name: "external-bot".to_string(),
                endpoint: Some("http://other:9090".to_string()),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_connect_multiple_agents() {
    let (router, registry, _) = test_state();
    connect_helper(&router, "bot-1").await;
    connect_helper(&router, "bot-2").await;
    connect_helper(&router, "bot-3").await;
    assert_eq!(registry.read().unwrap().count(), 3);
}

#[tokio::test]
async fn test_list_agents_after_connect() {
    let (router, _, _) = test_state();
    connect_helper(&router, "listed-bot").await;

    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096)
        .await
        .unwrap();
    let agents: Vec<ApiAgent> = serde_json::from_slice(&body).unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "listed-bot");
    assert_eq!(agents[0].state, "idle");
}

#[tokio::test]
async fn test_delete_agent() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "delete-me").await;
    assert_eq!(registry.read().unwrap().count(), 1);

    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/agents/{}", agent_id))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let del: DeleteResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(del.status, "removed");
    assert_eq!(registry.read().unwrap().count(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_agent() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .method("DELETE")
        .uri("/agents/nonexistent")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_clears_grid_occupant() {
    let (router, _, grid) = test_state();
    let agent_id = connect_helper(&router, "grid-check").await;

    // Find agent position
    let pos = {
        let g = grid.read().unwrap();
        let mut found = None;
        for y in 0..g.height {
            for x in 0..g.width {
                let p = Position::new(x, y);
                if g.get(p).unwrap().occupant.is_some() {
                    found = Some(p);
                    break;
                }
            }
            if found.is_some() {
                break;
            }
        }
        found.unwrap()
    };

    // Delete agent
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/agents/{}", agent_id))
        .body(Body::empty())
        .unwrap();
    router.oneshot(req).await.unwrap();

    // Grid cell should be clear
    let g = grid.read().unwrap();
    assert!(g.get(pos).unwrap().occupant.is_none());
}

// ─── Messaging ──────────────────────────────────────────────

#[tokio::test]
async fn test_message_to_nonexistent_agent() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .method("POST")
        .uri("/agents/nonexistent/message")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hello".to_string(),
                to: None,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_send_self_message() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "speaker").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hello world".to_string(),
                to: None,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let msg_resp: MessageResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(msg_resp.status, "delivered");
    assert!(msg_resp.delivered_to.is_none());

    // Agent should have speech and be in Messaging state
    let reg = registry.read().unwrap();
    let agent = reg.agents().next().unwrap();
    assert_eq!(agent.speech.as_deref(), Some("hello world"));
    assert_eq!(agent.state, crate::agent::AgentState::Messaging);
}

#[tokio::test]
async fn test_agent_to_agent_message() {
    let (router, registry, _) = test_state();
    let sender_id = connect_helper(&router, "sender").await;
    let receiver_id = connect_helper(&router, "receiver").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", sender_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hey there".to_string(),
                to: Some(receiver_id.clone()),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let msg_resp: MessageResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(msg_resp.status, "delivered");
    assert_eq!(msg_resp.delivered_to.as_deref(), Some(receiver_id.as_str()));

    // Receiver should have the speech, sender should not
    let reg = registry.read().unwrap();
    let receiver = reg
        .agents()
        .find(|a| a.name == "receiver")
        .unwrap();
    assert_eq!(receiver.speech.as_deref(), Some("hey there"));
    assert_eq!(receiver.state, crate::agent::AgentState::Messaging);
}

#[tokio::test]
async fn test_message_to_invalid_target() {
    let (router, _, _) = test_state();
    let sender_id = connect_helper(&router, "sender").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", sender_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hello".to_string(),
                to: Some("nonexistent".to_string()),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ─── Agent Actions: Move ────────────────────────────────────

#[tokio::test]
async fn test_move_agent() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "mover").await;

    // Get agent's current position, then pick an adjacent floor
    let target = {
        let reg = registry.read().unwrap();
        let agent = reg.agents().next().unwrap();
        let pos = agent.position;
        // Try to find a nearby walkable cell
        Position::new(pos.x + 1, pos.y)
    };

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/move", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&MoveRequest {
                x: target.x,
                y: target.y,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Agent should be walking
    let reg = registry.read().unwrap();
    let agent = reg.agents().next().unwrap();
    assert_eq!(agent.state, crate::agent::AgentState::Walking);
    assert!(!agent.path.is_empty());
}

#[tokio::test]
async fn test_move_to_wall() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "wall-mover").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/move", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&MoveRequest { x: 0, y: 0 }).unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_move_out_of_bounds() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "oob-mover").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/move", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&MoveRequest { x: 999, y: 999 }).unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ─── Agent Actions: Goal ────────────────────────────────────

#[tokio::test]
async fn test_set_goal_wander() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "wanderer").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/goal", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&GoalRequest {
                goal: "wander".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let reg = registry.read().unwrap();
    let agent = reg.agents().next().unwrap();
    assert_eq!(agent.state, crate::agent::AgentState::Walking);
}

#[tokio::test]
async fn test_set_goal_invalid() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "invalid-goal").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/goal", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&GoalRequest {
                goal: "swimming".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(resp.into_body(), 4096)
        .await
        .unwrap();
    let err: ErrorBody = serde_json::from_slice(&body).unwrap();
    assert_eq!(err.error, "bad_request");
    assert!(err.message.contains("swimming"));
}

// ─── Agent Actions: State ───────────────────────────────────

#[tokio::test]
async fn test_set_agent_state() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "state-setter").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/state", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&StateRequest {
                state: "working".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let reg = registry.read().unwrap();
    let agent = reg.agents().next().unwrap();
    assert_eq!(agent.state, crate::agent::AgentState::Working);
}

#[tokio::test]
async fn test_set_agent_state_idle_clears_path() {
    let (router, registry, _) = test_state();
    let agent_id = connect_helper(&router, "idle-clearer").await;

    // First set to walking with a wander goal
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/goal", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&GoalRequest {
                goal: "wander".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Now set to idle
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/state", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&StateRequest {
                state: "idle".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let reg = registry.read().unwrap();
    let agent = reg.agents().next().unwrap();
    assert_eq!(agent.state, crate::agent::AgentState::Idle);
    assert!(agent.path.is_empty());
    assert!(agent.goal.is_none());
}

#[tokio::test]
async fn test_set_agent_state_invalid() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "invalid-state").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/state", agent_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&StateRequest {
                state: "dancing".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_set_all_valid_states() {
    let states = [
        "idle", "walking", "thinking", "working", "messaging", "eating", "exercising", "playing",
        "error", "offline",
    ];
    for state_name in &states {
        let (router, _, _) = test_state();
        let agent_id = connect_helper(&router, "multi-state").await;

        let req = Request::builder()
            .method("POST")
            .uri(format!("/agents/{}/state", agent_id))
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&StateRequest {
                    state: state_name.to_string(),
                })
                .unwrap(),
            ))
            .unwrap();
        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "state '{}' should be valid",
            state_name
        );
    }
}

// ─── World ──────────────────────────────────────────────────

#[tokio::test]
async fn test_world_snapshot() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/world")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let snapshot: WorldSnapshot = serde_json::from_slice(&body).unwrap();
    assert_eq!(snapshot.width, 16);
    assert_eq!(snapshot.height, 12);
    assert!(snapshot.agents.is_empty());
}

#[tokio::test]
async fn test_world_snapshot_with_agents() {
    let (router, _, _) = test_state();
    connect_helper(&router, "snap-agent").await;

    let req = Request::builder()
        .uri("/world")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096)
        .await
        .unwrap();
    let snapshot: WorldSnapshot = serde_json::from_slice(&body).unwrap();
    assert_eq!(snapshot.agents.len(), 1);
    assert_eq!(snapshot.agents[0].name, "snap-agent");
}

#[tokio::test]
async fn test_world_tiles() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/world/tiles")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 65536)
        .await
        .unwrap();
    let tile_map: TileMapResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(tile_map.width, 16);
    assert_eq!(tile_map.height, 12);
    assert_eq!(tile_map.tiles.len(), 12); // 12 rows
    assert_eq!(tile_map.tiles[0].len(), 16); // 16 columns

    // Corners should be walls
    assert!(tile_map.tiles[0][0].tile.contains("Wall"));
    // Interior should be floor
    assert!(tile_map.tiles[5][5].tile.contains("Floor"));
}

// ─── Auth ───────────────────────────────────────────────────

#[tokio::test]
async fn test_api_key_auth_required() {
    let (router, _, _) = test_state_with_auth("test-secret-key");

    // No API key → 401
    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(resp.into_body(), 1024)
        .await
        .unwrap();
    let err: ErrorBody = serde_json::from_slice(&body).unwrap();
    assert_eq!(err.error, "unauthorized");
}

#[tokio::test]
async fn test_api_key_auth_wrong_key() {
    let (router, _, _) = test_state_with_auth("correct-key");

    let req = Request::builder()
        .uri("/agents")
        .header("X-API-Key", "wrong-key")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_api_key_auth_correct_key() {
    let (router, _, _) = test_state_with_auth("correct-key");

    let req = Request::builder()
        .uri("/agents")
        .header("X-API-Key", "correct-key")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_no_auth_needed() {
    let (router, _, _) = test_state_with_auth("secret");

    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_no_auth_when_key_not_configured() {
    let (router, _, _) = test_state(); // no api_key

    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ─── Error Response Format ──────────────────────────────────

#[tokio::test]
async fn test_error_response_json_format() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .method("DELETE")
        .uri("/agents/does-not-exist")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let body = axum::body::to_bytes(resp.into_body(), 4096)
        .await
        .unwrap();
    let err: ErrorBody = serde_json::from_slice(&body).unwrap();
    assert_eq!(err.error, "not_found");
    assert!(!err.message.is_empty());
}

// ─── Agent ID Prefix Matching ───────────────────────────────

#[tokio::test]
async fn test_agent_id_prefix_match() {
    let (router, _, _) = test_state();
    let full_id = connect_helper(&router, "prefix-test").await;

    // Use first 8 chars (short ID) — should work
    let short_id = &full_id[..8];
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/state", short_id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&StateRequest {
                state: "thinking".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ─── SSE Event Stream ───────────────────────────────────────

#[tokio::test]
async fn test_event_stream_endpoint_exists() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/events")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    // SSE endpoint should return 200 with text/event-stream content type
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(ct.contains("text/event-stream"));
}
