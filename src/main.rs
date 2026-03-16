use agentverse::config::AppConfig;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("agentverse=info".parse()?),
        )
        .with_target(false)
        .init();

    let use_tui = std::env::args().any(|a| a == "--tui");

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
