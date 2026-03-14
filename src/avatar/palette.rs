use crate::agent::AgentState;
use ratatui::style::Color;

/// Agent color by index (for distinguishing agents visually).
pub fn agent_color(index: u8) -> Color {
    match index % 6 {
        0 => Color::Cyan,
        1 => Color::Green,
        2 => Color::Magenta,
        3 => Color::Yellow,
        4 => Color::Blue,
        _ => Color::Red,
    }
}

/// State indicator color.
pub fn state_color(state: &AgentState) -> Color {
    match state {
        AgentState::Idle => Color::Gray,
        AgentState::Thinking => Color::Yellow,
        AgentState::Working => Color::Green,
        AgentState::Messaging => Color::Cyan,
        AgentState::Error => Color::Red,
        AgentState::Offline => Color::DarkGray,
    }
}

/// State indicator symbol for the sidebar/status.
pub fn state_symbol(state: &AgentState) -> &'static str {
    match state {
        AgentState::Idle => "~",
        AgentState::Thinking => "?",
        AgentState::Working => "*",
        AgentState::Messaging => ">",
        AgentState::Error => "!",
        AgentState::Offline => "x",
    }
}
