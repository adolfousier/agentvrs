use crate::agent::{AgentId, AgentRegistry, MessageLog};
use crate::api::observability::AgentObserver;
use crate::db::Database;
use crate::world::{Grid, WorldEvent};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    WorldView,
    AgentDetail,
    MessageLog,
    CommandInput,
    MissionControl,
}

/// Which MC panel has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McPanel {
    Agents,
    Activity,
    Tasks,
}

pub struct App {
    pub mode: AppMode,
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub message_log: Arc<RwLock<MessageLog>>,
    pub observer: Arc<RwLock<AgentObserver>>,
    pub db: Arc<Mutex<Database>>,
    pub event_rx: mpsc::Receiver<WorldEvent>,
    pub selected_agent: Option<AgentId>,
    pub selected_index: usize,
    pub tick_count: u64,
    pub should_quit: bool,
    pub command_input: String,
    pub status_message: Option<String>,
    /// Previous mode before entering MC (to restore on exit)
    pub previous_mode: Option<AppMode>,
    /// Whether the sidebar is visible (toggle with H, matches Bevy behavior)
    pub sidebar_visible: bool,
    /// Which MC panel is focused
    pub mc_panel: McPanel,
    /// Selected item index within the focused MC panel
    pub mc_selected: usize,
    /// Scroll offset for MC panels
    pub mc_scroll: u16,
    /// Whether showing an agent detail popup in MC
    pub mc_detail_open: bool,
}

impl App {
    pub fn new(
        grid: Arc<RwLock<Grid>>,
        registry: Arc<RwLock<AgentRegistry>>,
        message_log: Arc<RwLock<MessageLog>>,
        observer: Arc<RwLock<AgentObserver>>,
        db: Arc<Mutex<Database>>,
        event_rx: mpsc::Receiver<WorldEvent>,
    ) -> Self {
        Self {
            mode: AppMode::WorldView,
            grid,
            registry,
            message_log,
            observer,
            db,
            event_rx,
            selected_agent: None,
            selected_index: 0,
            tick_count: 0,
            should_quit: false,
            command_input: String::new(),
            status_message: None,
            previous_mode: None,
            sidebar_visible: true,
            mc_panel: McPanel::Agents,
            mc_selected: 0,
            mc_scroll: 0,
            mc_detail_open: false,
        }
    }

    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                WorldEvent::Tick { count } => self.tick_count = count,
                WorldEvent::AgentSpawned { agent_id, .. } => {
                    self.status_message = Some(format!("Agent {} joined", agent_id));
                }
                WorldEvent::AgentRemoved { agent_id } => {
                    self.status_message = Some(format!("Agent {} left", agent_id));
                    if self.selected_agent == Some(agent_id) {
                        self.selected_agent = None;
                    }
                }
                _ => {}
            }
        }
    }
}
