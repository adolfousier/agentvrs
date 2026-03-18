use crate::agent::{AgentRegistry, MessageLog};
use crate::api::observability::AgentObserver;
use crate::db::Database;
use crate::tui::app::{App, AppMode};
use crate::tui::input::handle_key;
use crate::world::{Grid, WorldEvent};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;

fn test_app() -> App {
    let grid = Arc::new(RwLock::new(Grid::new(16, 12)));
    let registry = Arc::new(RwLock::new(AgentRegistry::new()));
    let message_log = Arc::new(RwLock::new(MessageLog::new()));
    let observer = Arc::new(RwLock::new(AgentObserver::new(100, 50)));
    let db = Arc::new(Mutex::new(Database::open_in_memory().expect("test db")));
    let (_tx, rx) = mpsc::channel::<WorldEvent>(64);
    App::new(grid, registry, message_log, observer, db, rx)
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
fn test_select_with_arrow_keys() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Down));
    assert_eq!(app.selected_index, 1);
    handle_key(&mut app, key(KeyCode::Up));
    assert_eq!(app.selected_index, 0);
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

// ── Mission Control tests ────────────────────────────────────────────

#[test]
fn test_m_key_opens_mission_control() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char('m')));
    assert_eq!(app.mode, AppMode::MissionControl);
    assert_eq!(app.previous_mode, Some(AppMode::WorldView));
}

#[test]
fn test_m_key_closes_mission_control() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    handle_key(&mut app, key(KeyCode::Char('m')));
    assert_eq!(app.mode, AppMode::WorldView);
}

#[test]
fn test_esc_closes_mission_control() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::AgentDetail);
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mode, AppMode::AgentDetail);
}

#[test]
fn test_mc_from_detail_restores_detail() {
    let mut app = test_app();
    app.mode = AppMode::AgentDetail;
    handle_key(&mut app, key(KeyCode::Char('m')));
    assert_eq!(app.mode, AppMode::MissionControl);
    assert_eq!(app.previous_mode, Some(AppMode::AgentDetail));
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mode, AppMode::AgentDetail);
}

#[test]
fn test_mc_quit_works() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    handle_key(&mut app, key(KeyCode::Char('q')));
    assert!(app.should_quit);
}

#[test]
fn test_mc_from_message_log() {
    let mut app = test_app();
    app.mode = AppMode::MessageLog;
    handle_key(&mut app, key(KeyCode::Char('M')));
    assert_eq!(app.mode, AppMode::MissionControl);
    assert_eq!(app.previous_mode, Some(AppMode::MessageLog));
}

// ── Sidebar toggle tests ────────────────────────────────────────────

#[test]
fn test_sidebar_visible_by_default() {
    let app = test_app();
    assert!(app.sidebar_visible);
}

#[test]
fn test_h_key_toggles_sidebar() {
    let mut app = test_app();
    assert!(app.sidebar_visible);
    handle_key(&mut app, key(KeyCode::Char('h')));
    assert!(!app.sidebar_visible);
    handle_key(&mut app, key(KeyCode::Char('h')));
    assert!(app.sidebar_visible);
}

#[test]
fn test_h_key_uppercase_toggles_sidebar() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char('H')));
    assert!(!app.sidebar_visible);
}

#[test]
fn test_sidebar_toggle_stays_in_world_view() {
    let mut app = test_app();
    handle_key(&mut app, key(KeyCode::Char('h')));
    assert_eq!(app.mode, AppMode::WorldView);
}

// ── MC scroll tests ────────────────────────────────────────────

#[test]
fn test_mc_scroll_default_zero() {
    let app = test_app();
    assert_eq!(app.mc_scroll, 0);
}

#[test]
fn test_mc_j_selects_down() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    handle_key(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.mc_selected, 1);
    handle_key(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.mc_selected, 2);
}

#[test]
fn test_mc_k_selects_up() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    app.mc_selected = 3;
    handle_key(&mut app, key(KeyCode::Char('k')));
    assert_eq!(app.mc_selected, 2);
}

#[test]
fn test_mc_k_at_zero_stays_zero() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    handle_key(&mut app, key(KeyCode::Char('k')));
    assert_eq!(app.mc_scroll, 0);
}

#[test]
fn test_mc_exit_resets_scroll() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    app.mc_scroll = 10;
    handle_key(&mut app, key(KeyCode::Esc));
    assert_eq!(app.mc_scroll, 0);
    assert_eq!(app.mode, AppMode::WorldView);
}

#[test]
fn test_mc_tab_cycles_panels() {
    use crate::tui::app::McPanel;
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    assert_eq!(app.mc_panel, McPanel::Agents);
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mc_panel, McPanel::Activity);
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mc_panel, McPanel::Tasks);
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mc_panel, McPanel::Agents);
}

#[test]
fn test_mc_tab_resets_selection() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    app.mc_selected = 5;
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mc_selected, 0);
}

#[test]
fn test_mc_enter_opens_detail() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    assert!(!app.mc_detail_open);
    handle_key(&mut app, key(KeyCode::Enter));
    assert!(app.mc_detail_open);
    assert_eq!(app.mode, AppMode::MissionControl);
}

#[test]
fn test_mc_detail_esc_closes() {
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    app.mc_detail_open = true;
    handle_key(&mut app, key(KeyCode::Esc));
    assert!(!app.mc_detail_open);
    assert_eq!(app.mode, AppMode::MissionControl);
}

#[test]
fn test_mc_detail_blocks_navigation() {
    use crate::tui::app::McPanel;
    let mut app = test_app();
    app.mode = AppMode::MissionControl;
    app.previous_mode = Some(AppMode::WorldView);
    app.mc_detail_open = true;
    app.mc_selected = 2;
    // j/k/Tab should not work when detail is open
    handle_key(&mut app, key(KeyCode::Char('j')));
    assert_eq!(app.mc_selected, 2);
    handle_key(&mut app, key(KeyCode::Tab));
    assert_eq!(app.mc_panel, McPanel::Agents);
}
