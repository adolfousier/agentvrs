use crate::a2a::client::A2aClient;
use crate::a2a::types::*;
use crate::agent::{Agent, AgentId, AgentKind, AgentRegistry, AgentState};
use crate::world::{Grid, Position, WorldEvent};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

/// Maps A2A task states to visual agent states.
fn task_state_to_agent_state(state: &TaskState) -> AgentState {
    match state {
        TaskState::Submitted => AgentState::Thinking,
        TaskState::Working => AgentState::Working,
        TaskState::Completed => AgentState::Idle,
        TaskState::Failed => AgentState::Error,
        TaskState::Canceled => AgentState::Idle,
        TaskState::InputRequired => AgentState::Messaging,
        TaskState::Rejected => AgentState::Error,
        TaskState::AuthRequired => AgentState::Error,
    }
}

/// Bridge between A2A protocol and the visual agent world.
pub struct A2aBridge {
    registry: Arc<RwLock<AgentRegistry>>,
    grid: Arc<RwLock<Grid>>,
    event_tx: mpsc::Sender<WorldEvent>,
    clients: HashMap<AgentId, A2aClient>,
}

impl A2aBridge {
    pub fn new(
        registry: Arc<RwLock<AgentRegistry>>,
        grid: Arc<RwLock<Grid>>,
        event_tx: mpsc::Sender<WorldEvent>,
    ) -> Self {
        Self {
            registry,
            grid,
            event_tx,
            clients: HashMap::new(),
        }
    }

    /// Discover and connect an A2A agent by its base URL.
    pub async fn connect(&mut self, base_url: &str) -> Result<AgentId> {
        let card = A2aClient::discover(base_url).await?;

        let endpoint = card
            .supported_interfaces
            .iter()
            .find(|i| i.protocol_binding == "JSONRPC")
            .map(|i| i.url.clone())
            .unwrap_or_else(|| format!("{}/a2a/v1", base_url.trim_end_matches('/')));

        let client = A2aClient::new(&endpoint);

        let position = {
            let grid = self.grid.read().map_err(|_| anyhow::anyhow!("grid lock poisoned"))?;
            grid.find_empty_floor().unwrap_or(Position::new(1, 1))
        };

        let agent = Agent::new(
            &card.name,
            AgentKind::OpenCrabs {
                endpoint: endpoint.clone(),
            },
            position,
        );
        let agent_id = agent.id;

        {
            let mut grid = self.grid.write().map_err(|_| anyhow::anyhow!("grid lock poisoned"))?;
            grid.place_agent(position, agent_id);
        }

        {
            let mut reg = self.registry.write().map_err(|_| anyhow::anyhow!("registry lock poisoned"))?;
            reg.register(agent);
        }

        self.clients.insert(agent_id, client);

        let _ = self
            .event_tx
            .send(WorldEvent::AgentSpawned { agent_id, position })
            .await;

        Ok(agent_id)
    }

    /// Send a message to an A2A agent and update its visual state.
    pub async fn send_message(&self, agent_id: &AgentId, text: &str) -> Result<Task> {
        let client = self
            .clients
            .get(agent_id)
            .ok_or_else(|| anyhow::anyhow!("no A2A client for agent {}", agent_id))?;

        {
            let mut reg = self.registry.write().map_err(|_| anyhow::anyhow!("registry lock poisoned"))?;
            if let Some(agent) = reg.get_mut(agent_id) {
                agent.set_state(AgentState::Thinking);
            }
        }
        let _ = self
            .event_tx
            .send(WorldEvent::AgentStateChanged {
                agent_id: *agent_id,
                state: AgentState::Thinking,
            })
            .await;

        let params = SendMessageParams {
            message: Message {
                message_id: Some(uuid::Uuid::new_v4().to_string()),
                context_id: None,
                task_id: None,
                role: Role::User,
                parts: vec![Part::text(text)],
                metadata: None,
            },
            configuration: None,
            metadata: None,
        };

        let task = client.send_message(params).await?;

        let new_state = task_state_to_agent_state(&task.status.state);
        {
            let mut reg = self.registry.write().map_err(|_| anyhow::anyhow!("registry lock poisoned"))?;
            if let Some(agent) = reg.get_mut(agent_id) {
                agent.set_state(new_state.clone());
                agent.task_count += 1;

                if let Some(ref msg) = task.status.message
                    && let Some(part) = msg.parts.first()
                    && let Some(ref text) = part.text
                {
                    agent.say(text.clone());
                }
            }
        }
        let _ = self
            .event_tx
            .send(WorldEvent::AgentStateChanged {
                agent_id: *agent_id,
                state: new_state,
            })
            .await;

        Ok(task)
    }

    /// Disconnect an A2A agent.
    pub async fn disconnect(&mut self, agent_id: &AgentId) -> Result<()> {
        self.clients.remove(agent_id);

        {
            let mut reg = self.registry.write().map_err(|_| anyhow::anyhow!("registry lock poisoned"))?;
            if let Some(agent) = reg.remove(agent_id) {
                let mut grid = self.grid.write().map_err(|_| anyhow::anyhow!("grid lock poisoned"))?;
                grid.remove_agent(agent.position);
            }
        }

        let _ = self
            .event_tx
            .send(WorldEvent::AgentRemoved {
                agent_id: *agent_id,
            })
            .await;

        Ok(())
    }
}
