use super::routes::{self, ApiState};
use crate::agent::AgentRegistry;
use crate::config::ServerConfig;
use crate::world::{Grid, WorldEvent};
use axum::Router;
use axum::routing::{get, post};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

pub fn build_router(
    registry: Arc<RwLock<AgentRegistry>>,
    grid: Arc<RwLock<Grid>>,
    event_tx: mpsc::Sender<WorldEvent>,
) -> Router {
    let state = ApiState {
        registry,
        grid,
        event_tx,
    };

    Router::new()
        .route("/health", get(routes::health))
        .route("/agents", get(routes::list_agents))
        .route("/agents/connect", post(routes::connect_agent))
        .route("/agents/{id}/message", post(routes::send_agent_message))
        .route("/world", get(routes::world_snapshot))
        .with_state(state)
}

pub async fn start_api_server(
    config: &ServerConfig,
    registry: Arc<RwLock<AgentRegistry>>,
    grid: Arc<RwLock<Grid>>,
    event_tx: mpsc::Sender<WorldEvent>,
) -> anyhow::Result<()> {
    let router = build_router(registry, grid, event_tx);
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("API server listening on {}", addr);
    axum::serve(listener, router).await?;
    Ok(())
}
