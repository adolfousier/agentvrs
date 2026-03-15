use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub world: WorldConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub a2a: A2aConfig,
    #[serde(default)]
    pub gui: GuiConfig,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// API key for authentication. All endpoints (except /health) require X-API-Key header.
    pub api_key: String,
}

impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("enabled", &self.enabled)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aConfig {
    #[serde(default)]
    pub endpoints: Vec<String>,
    #[serde(default = "default_discovery_interval")]
    pub discovery_interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    #[serde(default = "default_win_width")]
    pub window_width: i32,
    #[serde(default = "default_win_height")]
    pub window_height: i32,
    #[serde(default = "default_sidebar_visible")]
    pub sidebar_visible: bool,
    #[serde(default = "default_sidebar_width")]
    pub sidebar_width: i32,
}

fn default_win_width() -> i32 {
    1200
}
fn default_win_height() -> i32 {
    800
}
fn default_sidebar_visible() -> bool {
    true
}
fn default_sidebar_width() -> i32 {
    280
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            window_width: default_win_width(),
            window_height: default_win_height(),
            sidebar_visible: default_sidebar_visible(),
            sidebar_width: default_sidebar_width(),
        }
    }
}

fn default_width() -> u16 {
    10
}
fn default_height() -> u16 {
    8
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
            api_key: String::new(),
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

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = dirs_config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}

fn dirs_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home)
        .join(".config")
        .join("agentverse")
        .join("config.toml")
}
