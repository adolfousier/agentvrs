use crate::agent::AgentRegistry;
use crate::config::AppConfig;
use crate::gui::types::ViewState;
use crate::world::{Grid, WorldEvent};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;

/// Shared state accessible from GTK callbacks.
pub struct GuiState {
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub view: Arc<Mutex<ViewState>>,
    pub event_rx: Arc<Mutex<mpsc::Receiver<WorldEvent>>>,
    pub config: Arc<Mutex<AppConfig>>,
}

impl GuiState {
    pub fn new(
        grid: Arc<RwLock<Grid>>,
        registry: Arc<RwLock<AgentRegistry>>,
        event_rx: mpsc::Receiver<WorldEvent>,
        config: AppConfig,
    ) -> Self {
        let sidebar_visible = config.gui.sidebar_visible;
        Self {
            grid,
            registry,
            view: Arc::new(Mutex::new(ViewState::with_sidebar(sidebar_visible))),
            event_rx: Arc::new(Mutex::new(event_rx)),
            config: Arc::new(Mutex::new(config)),
        }
    }

    /// Drain pending world events, updating view state.
    pub fn process_events(&self) {
        let mut rx = self.event_rx.lock().unwrap();
        let mut view = self.view.lock().unwrap();
        while let Ok(event) = rx.try_recv() {
            match event {
                WorldEvent::Tick { count } => view.tick_count = count,
                WorldEvent::AgentSpawned { agent_id, .. } => {
                    view.status_message = Some(format!("Agent {} joined", agent_id));
                }
                WorldEvent::AgentRemoved { agent_id } => {
                    view.status_message = Some(format!("Agent {} left", agent_id));
                    if view.selected_agent == Some(agent_id) {
                        view.selected_agent = None;
                    }
                }
                _ => {}
            }
        }
    }
}
