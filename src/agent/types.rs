use crate::agent::AgentMessage;
use crate::world::Position;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
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
        let s = self.0.to_string();
        write!(f, "{}", &s[..s.len().min(8)])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Walking,
    Thinking,
    Working,
    Messaging,
    Eating,
    Exercising,
    Playing,
    Error,
    Offline,
}

impl AgentState {
    pub fn label(&self) -> &'static str {
        match self {
            AgentState::Idle => "idle",
            AgentState::Walking => "walking",
            AgentState::Thinking => "thinking",
            AgentState::Working => "working",
            AgentState::Messaging => "messaging",
            AgentState::Eating => "eating",
            AgentState::Exercising => "exercising",
            AgentState::Playing => "playing",
            AgentState::Error => "error",
            AgentState::Offline => "offline",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentKind {
    OpenCrabs { endpoint: String },
    External { endpoint: String },
    Local,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct AnimState {
    pub facing: Facing,
    pub frame: u8,
    pub activity_ticks: u32,
    /// How many consecutive ticks the agent has been blocked during walking.
    pub blocked_ticks: u32,
}

impl Default for AnimState {
    fn default() -> Self {
        Self {
            facing: Facing::Right,
            frame: 0,
            activity_ticks: 0,
            blocked_ticks: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AgentGoal {
    GoToDesk(Position),
    GoToVending(Position),
    GoToCoffee(Position),
    GoToPinball(Position),
    GoToMeeting(Position),
    GoToGym(Position),
    GoToCouch(Position),
    GoToServer(Position),
    Wander(Position),
}

impl AgentGoal {
    pub fn target(&self) -> Position {
        match self {
            AgentGoal::GoToDesk(p)
            | AgentGoal::GoToVending(p)
            | AgentGoal::GoToCoffee(p)
            | AgentGoal::GoToPinball(p)
            | AgentGoal::GoToMeeting(p)
            | AgentGoal::GoToGym(p)
            | AgentGoal::GoToCouch(p)
            | AgentGoal::GoToServer(p)
            | AgentGoal::Wander(p) => *p,
        }
    }
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
    pub anim: AnimState,
    pub goal: Option<AgentGoal>,
    pub path: Vec<Position>,
    pub inbox: VecDeque<AgentMessage>,
    /// When true, the simulation won't override this agent's state.
    /// Set by API task reports; cleared when agent returns to Idle.
    pub api_locked: bool,
}

impl Agent {
    pub fn new(name: impl Into<String>, kind: AgentKind, position: Position) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            kind,
            state: AgentState::Idle,
            position,
            color_index: rand::random::<u8>() % 8,
            last_activity: Instant::now(),
            task_count: 0,
            speech: None,
            anim: AnimState::default(),
            goal: None,
            path: Vec::new(),
            inbox: VecDeque::new(),
            api_locked: false,
        }
    }

    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
        self.last_activity = Instant::now();
    }

    /// Restore an agent from persisted data (e.g. SQLite).
    pub fn restore(
        id: AgentId,
        name: String,
        kind: AgentKind,
        position: Position,
        color_index: u8,
    ) -> Self {
        Self {
            id,
            name,
            kind,
            state: AgentState::Idle,
            position,
            color_index,
            last_activity: Instant::now(),
            task_count: 0,
            speech: None,
            anim: AnimState::default(),
            goal: None,
            path: Vec::new(),
            inbox: VecDeque::new(),
            api_locked: false,
        }
    }

    pub fn say(&mut self, text: impl Into<String>) {
        self.speech = Some(text.into());
        self.last_activity = Instant::now();
    }

    pub fn clear_speech(&mut self) {
        self.speech = None;
    }
}
