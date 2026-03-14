use crate::agent::AgentState;

/// Returns the sprite character(s) for an agent on the grid.
/// Each agent occupies a 2-wide cell in the terminal.
pub fn agent_sprite(state: &AgentState) -> &'static str {
    match state {
        AgentState::Idle => "@@",
        AgentState::Thinking => "??",
        AgentState::Working => "##",
        AgentState::Messaging => "<>",
        AgentState::Error => "!!",
        AgentState::Offline => "..",
    }
}

/// Returns the tile character for rendering the grid.
pub fn tile_sprite(is_wall: bool) -> &'static str {
    if is_wall { "██" } else { "  " }
}
