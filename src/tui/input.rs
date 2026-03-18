use crate::tui::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::WorldView => world_view(app, key),
        AppMode::AgentDetail => agent_detail(app, key),
        AppMode::MessageLog => message_log(app, key),
        AppMode::CommandInput => command_input(app, key),
        AppMode::MissionControl => mission_control(app, key),
    }
}

fn world_view(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Tab => app.mode = AppMode::MessageLog,
        KeyCode::Char(':') => {
            app.mode = AppMode::CommandInput;
            app.command_input.clear();
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.sidebar_visible = !app.sidebar_visible;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.previous_mode = Some(app.mode);
            app.mode = AppMode::MissionControl;
        }
        // Agent selection
        KeyCode::Char('n') | KeyCode::Down | KeyCode::Char('j') => app.selected_index += 1,
        KeyCode::Char('p') | KeyCode::Up | KeyCode::Char('k') => {
            app.selected_index = app.selected_index.saturating_sub(1);
        }
        KeyCode::Enter => app.mode = AppMode::AgentDetail,
        _ => {}
    }
}

fn agent_detail(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace => {
            app.mode = AppMode::WorldView;
            app.selected_agent = None;
        }
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.sidebar_visible = !app.sidebar_visible;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.previous_mode = Some(app.mode);
            app.mode = AppMode::MissionControl;
        }
        _ => {}
    }
}

fn message_log(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.mode = AppMode::WorldView,
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.sidebar_visible = !app.sidebar_visible;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.previous_mode = Some(app.mode);
            app.mode = AppMode::MissionControl;
        }
        _ => {}
    }
}

fn mission_control(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('m') | KeyCode::Char('M') => {
            app.mode = app.previous_mode.take().unwrap_or(AppMode::WorldView);
        }
        KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
}

fn command_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::WorldView;
            app.command_input.clear();
        }
        KeyCode::Enter => {
            app.command_input.clear();
            app.mode = AppMode::WorldView;
        }
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        KeyCode::Char(c) => app.command_input.push(c),
        _ => {}
    }
}
