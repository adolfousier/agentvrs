use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub world: WorldConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub a2a: A2aConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    #[serde(default = "default_width")]
    pub width: u16,
    #[serde(default = "default_height")]
    pub height: u16,
    #[serde(default = "default_tick_ms")]
    pub tick_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aConfig {
    #[serde(default)]
    pub endpoints: Vec<String>,
    #[serde(default = "default_discovery_interval")]
    pub discovery_interval_secs: u64,
}

fn default_width() -> u16 {
    28
}
fn default_height() -> u16 {
    20
}
fn default_tick_ms() -> u64 {
    200
}
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    18800
}
fn default_enabled() -> bool {
    true
}
fn default_discovery_interval() -> u64 {
    30
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            tick_ms: default_tick_ms(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            enabled: default_enabled(),
        }
    }
}

impl Default for A2aConfig {
    fn default() -> Self {
        Self {
            endpoints: Vec::new(),
            discovery_interval_secs: default_discovery_interval(),
        }
    }
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = dirs_config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }
}

fn dirs_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home)
        .join(".config")
        .join("agentverse")
        .join("config.toml")
}
