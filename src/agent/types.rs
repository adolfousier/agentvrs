use crate::world::Position;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl Default for AgentId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl AgentId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.to_string()[..8])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Thinking,
    Working,
    Messaging,
    Error,
    Offline,
}

impl AgentState {
    pub fn label(&self) -> &'static str {
        match self {
            AgentState::Idle => "idle",
            AgentState::Thinking => "thinking",
            AgentState::Working => "working",
            AgentState::Messaging => "messaging",
            AgentState::Error => "error",
            AgentState::Offline => "offline",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentKind {
    /// Connected via A2A protocol (OpenCrabs or compatible)
    OpenCrabs { endpoint: String },
    /// Connected via HTTP API
    External { endpoint: String },
    /// Local demo/test agent
    Local,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub kind: AgentKind,
    pub state: AgentState,
    pub position: Position,
    pub color_index: u8,
    pub last_activity: Instant,
    pub task_count: u32,
    pub speech: Option<String>,
}

impl Agent {
    pub fn new(name: impl Into<String>, kind: AgentKind, position: Position) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            kind,
            state: AgentState::Idle,
            position,
            color_index: rand::random::<u8>() % 6,
            last_activity: Instant::now(),
            task_count: 0,
            speech: None,
        }
    }

    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
        self.last_activity = Instant::now();
    }

    pub fn say(&mut self, text: impl Into<String>) {
        self.speech = Some(text.into());
        self.last_activity = Instant::now();
    }

    pub fn clear_speech(&mut self) {
        self.speech = None;
    }
}
