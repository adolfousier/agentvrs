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
            .constraints([Constraint::Min(8), Constraint::Length(3)])
            .split(frame.area());

        mission_control::draw(frame, app, chunks[0]);
        status_bar::draw(frame, app, chunks[1]);
        return;
    }

    // Detail and MessageLog views with sidebar split (H to toggle)
    if matches!(app.mode, AppMode::AgentDetail | AppMode::MessageLog) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(3)])
            .split(frame.area());

        if app.sidebar_visible {
            let sidebar_width = 34u16.min(rows[0].width / 3);
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(20), Constraint::Length(sidebar_width)])
                .split(rows[0]);

            match app.mode {
                AppMode::AgentDetail => details_panel::draw(frame, app, cols[0]),
                AppMode::MessageLog => message_log::draw(frame, app, cols[0]),
                _ => {}
            }
            sidebar::draw(frame, app, cols[1]);
        } else {
            match app.mode {
                AppMode::AgentDetail => details_panel::draw(frame, app, rows[0]),
                AppMode::MessageLog => message_log::draw(frame, app, rows[0]),
                _ => {}
            }
        }
        status_bar::draw(frame, app, rows[1]);
        return;
    }

    // World view: world takes full screen, sidebar splits when visible
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(frame.area());

    if app.sidebar_visible {
        let sidebar_width = 34u16.min(rows[0].width / 3);
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(sidebar_width)])
            .split(rows[0]);

        world_view::draw(frame, app, cols[0]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(cols[1]);

        sidebar::draw(frame, app, right[0]);
        details_panel::draw(frame, app, right[1]);
    } else {
        world_view::draw(frame, app, rows[0]);
    }

    status_bar::draw(frame, app, rows[1]);
}
