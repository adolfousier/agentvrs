use super::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::WorldView => handle_world_view(app, key),
        AppMode::AgentDetail => handle_agent_detail(app, key),
        AppMode::MessageLog => handle_message_log(app, key),
        AppMode::CommandInput => handle_command_input(app, key),
    }
}

fn handle_world_view(app: &mut App, key: KeyEvent) {
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
        KeyCode::Up | KeyCode::Char('k') => {
            app.selected_index = app.selected_index.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.selected_index += 1;
        }
        KeyCode::Enter => {
            app.mode = AppMode::AgentDetail;
        }
        _ => {}
    }
}

fn handle_agent_detail(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace => {
            app.mode = AppMode::WorldView;
            app.selected_agent = None;
        }
        KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
}

fn handle_message_log(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.mode = AppMode::WorldView,
        KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
}

fn handle_command_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = AppMode::WorldView;
            app.command_input.clear();
        }
        KeyCode::Enter => {
            let _cmd = app.command_input.clone();
            app.command_input.clear();
            app.mode = AppMode::WorldView;
            // Command processing will be added here
        }
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        KeyCode::Char(c) => {
            app.command_input.push(c);
        }
        _ => {}
    }
}
