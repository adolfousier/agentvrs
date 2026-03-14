use crate::agent::{Agent, AgentKind, AgentRegistry, MessageLog};
use crate::api;
use crate::api::observability::AgentObserver;
use crate::config::AppConfig;
use crate::world::{Grid, Simulation, WorldEvent, build_office_world};
use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::{broadcast, mpsc};

pub struct WorldRuntime {
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub message_log: Arc<RwLock<MessageLog>>,
    pub event_rx: mpsc::Receiver<WorldEvent>,
    pub shutdown_tx: mpsc::Sender<()>,
}

pub async fn setup(config: &AppConfig, world_w: u16, world_h: u16) -> Result<WorldRuntime> {
    let grid = Arc::new(RwLock::new(build_office_world(world_w, world_h)));
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let message_log = Arc::new(RwLock::new(MessageLog::new()));
    let (event_tx, event_rx) = mpsc::channel::<WorldEvent>(256);
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

    // Broadcast channel for SSE subscribers
    let (broadcast_tx, _) = broadcast::channel::<WorldEvent>(256);

    // Spawn demo agents on empty floors
    {
        let mut g = grid.write().unwrap();
        let mut r = registry.write().unwrap();
        let names = ["crab-alpha", "crab-beta", "crab-gamma", "crab-delta"];
        for name in &names {
            if let Some(pos) = g.find_empty_floor() {
                let agent = Agent::new(*name, AgentKind::Local, pos);
                let id = agent.id;
                g.place_agent(pos, id);
                r.register(agent);
            }
        }
    }

    // Simulation (with broadcast for SSE)
    let sim = Simulation::new(
        Arc::clone(&grid),
        Arc::clone(&registry),
        event_tx.clone(),
        config.world.tick_ms,
    )
    .with_broadcast(broadcast_tx.clone());
    let tick_count = Arc::clone(&sim.shared_tick);
    tokio::spawn(sim.run(shutdown_rx));

    // Agent observer (activity logs, heartbeats, task history)
    let observer = Arc::new(RwLock::new(AgentObserver::new(500, 200)));

    // API server
    if config.server.enabled {
        let sg = Arc::clone(&grid);
        let sr = Arc::clone(&registry);
        let stx = event_tx.clone();
        let sbtx = broadcast_tx;
        let sc = config.server.clone();
        let st = Arc::clone(&tick_count);
        let so = Arc::clone(&observer);
        tokio::spawn(async move {
            if let Err(e) = api::start_api_server(&sc, sr, sg, stx, sbtx, st, so).await {
                tracing::error!("API server error: {}", e);
            }
        });
    }

    Ok(WorldRuntime {
        grid,
        registry,
        message_log,
        event_rx,
        shutdown_tx,
    })
}
