use crate::agent::AgentRegistry;
use crate::api::observability::AgentObserver;
use crate::api::server::build_router;
use crate::api::types::*;
use crate::world::{Grid, WorldEvent};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::{Arc, RwLock};
use tokio::sync::{broadcast, mpsc};
use tower::ServiceExt;

const TEST_KEY: &str = "test-key";

fn test_state() -> axum::Router {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(64);
    let tick_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let observer = Arc::new(RwLock::new(AgentObserver::new(500, 200)));
    build_router(
        registry,
        grid,
        event_tx,
        broadcast_tx,
        TEST_KEY.to_string(),
        tick_count,
        observer,
    )
}

async fn connect(router: &axum::Router, name: &str) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/agents/connect")
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(format!(r#"{{"name":"{}"}}"#, name)))
        .unwrap();
    let resp = router.clone().oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let cr: ConnectResponse = serde_json::from_slice(&body).unwrap();
    cr.agent_id
}

async fn set_state(router: &axum::Router, id: &str, state: &str) {
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/state", id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(format!(r#"{{"state":"{}"}}"#, state)))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();
}

// ─── Detail ─────────────────────────────────────────────────

#[tokio::test]
async fn test_agent_detail() {
    let router = test_state();
    let id = connect(&router, "detail-test").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/detail", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let d: ApiAgentDetail = serde_json::from_slice(&body).unwrap();
    assert_eq!(d.name, "detail-test");
    assert_eq!(d.state, "idle");
}

#[tokio::test]
async fn test_agent_detail_not_found() {
    let router = test_state();
    let req = Request::builder()
        .uri("/agents/nonexistent/detail")
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ─── Activity ───────────────────────────────────────────────

#[tokio::test]
async fn test_activity_from_connect() {
    let router = test_state();
    let id = connect(&router, "act-test").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/activity", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let a: ActivityResponse = serde_json::from_slice(&body).unwrap();
    assert!(a.count >= 1);
    assert!(a.entries[0].detail.contains("connected"));
}

#[tokio::test]
async fn test_activity_limit() {
    let router = test_state();
    let id = connect(&router, "limit-test").await;
    set_state(&router, &id, "working").await;
    set_state(&router, &id, "thinking").await;
    set_state(&router, &id, "idle").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/activity?limit=2", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let a: ActivityResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(a.count, 2);
}

#[tokio::test]
async fn test_state_change_recorded() {
    let router = test_state();
    let id = connect(&router, "sc-test").await;
    set_state(&router, &id, "thinking").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/activity", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let a: ActivityResponse = serde_json::from_slice(&body).unwrap();
    assert!(a.entries.iter().any(|e| e.detail.contains("thinking")));
}

#[tokio::test]
async fn test_message_recorded() {
    let router = test_state();
    let id = connect(&router, "msg-test").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/message", id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(r#"{"text":"hello world"}"#))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    let req = Request::builder()
        .uri(format!("/agents/{}/activity", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let a: ActivityResponse = serde_json::from_slice(&body).unwrap();
    assert!(a.entries.iter().any(|e| e.detail.contains("hello world")));
}

// ─── Heartbeat ──────────────────────────────────────────────

#[tokio::test]
async fn test_heartbeat() {
    let router = test_state();
    let id = connect(&router, "hb-test").await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/heartbeat", id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(
            r#"{"status":"healthy","metadata":{"cpu":0.42}}"#,
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let hb: HeartbeatResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(hb.status, "ok");
    assert!(!hb.last_seen.is_empty());
}

// ─── Status ─────────────────────────────────────────────────

#[tokio::test]
async fn test_status_with_heartbeat() {
    let router = test_state();
    let id = connect(&router, "status-test").await;

    // Send heartbeat first
    let req = Request::builder()
        .method("POST")
        .uri(format!("/agents/{}/heartbeat", id))
        .header("content-type", "application/json")
        .header("X-API-Key", TEST_KEY)
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap();
    router.clone().oneshot(req).await.unwrap();

    let req = Request::builder()
        .uri(format!("/agents/{}/status", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let s: AgentStatusResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(s.connection_health, "online");
    assert!(s.heartbeat.is_some());
}

#[tokio::test]
async fn test_status_unknown_without_heartbeat() {
    let router = test_state();
    let id = connect(&router, "no-hb").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/status", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let s: AgentStatusResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(s.connection_health, "unknown");
    assert!(s.heartbeat.is_none());
}

// ─── Tasks ──────────────────────────────────────────────────

#[tokio::test]
async fn test_tasks_empty() {
    let router = test_state();
    let id = connect(&router, "task-test").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/tasks", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    let t: TaskHistoryResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(t.count, 0);
}

// ─── Dashboard ──────────────────────────────────────────────

#[tokio::test]
async fn test_dashboard() {
    let router = test_state();
    let id = connect(&router, "dash-test").await;
    set_state(&router, &id, "working").await;

    let req = Request::builder()
        .uri(format!("/agents/{}/dashboard", id))
        .header("X-API-Key", TEST_KEY)
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 8192).await.unwrap();
    let d: DashboardResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(d.agent.name, "dash-test");
    assert_eq!(d.agent.state, "working");
    assert!(!d.recent_activity.is_empty());
    assert_eq!(d.connection_health, "unknown");
}
