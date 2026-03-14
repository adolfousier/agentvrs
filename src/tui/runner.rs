use super::app::App;
use super::events::{EventHandler, TuiEvent};
use super::input::handle_key;
use super::render;
use crate::agent::{Agent, AgentKind, AgentRegistry, AgentState, MessageLog};
use crate::api;
use crate::config::AppConfig;
use crate::world::{Grid, Simulation, WorldEvent};
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
    // Get terminal size for responsive grid
    let (term_w, term_h) = crossterm::terminal::size()?;
    // Grid fills the world panel: subtract sidebar (28 cols) and borders (4),
    // each cell is 2 chars wide. Height subtracts status bar (3) and borders (2).
    let grid_w = config
        .world
        .width
        .max(((term_w.saturating_sub(32)) / 2).max(16));
    let grid_h = config.world.height.max((term_h.saturating_sub(5)).max(8));

    let grid = Arc::new(RwLock::new(Grid::with_walls(grid_w, grid_h)));
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let message_log = Arc::new(RwLock::new(MessageLog::new()));
    let (event_tx, event_rx) = mpsc::channel::<WorldEvent>(256);
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

    // Spawn demo agents
    {
        let mut g = grid.write().unwrap();
        let mut r = registry.write().unwrap();

        let demo_agents = [
            ("crab-alpha", AgentState::Working),
            ("crab-beta", AgentState::Thinking),
        ];

        for (name, state) in &demo_agents {
            if let Some(pos) = g.find_empty_floor() {
                let mut agent = Agent::new(*name, AgentKind::Local, pos);
                agent.set_state(state.clone());
                if *name == "crab-alpha" {
                    agent.say("building agentverse...");
                }
                let id = agent.id;
                g.place_agent(pos, id);
                r.register(agent);
            }
        }
    }

    // Start simulation
    let sim = Simulation::new(
        Arc::clone(&grid),
        Arc::clone(&registry),
        event_tx.clone(),
        config.world.tick_ms,
    );
    tokio::spawn(sim.run(shutdown_rx));

    // Start API server if enabled
    if config.server.enabled {
        let api_grid = Arc::clone(&grid);
        let api_registry = Arc::clone(&registry);
        let api_tx = event_tx.clone();
        let server_config = config.server.clone();
        tokio::spawn(async move {
            if let Err(e) =
                api::start_api_server(&server_config, api_registry, api_grid, api_tx).await
            {
                tracing::error!("API server error: {}", e);
            }
        });
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(grid, registry, message_log, event_rx);
    let event_handler = EventHandler::new(config.world.tick_ms);

    // Main loop
    loop {
        app.process_events();

        terminal.draw(|frame| {
            render::draw(frame, &app);
        })?;

        match event_handler.next()? {
            TuiEvent::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut app, key);
                }
            }
            TuiEvent::Resize(_, _) => {}
            TuiEvent::Tick => {}
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    let _ = shutdown_tx.send(()).await;
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
