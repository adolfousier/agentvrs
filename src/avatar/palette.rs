use crate::agent::AgentState;
use crate::world::{FloorKind, WallKind};
use ratatui::style::Color;

pub fn agent_color(index: u8) -> Color {
    shirt_color(index)
}

pub fn shirt_color(index: u8) -> Color {
    match index % 8 {
        0 => Color::Rgb(66, 133, 244),
        1 => Color::Rgb(234, 67, 53),
        2 => Color::Rgb(251, 188, 4),
        3 => Color::Rgb(52, 168, 83),
        4 => Color::Rgb(155, 89, 182),
        5 => Color::Rgb(230, 126, 34),
        6 => Color::Rgb(26, 188, 156),
        _ => Color::Rgb(241, 196, 15),
    }
}

pub fn skin_color(index: u8) -> Color {
    match index % 4 {
        0 => Color::Rgb(255, 218, 185),
        1 => Color::Rgb(210, 170, 120),
        2 => Color::Rgb(160, 110, 70),
        _ => Color::Rgb(100, 70, 40),
    }
}

pub fn hair_color(index: u8) -> Color {
    match index % 4 {
        0 => Color::Rgb(40, 30, 20),
        1 => Color::Rgb(180, 120, 50),
        2 => Color::Rgb(200, 60, 30),
        _ => Color::Rgb(60, 60, 60),
    }
}

pub fn state_color(state: &AgentState) -> Color {
    match state {
        AgentState::Idle => Color::Gray,
        AgentState::Walking => Color::White,
        AgentState::Thinking => Color::Yellow,
        AgentState::Working => Color::Green,
        AgentState::Messaging => Color::Cyan,
        AgentState::Eating => Color::Rgb(255, 165, 0),
        AgentState::Exercising => Color::Rgb(255, 100, 100),
        AgentState::Playing => Color::Magenta,
        AgentState::Error => Color::Red,
        AgentState::Offline => Color::DarkGray,
    }
}

pub fn state_symbol(state: &AgentState) -> &'static str {
    match state {
        AgentState::Idle => "~",
        AgentState::Walking => ">",
        AgentState::Thinking => "?",
        AgentState::Working => "*",
        AgentState::Messaging => "@",
        AgentState::Eating => "o",
        AgentState::Exercising => "!",
        AgentState::Playing => "^",
        AgentState::Error => "x",
        AgentState::Offline => ".",
    }
}

pub fn floor_colors(kind: &FloorKind) -> (Color, Color) {
    match kind {
        FloorKind::Wood => (Color::Rgb(150, 110, 70), Color::Rgb(130, 85, 40)),
        FloorKind::Tile => (Color::Rgb(210, 210, 215), Color::Rgb(195, 195, 205)),
        FloorKind::Carpet => (Color::Rgb(55, 55, 95), Color::Rgb(45, 45, 80)),
        FloorKind::Concrete => (Color::Rgb(130, 130, 135), Color::Rgb(115, 115, 120)),
    }
}

pub fn wall_color(kind: &WallKind) -> Color {
    match kind {
        WallKind::Solid => Color::Rgb(80, 80, 90),
        WallKind::Window => Color::Rgb(100, 110, 130),
    }
}
