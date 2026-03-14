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

    let use_gui = std::env::args().any(|a| a == "--gui");

    if use_gui {
        #[cfg(feature = "gui")]
        {
            agentverse::gui::run(config).await
        }
        #[cfg(not(feature = "gui"))]
        {
            anyhow::bail!("GUI not available. Rebuild with: cargo build --features gui")
        }
    } else {
        agentverse::tui::run(config).await
    }
}
