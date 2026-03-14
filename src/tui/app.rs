use crate::agent::{AgentId, AgentRegistry, MessageLog};
use crate::world::{Grid, WorldEvent};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    WorldView,
    AgentDetail,
    MessageLog,
    CommandInput,
}

pub struct App {
    pub mode: AppMode,
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub message_log: Arc<RwLock<MessageLog>>,
    pub event_rx: mpsc::Receiver<WorldEvent>,
    pub selected_agent: Option<AgentId>,
    pub selected_index: usize,
    pub tick_count: u64,
    pub should_quit: bool,
    pub command_input: String,
    pub status_message: Option<String>,
}

impl App {
    pub fn new(
        grid: Arc<RwLock<Grid>>,
        registry: Arc<RwLock<AgentRegistry>>,
        message_log: Arc<RwLock<MessageLog>>,
        event_rx: mpsc::Receiver<WorldEvent>,
    ) -> Self {
        Self {
            mode: AppMode::WorldView,
            grid,
            registry,
            message_log,
            event_rx,
            selected_agent: None,
            selected_index: 0,
            tick_count: 0,
            should_quit: false,
            command_input: String::new(),
            status_message: None,
        }
    }

    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                WorldEvent::Tick { count } => {
                    self.tick_count = count;
                }
                WorldEvent::AgentSpawned { agent_id, .. } => {
                    self.status_message = Some(format!("Agent {} joined the world", agent_id));
                }
                WorldEvent::AgentRemoved { agent_id } => {
                    self.status_message = Some(format!("Agent {} left the world", agent_id));
                    if self.selected_agent == Some(agent_id) {
                        self.selected_agent = None;
                    }
                }
                _ => {}
            }
        }
    }
}
