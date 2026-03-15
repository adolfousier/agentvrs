use crate::config::*;

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.world.width, 28);
    assert_eq!(config.world.height, 20);
    assert_eq!(config.world.tick_ms, 200);
    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 18800);
    assert!(config.server.enabled);
    assert!(config.server.api_key.is_empty());
    assert!(config.a2a.endpoints.is_empty());
    assert_eq!(config.a2a.discovery_interval_secs, 30);
}

#[test]
fn test_gui_config_defaults() {
    let gui = GuiConfig::default();
    assert_eq!(gui.window_width, 1200);
    assert_eq!(gui.window_height, 800);
    assert!(gui.sidebar_visible);
    assert_eq!(gui.sidebar_width, 280);
}

#[test]
fn test_config_from_toml_minimal() {
    let toml_str = "";
    let config: AppConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.world.width, 28);
    assert_eq!(config.server.port, 18800);
}

#[test]
fn test_config_from_toml_partial() {
    let toml_str = r#"
[world]
width = 50
height = 40

[server]
port = 9999
api_key = "my-secret"
"#;
    let config: AppConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.world.width, 50);
    assert_eq!(config.world.height, 40);
    assert_eq!(config.world.tick_ms, 200); // default
    assert_eq!(config.server.port, 9999);
    assert_eq!(config.server.api_key, "my-secret");
}

#[test]
fn test_config_roundtrip() {
    let config = AppConfig {
        world: WorldConfig {
            width: 32,
            height: 24,
            tick_ms: 100,
        },
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            enabled: false,
            api_key: "test-key".to_string(),
        },
        a2a: A2aConfig {
            endpoints: vec!["http://localhost:9090".to_string()],
            discovery_interval_secs: 60,
        },
        gui: GuiConfig::default(),
    };

    let toml_str = toml::to_string_pretty(&config).unwrap();
    let parsed: AppConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.world.width, 32);
    assert_eq!(parsed.server.port, 8080);
    assert!(!parsed.server.enabled);
    assert_eq!(parsed.server.api_key, "test-key");
    assert_eq!(parsed.a2a.endpoints.len(), 1);
}

#[test]
fn test_config_save_and_load() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let config = AppConfig {
        world: WorldConfig {
            width: 40,
            height: 30,
            tick_ms: 150,
        },
        ..AppConfig::default()
    };

    // Save
    let content = toml::to_string_pretty(&config).unwrap();
    std::fs::write(&config_path, &content).unwrap();

    // Load
    let loaded_content = std::fs::read_to_string(&config_path).unwrap();
    let loaded: AppConfig = toml::from_str(&loaded_content).unwrap();

    assert_eq!(loaded.world.width, 40);
    assert_eq!(loaded.world.height, 30);
    assert_eq!(loaded.world.tick_ms, 150);
}

#[test]
fn test_config_api_key_redacted_in_debug() {
    let config = ServerConfig {
        api_key: "super-secret-key".to_string(),
        ..ServerConfig::default()
    };
    let debug_output = format!("{:?}", config);
    assert!(!debug_output.contains("super-secret-key"));
    assert!(debug_output.contains("[REDACTED]"));
}

#[test]
fn test_config_api_key_serialized() {
    let mut config = AppConfig::default();
    config.server.api_key = "secret".to_string();
    let toml_str = toml::to_string_pretty(&config).unwrap();
    assert!(toml_str.contains("api_key"));
    // The value should be in the serialized TOML (it's a config file, not debug output)
    assert!(toml_str.contains("secret"));
}
