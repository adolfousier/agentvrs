use crate::agent::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub from: AgentId,
    pub to: AgentId,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

impl AgentMessage {
    pub fn new(from: AgentId, to: AgentId, text: impl Into<String>) -> Self {
        Self {
            from,
            to,
            text: text.into(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Default)]
pub struct MessageLog {
    messages: Vec<AgentMessage>,
}

impl MessageLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, msg: AgentMessage) {
        self.messages.push(msg);
    }

    pub fn messages(&self) -> &[AgentMessage] {
        &self.messages
    }

    pub fn recent(&self, n: usize) -> &[AgentMessage] {
        let start = self.messages.len().saturating_sub(n);
        &self.messages[start..]
    }

    pub fn count(&self) -> usize {
        self.messages.len()
    }
}
