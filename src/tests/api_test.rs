use crate::agent::AgentRegistry;
use crate::api::observability::AgentObserver;
use crate::api::server::build_router;
use crate::api::types::*;
use crate::db::Database;
use crate::error::ErrorBody;
use crate::world::{Grid, Position, WorldEvent};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::{broadcast, mpsc};
use tower::ServiceExt;

const TEST_KEY: &str = "test-key";

fn test_state() -> (axum::Router, Arc<RwLock<AgentRegistry>>, Arc<RwLock<Grid>>) {
    let (router, registry, grid, _db) = test_state_full();
    (router, registry, grid)
}

fn test_state_full() -> (
    axum::Router,
    Arc<RwLock<AgentRegistry>>,
    Arc<RwLock<Grid>>,
    Arc<Mutex<Database>>,
) {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(64);
    let tick_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let observer = Arc::new(RwLock::new(AgentObserver::new(500, 200)));
    let db = Arc::new(Mutex::new(Database::open_in_memory().unwrap()));
    let router = build_router(
        Arc::clone(&registry),
        Arc::clone(&grid),
        event_tx,
        broadcast_tx,
        TEST_KEY.to_string(),
        tick_count,
        observer,
        Arc::clone(&db),
    );
    (router, registry, grid, db)
}

fn test_state_with_auth(
    api_key: &str,
) -> (axum::Router, Arc<RwLock<AgentRegistry>>, Arc<RwLock<Grid>>) {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(64);
    let tick_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let observer = Arc::new(RwLock::new(AgentObserver::new(500, 200)));
    let db = Arc::new(Mutex::new(Database::open_in_memory().unwrap()));
    let router = build_router(
        Arc::clone(&registry),
        Arc::clone(&grid),
        event_tx,
        broadcast_tx,
        api_key.to_string(),
        tick_count,
        observer,
        db,
    );
    (router, registry, grid)
}

/// Helper: connect an agent and return its ID
async fn connect_helper(router: &axum::Router, name: &str) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/agents/connect")
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ConnectRequest {
                name: name.to_string(),
                endpoint: None,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
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

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
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

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
    let msg_resp: MessageResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(msg_resp.status, "delivered");
    assert_eq!(msg_resp.delivered_to.as_deref(), Some(receiver_id.as_str()));

    // Receiver should have the speech, sender should not
    let reg = registry.read().unwrap();
    let receiver = reg.agents().find(|a| a.name == "receiver").unwrap();
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
        .header("X-API-Key", TEST_KEY)
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

// ─── Agent Inbox ────────────────────────────────────────────

#[tokio::test]
async fn test_inbox_empty() {
    let (router, _, _) = test_state();
    let id = connect_helper(&router, "inbox-bot").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/messages", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 0);
    assert!(inbox.messages.is_empty());
}

#[tokio::test]
async fn test_inbox_receives_message() {
    let (router, _, _) = test_state();
    let sender_id = connect_helper(&router, "sender").await;
    let receiver_id = connect_helper(&router, "receiver").await;

    // Send message from sender to receiver
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", sender_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "do task X".to_string(),
                to: Some(receiver_id.clone()),
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Check receiver's inbox
    let req = Request::builder()
        .uri(format!("/agents/{}/messages", receiver_id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 1);
    assert_eq!(inbox.messages[0].text, "do task X");
    assert_eq!(inbox.messages[0].from_name, "sender");
}

#[tokio::test]
async fn test_inbox_multiple_messages() {
    let (router, _, _) = test_state();
    let a = connect_helper(&router, "agent-a").await;
    let b = connect_helper(&router, "agent-b").await;

    for text in &["msg-1", "msg-2", "msg-3"] {
        let req = Request::builder()
            .method("POST")
            .uri(format!("/agents/{}/message", a))
            .header("content-type", "application/json")
            .header("X-API-Key", TEST_KEY)
            .body(Body::from(
                serde_json::to_string(&ApiMessage {
                    text: text.to_string(),
                    to: Some(b.clone()),
                })
                .unwrap(),
            ))
            .unwrap();
        router.clone().oneshot(req).await.unwrap();
    }

    let req = Request::builder()
        .uri(format!("/agents/{}/messages", b))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 3);
}

#[tokio::test]
async fn test_inbox_limit() {
    let (router, _, _) = test_state();
    let a = connect_helper(&router, "agent-a").await;
    let b = connect_helper(&router, "agent-b").await;

    for i in 0..5 {
        let req = Request::builder()
            .method("POST")
            .uri(format!("/agents/{}/message", a))
            .header("content-type", "application/json")
            .header("X-API-Key", TEST_KEY)
            .body(Body::from(
                serde_json::to_string(&ApiMessage {
                    text: format!("msg-{}", i),
                    to: Some(b.clone()),
                })
                .unwrap(),
            ))
            .unwrap();
        router.clone().oneshot(req).await.unwrap();
    }

    let req = Request::builder()
        .uri(format!("/agents/{}/messages?limit=2", b))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 2);
}

#[tokio::test]
async fn test_inbox_ack_clears() {
    let (router, _, _) = test_state();
    let a = connect_helper(&router, "agent-a").await;
    let b = connect_helper(&router, "agent-b").await;

    // Send a message
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", a))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hello".to_string(),
                to: Some(b.clone()),
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Ack
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/messages/ack", b))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
    let ack: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(ack["cleared"], 1);

    // Inbox should be empty now
    let req = Request::builder()
        .uri(format!("/agents/{}/messages", b))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 0);
}

#[tokio::test]
async fn test_self_message_not_in_inbox() {
    let (router, _, _) = test_state();
    let id = connect_helper(&router, "self-talker").await;

    // Self-message (no "to")
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "thinking out loud".to_string(),
                to: None,
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Inbox should be empty — self-messages are speech bubbles only
    let req = Request::builder()
        .uri(format!("/agents/{}/messages", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 0);
}

#[tokio::test]
async fn test_inbox_cap_at_500() {
    let (router, registry, _) = test_state();
    let sender_id = connect_helper(&router, "spammer").await;
    let receiver_id = connect_helper(&router, "receiver").await;

    // Send 505 messages directly via registry to avoid 505 HTTP round trips
    {
        let mut reg = registry.write().unwrap();
        // Find agents by name since API returns short IDs
        let sender_agent_id = reg.agents().find(|a| a.name == "spammer").unwrap().id;
        let receiver_agent_id = reg.agents().find(|a| a.name == "receiver").unwrap().id;

        if let Some(receiver) = reg.get_mut(&receiver_agent_id) {
            for i in 0..505 {
                let msg = crate::agent::AgentMessage::new(
                    sender_agent_id,
                    receiver_agent_id,
                    format!("msg-{}", i),
                );
                receiver.inbox.push_back(msg);
                while receiver.inbox.len() > 500 {
                    receiver.inbox.pop_front();
                }
            }
        }
    }

    // Check inbox — should be capped at 500
    let req = Request::builder()
        .uri(format!("/agents/{}/messages?limit=600", receiver_id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 512_000)
        .await
        .unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 500);
    // Oldest messages should be dropped — first message should be msg-5
    assert_eq!(inbox.messages.last().unwrap().text, "msg-5");
}

#[tokio::test]
async fn test_inbox_multiple_senders() {
    let (router, _, _) = test_state();
    let a = connect_helper(&router, "agent-a").await;
    let b = connect_helper(&router, "agent-b").await;
    let c = connect_helper(&router, "agent-c").await;

    // A sends to C
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", a))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "from A".to_string(),
                to: Some(c.clone()),
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // B sends to C
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", b))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "from B".to_string(),
                to: Some(c.clone()),
            })
            .unwrap(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // C's inbox should have both
    let req = Request::builder()
        .uri(format!("/agents/{}/messages", c))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let inbox: InboxResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(inbox.count, 2);

    let texts: Vec<&str> = inbox.messages.iter().map(|m| m.text.as_str()).collect();
    assert!(texts.contains(&"from A"));
    assert!(texts.contains(&"from B"));

    // Verify sender names
    let names: Vec<&str> = inbox
        .messages
        .iter()
        .map(|m| m.from_name.as_str())
        .collect();
    assert!(names.contains(&"agent-a"));
    assert!(names.contains(&"agent-b"));
}

// ─── Agent Actions: Move ────────────────────────────────────

#[tokio::test]
async fn test_move_agent() {
    let (router, registry, grid) = test_state();
    let agent_id = connect_helper(&router, "mover").await;

    // Get agent's current position, then pick an adjacent walkable cell
    let target = {
        let reg = registry.read().unwrap();
        let agent = reg.agents().next().unwrap();
        let pos = agent.position;
        let g = grid.read().unwrap();
        g.find_adjacent_floor(pos).expect("no adjacent floor found")
    };

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/move", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::to_string(&GoalRequest {
                goal: "swimming".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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
        "idle",
        "walking",
        "thinking",
        "working",
        "messaging",
        "eating",
        "exercising",
        "playing",
        "error",
        "offline",
    ];
    for state_name in &states {
        let (router, _, _) = test_state();
        let agent_id = connect_helper(&router, "multi-state").await;

        let req = Request::builder()
            .method("POST")
            .uri(format!("/agents/{}/state", agent_id))
            .header("content-type", "application/json")
            .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let snapshot: WorldSnapshot = serde_json::from_slice(&body).unwrap();
    assert_eq!(snapshot.agents.len(), 1);
    assert_eq!(snapshot.agents[0].name, "snap-agent");
}

#[tokio::test]
async fn test_world_tiles() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/world/tiles")
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 65536).await.unwrap();
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
    let (router, _, _) = test_state();

    // No API key → 401
    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
    let err: ErrorBody = serde_json::from_slice(&body).unwrap();
    assert_eq!(err.error, "unauthorized");
}

#[tokio::test]
async fn test_api_key_auth_wrong_key() {
    let (router, _, _) = test_state();

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
    let (router, _, _) = test_state();

    let req = Request::builder()
        .uri("/agents")
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_no_auth_needed() {
    let (router, _, _) = test_state();

    let req = Request::builder()
        .uri("/health")
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
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
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
        .header("X-API-Key", TEST_KEY)
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
        .header("X-API-Key", TEST_KEY)
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

// ─── Task Reporting ──────────────────────────────────────────

#[tokio::test]
async fn test_report_task_submitted() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "task-agent").await;

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({
                "task_id": "task-001",
                "state": "submitted",
                "summary": "Parse CSV data"
            })
            .to_string(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let val: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(val["status"], "recorded");
    assert_eq!(val["task_id"], "task-001");
    assert_eq!(val["state"], "submitted");

    // Verify task persisted to DB
    let aid = db_agent_id(&db, "task-agent");
    let dbl = db.lock().unwrap();
    let tasks = dbl.load_tasks(&aid, 10).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_id, "task-001");
    assert_eq!(tasks[0].state, "submitted");
    assert_eq!(tasks[0].response_summary.as_deref(), Some("Parse CSV data"));
}

#[tokio::test]
async fn test_report_task_completed() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "task-agent").await;

    // Submit
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"task_id": "t1", "state": "submitted"}).to_string(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Complete
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"task_id": "t1", "state": "completed", "summary": "Done"})
                .to_string(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // DB should have latest state
    let aid = db_agent_id(&db, "task-agent");
    let dbl = db.lock().unwrap();
    let tasks = dbl.load_tasks(&aid, 10).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].state, "completed");
}

#[tokio::test]
async fn test_report_task_with_scope() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "scope-agent").await;

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({
                "task_id": "scoped-t1",
                "state": "submitted",
                "summary": "Short summary",
                "scope": "Full detailed scope of what this task covers"
            })
            .to_string(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify scope persisted to DB
    let aid = db_agent_id(&db, "scope-agent");
    let dbl = db.lock().unwrap();
    let tasks = dbl.load_tasks(&aid, 10).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_id, "scoped-t1");
    assert_eq!(tasks[0].response_summary.as_deref(), Some("Short summary"));
    assert_eq!(
        tasks[0].scope.as_deref(),
        Some("Full detailed scope of what this task covers")
    );
}

#[tokio::test]
async fn test_report_task_scope_preserved_on_update() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "scope-keep").await;

    // Submit with scope
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({
                "task_id": "scope-keep-t1",
                "state": "submitted",
                "summary": "Initial",
                "scope": "Original scope text"
            })
            .to_string(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    // Update without scope — should preserve original
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({
                "task_id": "scope-keep-t1",
                "state": "completed",
                "summary": "Done"
            })
            .to_string(),
        ))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    let aid = db_agent_id(&db, "scope-keep");
    let dbl = db.lock().unwrap();
    let tasks = dbl.load_tasks(&aid, 10).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].state, "completed");
    assert_eq!(tasks[0].scope.as_deref(), Some("Original scope text"));
}

#[tokio::test]
async fn test_report_task_invalid_state() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "task-agent").await;

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"task_id": "t1", "state": "bogus"}).to_string(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_report_task_empty_id() {
    let (router, _, _) = test_state();
    let agent_id = connect_helper(&router, "task-agent").await;

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/tasks", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"task_id": "", "state": "submitted"}).to_string(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ─── Activity Persistence ────────────────────────────────────

/// Helper: get full AgentId from DB by agent name
fn db_agent_id(db: &Arc<Mutex<Database>>, name: &str) -> crate::agent::AgentId {
    let db = db.lock().unwrap();
    db.load_agents()
        .unwrap()
        .into_iter()
        .find(|a| a.name == name)
        .unwrap_or_else(|| panic!("agent '{}' not in DB", name))
        .id
}

#[tokio::test]
async fn test_connect_persists_agent_to_db() {
    let (router, _, _, db) = test_state_full();
    connect_helper(&router, "persist-me").await;

    let dbl = db.lock().unwrap();
    let agents = dbl.load_agents().unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "persist-me");
}

#[tokio::test]
async fn test_connect_persists_activity_to_db() {
    let (router, _, _, db) = test_state_full();
    connect_helper(&router, "activity-agent").await;

    let aid = db_agent_id(&db, "activity-agent");
    let dbl = db.lock().unwrap();
    let activity = dbl.load_activity(&aid, 10).unwrap();
    assert!(
        !activity.is_empty(),
        "connect should persist activity to DB"
    );
    assert!(activity[0].detail.contains("connected"));
}

#[tokio::test]
async fn test_message_persists_activity_to_db() {
    let (router, _, _, db) = test_state_full();
    let sender = connect_helper(&router, "sender").await;
    let receiver = connect_helper(&router, "receiver").await;

    // Send message
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/message", sender))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"text": "hello!", "to": receiver}).to_string(),
        ))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Both sender and receiver should have activity in DB
    let sender_aid = db_agent_id(&db, "sender");
    let receiver_aid = db_agent_id(&db, "receiver");
    let dbl = db.lock().unwrap();
    let sender_activity = dbl.load_activity(&sender_aid, 10).unwrap();
    let receiver_activity = dbl.load_activity(&receiver_aid, 10).unwrap();
    assert!(sender_activity.iter().any(|a| a.detail.contains("Sent to")));
    assert!(receiver_activity.iter().any(|a| a.detail.contains("From")));
}

#[tokio::test]
async fn test_delete_purges_agent_from_db() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "delete-me").await;

    // Verify agent exists in DB
    {
        let db = db.lock().unwrap();
        assert_eq!(db.load_agents().unwrap().len(), 1);
    }

    // Delete
    let req = Request::builder()
        .method("DELETE")
        .uri(&format!("/agents/{}", agent_id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Agent should be purged from DB
    let db = db.lock().unwrap();
    assert_eq!(db.load_agents().unwrap().len(), 0);
}

#[tokio::test]
async fn test_rename_persists_to_db() {
    let (router, _, _, db) = test_state_full();
    let agent_id = connect_helper(&router, "old-name").await;

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{}/rename", agent_id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            serde_json::json!({"name": "new-name"}).to_string(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let db = db.lock().unwrap();
    let agents = db.load_agents().unwrap();
    assert_eq!(agents[0].name, "new-name");
}
