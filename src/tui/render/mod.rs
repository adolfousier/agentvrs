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
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(frame.area());

        mission_control::draw(frame, app, chunks[0]);
        status_bar::draw(frame, app, chunks[1]);
        return;
    }

    // Detail and MessageLog views are full-screen (with sidebar)
    if matches!(app.mode, AppMode::AgentDetail | AppMode::MessageLog) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(20),
                Constraint::Length(30),
            ])
            .split(frame.area());

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(cols[0]);

        match app.mode {
            AppMode::AgentDetail => details_panel::draw(frame, app, left[0]),
            AppMode::MessageLog => message_log::draw(frame, app, left[0]),
            _ => {}
        }
        status_bar::draw(frame, app, left[1]);
        sidebar::draw(frame, app, cols[1]);
        return;
    }

    // World view: world on left, sidebar + detail stacked on right
    // Calculate world width based on grid
    let world_char_width = {
        let grid = app.grid.read();
        match grid {
            Ok(g) => g.width * 4 + 2, // 4 chars per tile + small margin
            Err(_) => 114,
        }
    };

    let total_width = frame.area().width;
    let right_width = total_width.saturating_sub(world_char_width).max(30);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(world_char_width),
            Constraint::Length(right_width),
        ])
        .split(frame.area());

    // Left: world + status bar
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(cols[0]);

    world_view::draw(frame, app, left[0]);
    status_bar::draw(frame, app, left[1]);

    // Right: sidebar on top, detail panel below
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(right_width.min(cols[1].height / 2 + 4)),
            Constraint::Min(8),
        ])
        .split(cols[1]);

    sidebar::draw(frame, app, right[0]);
    details_panel::draw(frame, app, right[1]);
}
