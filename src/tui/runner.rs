use crate::config::AppConfig;
use crate::runner;
use crate::tui::app::App;
use crate::tui::events::{EventHandler, TuiEvent};
use crate::tui::input::handle_key;
use crate::tui::render;
use anyhow::Result;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;

pub async fn run(config: AppConfig) -> Result<()> {
    // Use config dimensions for a compact, dense grid (default 10x8)
    let world_w = config.world.width;
    let world_h = config.world.height;

    let rt = runner::setup(&config, world_w, world_h).await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new(rt.grid, rt.registry, rt.message_log, rt.observer, rt.db, rt.event_rx);
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

    let _ = rt.shutdown_tx.send(()).await;
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
