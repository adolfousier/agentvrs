use super::observability::AgentObserver;
use super::routes::{self, ApiState};
use crate::agent::AgentRegistry;
use crate::config::ServerConfig;
use crate::world::{Grid, WorldEvent};
use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, post};
use std::sync::{Arc, RwLock};
use tokio::sync::{broadcast, mpsc};
use tower::limit::ConcurrencyLimitLayer;

pub fn build_router(
    registry: Arc<RwLock<AgentRegistry>>,
    grid: Arc<RwLock<Grid>>,
    event_tx: mpsc::Sender<WorldEvent>,
    event_broadcast: broadcast::Sender<WorldEvent>,
    api_key: String,
    tick_count: Arc<std::sync::atomic::AtomicU64>,
    observer: Arc<RwLock<AgentObserver>>,
) -> Router {
    let state = ApiState {
        registry,
        grid,
        event_tx,
        event_broadcast,
        api_key,
        tick_count,
        observer,
    };

    // Health endpoint — no auth required
    let health_routes = Router::new()
        .route("/health", get(routes::health))
        .with_state(state.clone());

    // All other routes — auth + rate limit
    let api_routes = Router::new()
        .route("/agents", get(routes::list_agents))
        .route("/agents/connect", post(routes::connect_agent))
        .route("/agents/{id}", delete(routes::delete_agent))
        .route("/agents/{id}/message", post(routes::send_agent_message))
        .route("/agents/{id}/move", post(routes::move_agent))
        .route("/agents/{id}/goal", post(routes::set_agent_goal))
        .route("/agents/{id}/state", post(routes::set_agent_state))
        // Observability endpoints
        .route("/agents/{id}/detail", get(routes::get_agent))
        .route("/agents/{id}/activity", get(routes::get_agent_activity))
        .route("/agents/{id}/heartbeat", post(routes::post_agent_heartbeat))
        .route("/agents/{id}/status", get(routes::get_agent_status))
        .route("/agents/{id}/tasks", get(routes::get_agent_tasks))
        .route("/agents/{id}/dashboard", get(routes::get_agent_dashboard))
        // Agent inbox
        .route("/agents/{id}/messages", get(routes::get_agent_messages))
        .route("/agents/{id}/messages/ack", post(routes::ack_agent_messages))
        // World
        .route("/world", get(routes::world_snapshot))
        .route("/world/tiles", get(routes::world_tiles))
        .route("/events", get(routes::event_stream))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            routes::auth_middleware,
        ))
        .layer(ConcurrencyLimitLayer::new(100))
        .with_state(state);

    Router::new().merge(health_routes).merge(api_routes)
}

pub async fn start_api_server(
    config: &ServerConfig,
    registry: Arc<RwLock<AgentRegistry>>,
    grid: Arc<RwLock<Grid>>,
    event_tx: mpsc::Sender<WorldEvent>,
    event_broadcast: broadcast::Sender<WorldEvent>,
    tick_count: Arc<std::sync::atomic::AtomicU64>,
    observer: Arc<RwLock<AgentObserver>>,
) -> anyhow::Result<()> {
    let router = build_router(
        registry,
        grid,
        event_tx,
        event_broadcast,
        config.api_key.clone(),
        tick_count,
        observer,
    );
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("API server listening on {}", addr);
    axum::serve(listener, router).await?;
    Ok(())
}
