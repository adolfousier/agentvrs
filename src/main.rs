use agentverse::config::AppConfig;
use agentverse::tui;
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

    tui::run(config).await
}
