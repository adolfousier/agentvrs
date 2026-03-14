use super::app::App;
use super::events::{EventHandler, TuiEvent};
use super::input::handle_key;
use super::render;
use crate::config::AppConfig;
use crate::runner;
use anyhow::Result;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;

const SIDEBAR_WIDTH: u16 = 28;
const STATUS_HEIGHT: u16 = 3;
const TILE_W: u16 = 4;
const TILE_H: u16 = 3;

pub async fn run(config: AppConfig) -> Result<()> {
    let (term_w, term_h) = crossterm::terminal::size()?;
    let world_cols = term_w.saturating_sub(SIDEBAR_WIDTH + 2);
    let world_rows = term_h.saturating_sub(STATUS_HEIGHT);
    let world_w = (world_cols / TILE_W).max(16);
    let world_h = (world_rows / TILE_H).max(8);

    let rt = runner::setup(&config, world_w, world_h).await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new(rt.grid, rt.registry, rt.message_log, rt.event_rx);
    let events = EventHandler::new(config.world.tick_ms);

    loop {
        app.process_events();
        terminal.draw(|frame| render::draw(frame, &app))?;

        match events.next()? {
            TuiEvent::Key(key) if key.kind == KeyEventKind::Press => handle_key(&mut app, key),
            TuiEvent::Resize(_, _) => {}
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
