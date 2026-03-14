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
