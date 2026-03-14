use super::app::App;
use super::events::{EventHandler, TuiEvent};
use super::input::handle_key;
use super::render;
use crate::agent::{Agent, AgentKind, AgentRegistry, MessageLog};
use crate::api;
use crate::config::AppConfig;
use crate::world::{Simulation, WorldEvent, build_office_world};
use anyhow::Result;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

pub async fn run(config: AppConfig) -> Result<()> {
    let grid = Arc::new(RwLock::new(build_office_world()));
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let message_log = Arc::new(RwLock::new(MessageLog::new()));
    let (event_tx, event_rx) = mpsc::channel::<WorldEvent>(256);
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

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

    // Simulation
    let sim = Simulation::new(
        Arc::clone(&grid),
        Arc::clone(&registry),
        event_tx.clone(),
        config.world.tick_ms,
    );
    tokio::spawn(sim.run(shutdown_rx));

    // API server
    if config.server.enabled {
        let sg = Arc::clone(&grid);
        let sr = Arc::clone(&registry);
        let stx = event_tx.clone();
        let sc = config.server.clone();
        tokio::spawn(async move {
            if let Err(e) = api::start_api_server(&sc, sr, sg, stx).await {
                tracing::error!("API server error: {}", e);
            }
        });
    }

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new(grid, registry, message_log, event_rx);
    let events = EventHandler::new(config.world.tick_ms);

    loop {
        app.process_events();
        terminal.draw(|frame| render::draw(frame, &app))?;

        match events.next()? {
            TuiEvent::Key(key) if key.kind == KeyEventKind::Press => handle_key(&mut app, key),
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }

    let _ = shutdown_tx.send(()).await;
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
