use super::observability::{ActivityEntry, HeartbeatInfo, TaskRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectResponse {
    pub agent_id: String,
    pub position: (u16, u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAgent {
    pub id: String,
    pub name: String,
    pub state: String,
    pub position: (u16, u16),
    pub task_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub text: String,
    /// Optional target agent ID for agent-to-agent messaging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub width: u16,
    pub height: u16,
    pub agents: Vec<ApiAgent>,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub agents: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRequest {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRequest {
    /// One of: "desk", "vending", "coffee", "pinball", "gym", "couch", "wander"
    pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRequest {
    /// One of: "idle", "walking", "thinking", "working", "messaging", "eating", "exercising", "playing", "error", "offline"
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCell {
    pub tile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMapResponse {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Vec<ApiCell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub status: String,
    pub agent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxMessage {
    pub from: String,
    pub from_name: String,
    pub text: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxResponse {
    pub agent_id: String,
    pub count: usize,
    pub messages: Vec<InboxMessage>,
}

// ─── Observability Types ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAgentDetail {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub state: String,
    pub position: (u16, u16),
    pub task_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    pub last_activity_secs_ago: u64,
    pub connection_health: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub status: String,
    pub last_seen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatusResponse {
    pub agent_id: String,
    pub name: String,
    pub state: String,
    pub connection_health: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<HeartbeatInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponse {
    pub agent_id: String,
    pub count: usize,
    pub entries: Vec<ActivityEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryResponse {
    pub agent_id: String,
    pub count: usize,
    pub tasks: Vec<TaskRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LimitQuery {
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub agent: ApiAgentDetail,
    pub recent_activity: Vec<ActivityEntry>,
    pub task_history: Vec<TaskRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<HeartbeatInfo>,
    pub connection_health: String,
}
