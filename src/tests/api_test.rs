use crate::agent::AgentRegistry;
use crate::api::server::build_router;
use crate::api::types::*;
use crate::world::{Grid, WorldEvent};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tower::ServiceExt;

fn test_state() -> (axum::Router, Arc<RwLock<AgentRegistry>>, Arc<RwLock<Grid>>) {
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let grid = Arc::new(RwLock::new(Grid::with_walls(16, 12)));
    let (event_tx, _rx) = mpsc::channel::<WorldEvent>(64);
    let router = build_router(Arc::clone(&registry), Arc::clone(&grid), event_tx);
    (router, registry, grid)
}

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

#[tokio::test]
async fn test_list_agents_empty() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/agents")
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
    let req = Request::builder()
        .method("POST")
        .uri("/agents/connect")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ConnectRequest {
                name: "test-bot".to_string(),
                endpoint: None,
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
    let connect: ConnectResponse = serde_json::from_slice(&body).unwrap();
    assert!(!connect.agent_id.is_empty());

    let reg = registry.read().unwrap();
    assert_eq!(reg.count(), 1);
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
async fn test_world_snapshot() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .uri("/world")
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
async fn test_message_to_nonexistent_agent() {
    let (router, _, _) = test_state();
    let req = Request::builder()
        .method("POST")
        .uri("/agents/nonexistent/message")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ApiMessage {
                text: "hello".to_string(),
            })
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
