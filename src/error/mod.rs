use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("world error: {0}")]
    World(String),

    #[error("agent not found: {0}")]
    AgentNotFound(String),

    #[error("agent already exists: {0}")]
    AgentAlreadyExists(String),

    #[error("A2A protocol error: {0}")]
    A2a(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("TUI error: {0}")]
    Tui(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
