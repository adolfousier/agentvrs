mod details_panel;
mod message_log;
pub mod mission_control;
mod sidebar;
mod status_bar;
mod world_view;

use crate::tui::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

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

    // Detail and MessageLog views are full-screen with sidebar overlay (H to toggle)
    if matches!(app.mode, AppMode::AgentDetail | AppMode::MessageLog) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(3)])
            .split(frame.area());

        match app.mode {
            AppMode::AgentDetail => details_panel::draw(frame, app, rows[0]),
            AppMode::MessageLog => message_log::draw(frame, app, rows[0]),
            _ => {}
        }
        status_bar::draw(frame, app, rows[1]);

        if app.sidebar_visible {
            let sidebar_width = 32u16.min(rows[0].width / 3);
            let sidebar_area = Rect {
                x: rows[0].x + rows[0].width - sidebar_width,
                y: rows[0].y,
                width: sidebar_width,
                height: rows[0].height,
            };
            sidebar::draw(frame, app, sidebar_area);
        }
        return;
    }

    // World view: full-screen world, sidebar overlaid on right (H to toggle)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(frame.area());

    world_view::draw(frame, app, rows[0]);
    status_bar::draw(frame, app, rows[1]);

    // Sidebar overlay on the right side (toggle with H)
    if app.sidebar_visible {
        let sidebar_width = 32u16.min(rows[0].width / 3);
        let sidebar_area = Rect {
            x: rows[0].x + rows[0].width - sidebar_width,
            y: rows[0].y,
            width: sidebar_width,
            height: rows[0].height / 2,
        };
        let detail_area = Rect {
            x: rows[0].x + rows[0].width - sidebar_width,
            y: rows[0].y + rows[0].height / 2,
            width: sidebar_width,
            height: rows[0].height - rows[0].height / 2,
        };
        sidebar::draw(frame, app, sidebar_area);
        details_panel::draw(frame, app, detail_area);
    }
}
