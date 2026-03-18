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
    use crate::tui::app::McPanel;

    // If detail popup is open, Esc/Enter closes it
    if app.mc_detail_open {
        match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Backspace => {
                app.mc_detail_open = false;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.mc_scroll = 0;
            app.mc_selected = 0;
            app.mc_panel = McPanel::Agents;
            app.mc_detail_open = false;
            app.mode = app.previous_mode.take().unwrap_or(AppMode::WorldView);
        }
        KeyCode::Esc => {
            app.mc_scroll = 0;
            app.mc_selected = 0;
            app.mc_panel = McPanel::Agents;
            app.mc_detail_open = false;
            app.mode = app.previous_mode.take().unwrap_or(AppMode::WorldView);
        }
        KeyCode::Char('q') => app.should_quit = true,
        // Tab cycles between panels
        KeyCode::Tab => {
            app.mc_selected = 0;
            app.mc_scroll = 0;
            app.mc_panel = match app.mc_panel {
                McPanel::Agents => McPanel::Activity,
                McPanel::Activity => McPanel::Tasks,
                McPanel::Tasks => McPanel::Agents,
            };
        }
        // j/k navigates within focused panel
        KeyCode::Char('j') | KeyCode::Down => {
            app.mc_selected = app.mc_selected.saturating_add(1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.mc_selected = app.mc_selected.saturating_sub(1);
        }
        // Enter opens detail for selected item
        KeyCode::Enter => {
            app.mc_detail_open = true;
        }
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
