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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub text: String,
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
