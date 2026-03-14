use super::Position;
use crate::agent::AgentId;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum WorldEvent {
    AgentSpawned {
        agent_id: AgentId,
        position: Position,
    },
    AgentMoved {
        agent_id: AgentId,
        from: Position,
        to: Position,
    },
    AgentStateChanged {
        agent_id: AgentId,
        state: crate::agent::AgentState,
    },
    AgentRemoved {
        agent_id: AgentId,
    },
    MessageSent {
        from: AgentId,
        to: AgentId,
        text: String,
    },
    Tick {
        count: u64,
    },
}
