use crate::config::AppConfig;
use crate::gui::app::GuiState;
use crate::gui::window;
use crate::runner;
use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, glib};
use std::sync::Arc;

const APP_ID: &str = "ai.neura.agentverse";

pub async fn run(config: AppConfig) -> Result<()> {
    let world_w = config.world.width;
    let world_h = config.world.height;

    let rt = runner::setup(&config, world_w, world_h).await?;
    let tick_ms = config.world.tick_ms;

    let state = Arc::new(GuiState::new(rt.grid, rt.registry, rt.event_rx));

    // GTK must run on main thread; tokio runtime is already running
    let app = Application::builder().application_id(APP_ID).build();

    let state_clone = Arc::clone(&state);
    app.connect_activate(move |app| {
        window::build(app, &state_clone, tick_ms);
    });

    // Run GTK main loop (blocks until window closes)
    let exit_code = app.run_with_args::<String>(&[]);

    let _ = rt.shutdown_tx.send(()).await;

    if exit_code != glib::ExitCode::SUCCESS {
        anyhow::bail!("GTK application exited with error");
    }
    Ok(())
}
