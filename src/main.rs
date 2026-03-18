use agentverse::config::AppConfig;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;

    let debug_mode = std::env::args().any(|a| a == "--debug" || a == "-d");
    let use_tui = std::env::args().any(|a| a == "--tui");

    if use_tui {
        // In TUI mode, suppress all log output — it would corrupt the terminal UI.
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .with_writer(std::io::sink)
            .init();
    } else {
        let default_level = if debug_mode {
            "agentverse=debug"
        } else {
            "agentverse=info"
        };

        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(default_level.parse()?),
            )
            .with_target(false)
            .init();
    }

    if debug_mode && !use_tui {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let db_path = format!("{}/.config/agentverse/agentverse.db", home);
        tracing::debug!("Debug logging enabled");
        tracing::debug!(
            "Config: host={}, port={}",
            config.server.host,
            config.server.port
        );
        tracing::debug!("Database path: {}", db_path);
    }

    if use_tui {
        agentverse::tui::run(config).await
    } else {
        #[cfg(feature = "bevy3d")]
        {
            agentverse::bevy3d::run(config).await
        }
        #[cfg(not(feature = "bevy3d"))]
        {
            // Fallback to TUI when bevy3d feature is not compiled in
            agentverse::tui::run(config).await
        }
    }
}
