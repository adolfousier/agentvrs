mod details_panel;
mod message_log;
mod sidebar;
mod status_bar;
mod world_view;

use super::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),    // world view
            Constraint::Length(24), // sidebar
        ])
        .split(frame.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),    // world or detail
            Constraint::Length(2), // status bar
        ])
        .split(chunks[0]);

    match app.mode {
        AppMode::AgentDetail => {
            details_panel::draw(frame, app, left_chunks[0]);
        }
        AppMode::MessageLog => {
            message_log::draw(frame, app, left_chunks[0]);
        }
        _ => {
            world_view::draw(frame, app, left_chunks[0]);
        }
    }

    status_bar::draw(frame, app, left_chunks[1]);
    sidebar::draw(frame, app, chunks[1]);
}
