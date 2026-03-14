use super::app::{App, AppMode};
use crate::world::Position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::WorldView => world_view(app, key),
        AppMode::AgentDetail => agent_detail(app, key),
        AppMode::MessageLog => message_log(app, key),
        AppMode::CommandInput => command_input(app, key),
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
        // Camera pan
        KeyCode::Left | KeyCode::Char('h') => {
            app.camera.x = app.camera.x.saturating_sub(1);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            let bounds = app.grid.read().unwrap().bounds();
            app.camera.x = (app.camera.x + 1).min(bounds.0.saturating_sub(1));
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.camera.y = app.camera.y.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let bounds = app.grid.read().unwrap().bounds();
            app.camera.y = (app.camera.y + 1).min(bounds.1.saturating_sub(1));
        }
        // Agent selection
        KeyCode::Char('n') => app.selected_index += 1,
        KeyCode::Char('p') => app.selected_index = app.selected_index.saturating_sub(1),
        KeyCode::Enter => app.mode = AppMode::AgentDetail,
        // Center on selected agent
        KeyCode::Char('c') => {
            let reg = app.registry.read().unwrap();
            let agents: Vec<_> = reg.agents().collect();
            if app.selected_index < agents.len() {
                app.camera = agents[app.selected_index].position;
            }
        }
        // Fit world
        KeyCode::Char('f') => {
            let g = app.grid.read().unwrap();
            app.camera = Position::new(g.width / 2, g.height / 2);
        }
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
        _ => {}
    }
}

fn message_log(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.mode = AppMode::WorldView,
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
