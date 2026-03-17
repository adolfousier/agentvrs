use crate::agent::{Agent, AgentId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AgentRegistry {
    agents: HashMap<AgentId, Agent>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, agent: Agent) -> AgentId {
        let id = agent.id;
        self.agents.insert(id, agent);
        id
    }

    pub fn remove(&mut self, id: &AgentId) -> Option<Agent> {
        self.agents.remove(id)
    }

    pub fn get(&self, id: &AgentId) -> Option<&Agent> {
        self.agents.get(id)
    }

    pub fn get_mut(&mut self, id: &AgentId) -> Option<&mut Agent> {
        self.agents.get_mut(id)
    }

    pub fn agents(&self) -> impl Iterator<Item = &Agent> {
        self.agents.values()
    }

    pub fn count(&self) -> usize {
        self.agents.len()
    }

    pub fn ids(&self) -> Vec<AgentId> {
        self.agents.keys().copied().collect()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Agent> {
        self.agents.values().find(|a| a.name == name)
    }
}
