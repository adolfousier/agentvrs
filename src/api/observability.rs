use crate::agent::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    StateChange,
    Movement,
    MessageSent,
    MessageReceived,
    GoalAssigned,
    TaskSubmitted,
    TaskCompleted,
    TaskFailed,
    Spawned,
    Removed,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub timestamp: DateTime<Utc>,
    pub kind: ActivityKind,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatInfo {
    pub last_seen: DateTime<Utc>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecord {
    pub task_id: String,
    pub submitted_at: DateTime<Utc>,
    pub state: String,
    pub last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_summary: Option<String>,
}

/// Central observability store for all agents.
pub struct AgentObserver {
    activity: HashMap<AgentId, VecDeque<ActivityEntry>>,
    heartbeats: HashMap<AgentId, HeartbeatInfo>,
    tasks: HashMap<AgentId, VecDeque<TaskRecord>>,
    max_activity: usize,
    max_tasks: usize,
}

impl AgentObserver {
    pub fn new(max_activity: usize, max_tasks: usize) -> Self {
        Self {
            activity: HashMap::new(),
            heartbeats: HashMap::new(),
            tasks: HashMap::new(),
            max_activity,
            max_tasks,
        }
    }

    pub fn record_activity(
        &mut self,
        agent_id: AgentId,
        kind: ActivityKind,
        detail: impl Into<String>,
    ) {
        let entries = self.activity.entry(agent_id).or_default();
        entries.push_back(ActivityEntry {
            timestamp: Utc::now(),
            kind,
            detail: detail.into(),
        });
        while entries.len() > self.max_activity {
            entries.pop_front();
        }
    }

    pub fn update_heartbeat(
        &mut self,
        agent_id: AgentId,
        status: impl Into<String>,
        metadata: Option<serde_json::Value>,
    ) {
        let info = HeartbeatInfo {
            last_seen: Utc::now(),
            status: status.into(),
            metadata,
        };
        self.heartbeats.insert(agent_id, info);
    }

    pub fn record_task(
        &mut self,
        agent_id: AgentId,
        task_id: impl Into<String>,
        state: impl Into<String>,
        response_summary: Option<String>,
    ) {
        let now = Utc::now();
        let records = self.tasks.entry(agent_id).or_default();
        let task_id = task_id.into();
        let state = state.into();

        // Update existing task or create new
        if let Some(existing) = records.iter_mut().find(|t| t.task_id == task_id) {
            existing.state = state;
            existing.last_updated = now;
            if response_summary.is_some() {
                existing.response_summary = response_summary;
            }
        } else {
            records.push_back(TaskRecord {
                task_id,
                submitted_at: now,
                state,
                last_updated: now,
                response_summary,
            });
            while records.len() > self.max_tasks {
                records.pop_front();
            }
        }
    }

    pub fn get_activity(&self, agent_id: &AgentId, limit: usize) -> Vec<&ActivityEntry> {
        self.activity
            .get(agent_id)
            .map(|entries| {
                let skip = entries.len().saturating_sub(limit);
                entries.iter().skip(skip).collect()
            })
            .unwrap_or_default()
    }

    pub fn get_heartbeat(&self, agent_id: &AgentId) -> Option<&HeartbeatInfo> {
        self.heartbeats.get(agent_id)
    }

    pub fn get_tasks(&self, agent_id: &AgentId, limit: usize) -> Vec<&TaskRecord> {
        self.tasks
            .get(agent_id)
            .map(|records| {
                let skip = records.len().saturating_sub(limit);
                records.iter().skip(skip).collect()
            })
            .unwrap_or_default()
    }

    pub fn connection_health(&self, agent_id: &AgentId) -> &'static str {
        match self.heartbeats.get(agent_id) {
            Some(hb) => {
                let elapsed = Utc::now() - hb.last_seen;
                if elapsed.num_seconds() < 60 {
                    "online"
                } else if elapsed.num_seconds() < 300 {
                    "stale"
                } else {
                    "offline"
                }
            }
            None => "unknown",
        }
    }

    /// Returns true if the agent has any tasks in submitted or running state.
    pub fn has_active_tasks(&self, agent_id: &AgentId) -> bool {
        self.tasks.get(agent_id).map_or(false, |records| {
            records.iter().any(|t| t.state == "submitted" || t.state == "running")
        })
    }

    pub fn delete_task(&mut self, agent_id: &AgentId, task_id: &str) -> bool {
        if let Some(records) = self.tasks.get_mut(agent_id) {
            let before = records.len();
            records.retain(|t| t.task_id != task_id);
            return records.len() < before;
        }
        false
    }

    pub fn remove_agent(&mut self, agent_id: &AgentId) {
        self.activity.remove(agent_id);
        self.heartbeats.remove(agent_id);
        self.tasks.remove(agent_id);
    }
}
