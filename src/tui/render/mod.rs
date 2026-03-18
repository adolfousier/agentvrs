mod details_panel;
mod message_log;
pub mod mission_control;
mod sidebar;
mod status_bar;
mod world_view;

use crate::tui::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn draw(frame: &mut Frame, app: &App) {
    // Mission Control is a full-screen overlay
    if app.mode == AppMode::MissionControl {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(8),    // MC panel
                Constraint::Length(3), // status bar
            ])
            .split(frame.area());

        mission_control::draw(frame, app, chunks[0]);
        status_bar::draw(frame, app, chunks[1]);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),    // world view
            Constraint::Length(28), // sidebar
        ])
        .split(frame.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),    // world or detail
            Constraint::Length(3), // status bar
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
