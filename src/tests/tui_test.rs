use crate::agent::{AgentRegistry, MessageLog};
use crate::tui::app::{App, AppMode};
use crate::tui::input::handle_key;
use crate::world::{Grid, WorldEvent};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

fn test_app() -> App {
    let grid = Arc::new(RwLock::new(Grid::new(16, 12)));
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let message_log = Arc::new(RwLock::new(MessageLog::new()));
    let (_tx, rx) = mpsc::channel::<WorldEvent>(64);
    App::new(grid, registry, message_log, rx)
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

fn key_with_mod(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

#[test]
fn test_app_initial_state() {
    let app = test_app();
    assert_eq!(app.mode, AppMode::WorldView);
    assert!(!app.should_quit);
    assert!(app.selected_agent.is_none());
    assert_eq!(app.tick_count, 0);
}

#[test]
fn test_quit_on_q() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char('q')));
    assert!(app.should_quit);
}

#[test]
fn test_quit_on_esc() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Esc));
    assert!(app.should_quit);
}

#[test]
fn test_quit_on_ctrl_c() {
    let mut app = test_app();
    handle_key(
        &mut app,
        key_with_mod(KeyCode::Char('c'), KeyModifiers::CONTROL),
    );
    assert!(app.should_quit);
}

#[test]
fn test_switch_to_message_log() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mode, AppMode::MessageLog);
}

#[test]
fn test_switch_to_command_input() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char(':')));
    assert_eq!(app.mode, AppMode::CommandInput);
}

#[test]
fn test_command_input_typing() {
    let mut app = test_app();
    app.mode = AppMode::CommandInput;
    handle_key(&mut app, key(KeyCode::Char('h')));
    handle_key(&mut app, key(KeyCode::Char('i')));
    assert_eq!(app.command_input, "hi");
}

#[test]
fn test_command_input_backspace() {
    let mut app = test_app();
    app.mode = AppMode::CommandInput;
    handle_key(&mut app, key(KeyCode::Char('a')));
    handle_key(&mut app, key(KeyCode::Char('b')));
    handle_key(&mut app, key(KeyCode::Backspace));
    assert_eq!(app.command_input, "a");
}

#[test]
fn test_command_input_escape_returns_to_world() {
    let mut app = test_app();
    app.mode = AppMode::CommandInput;
    handle_key(&mut app, key(KeyCode::Char('x')));
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mode, AppMode::WorldView);
    assert!(app.command_input.is_empty());
}

#[test]
fn test_command_input_enter_clears_and_returns() {
    let mut app = test_app();
    app.mode = AppMode::CommandInput;
    handle_key(&mut app, key(KeyCode::Char('t')));
    handle_key(&mut app, key(KeyCode::Enter));
    assert_eq!(app.mode, AppMode::WorldView);
    assert!(app.command_input.is_empty());
}

#[test]
fn test_select_next_agent() {
    let mut app = test_app();
    assert_eq!(app.selected_index, 0);
    handle_key(&mut app, key(KeyCode::Char('n')));
    assert_eq!(app.selected_index, 1);
    handle_key(&mut app, key(KeyCode::Char('n')));
    assert_eq!(app.selected_index, 2);
}

#[test]
fn test_select_prev_agent() {
    let mut app = test_app();
    app.selected_index = 3;
    handle_key(&mut app, key(KeyCode::Char('p')));
    assert_eq!(app.selected_index, 2);
    handle_key(&mut app, key(KeyCode::Char('p')));
    assert_eq!(app.selected_index, 1);
}

#[test]
fn test_select_prev_at_zero() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char('p')));
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_camera_pan() {
    let mut app = test_app();
    let start_x = app.camera.x;
    let start_y = app.camera.y;
    handle_key(&mut app, key(KeyCode::Char('l')));
    assert_eq!(app.camera.x, start_x + 1);
    handle_key(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.camera.y, start_y + 1);
}

#[test]
fn test_enter_agent_detail() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Enter));
    assert_eq!(app.mode, AppMode::AgentDetail);
}

#[test]
fn test_agent_detail_escape() {
    let mut app = test_app();
    app.mode = AppMode::AgentDetail;
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mode, AppMode::WorldView);
}

#[test]
fn test_agent_detail_backspace() {
    let mut app = test_app();
    app.mode = AppMode::AgentDetail;
    handle_key(&mut app, key(KeyCode::Backspace));
    assert_eq!(app.mode, AppMode::WorldView);
}

#[test]
fn test_message_log_escape() {
    let mut app = test_app();
    app.mode = AppMode::MessageLog;
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mode, AppMode::WorldView);
}

#[test]
fn test_message_log_tab_returns() {
    let mut app = test_app();
    app.mode = AppMode::MessageLog;
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mode, AppMode::WorldView);
}
