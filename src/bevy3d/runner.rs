use crate::bevy3d::bridge::WorldBridge;
use crate::bevy3d::camera::CameraState;
use crate::bevy3d::interaction::SelectedAgent;
use crate::bevy3d::sim_system::SimState;
use crate::bevy3d::sync::SyncState;
use crate::config::AppConfig;
use crate::runner;
use anyhow::Result;
use bevy::prelude::*;

pub async fn run(config: AppConfig) -> Result<()> {
    let world_w = config.world.width;
    let world_h = config.world.height;
    let tick_ms = config.world.tick_ms;

    let rt = runner::setup_no_sim(&config, world_w, world_h).await?;

    let grid = rt.grid;
    let registry = rt.registry;
    let _shutdown_tx = rt.shutdown_tx;
    let sim_state = SimState {
        tick_count: 0,
        tick_ms,
        last_tick: std::time::Instant::now(),
        event_tx: rt.event_tx,
        broadcast_tx: rt.broadcast_tx,
        shared_tick: rt.shared_tick,
    };

    let cam_state = CameraState {
        focus: Vec3::ZERO,
        zoom: 9.0,
        ..default()
    };

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Agentverse".into(),
                    resolution: bevy::window::WindowResolution::new(1400.0, 900.0),
                    ..default()
                }),
                ..default()
            })
            .disable::<bevy::log::LogPlugin>(),
    );

    // Resources
    app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 2048 });
    app.insert_resource(ClearColor(Color::srgb(0.12, 0.10, 0.09)));
    app.insert_resource(WorldBridge { grid, registry });
    app.insert_resource(cam_state);
    app.insert_resource(SyncState::default());
    app.insert_resource(SelectedAgent::default());
    app.insert_resource(super::overlay::MessageInputState::default());
    app.insert_resource(sim_state);

    // Startup systems (run once, chained so materials/meshes exist before camera)
    app.add_systems(
        Startup,
        (
            super::materials::setup_materials,
            super::meshes::setup_meshes,
            super::lighting::setup_lighting,
            super::camera::setup_camera,
            super::overlay::setup_ui,
        )
            .chain(),
    );

    // World tile spawning (runs once after resources are ready)
    app.add_systems(
        Update,
        super::sync::spawn_tiles.run_if(resource_exists::<super::materials::MaterialLib>),
    );

    // Per-frame systems: simulation tick runs first, then sync, then everything else
    app.add_systems(
        Update,
        (
            super::sim_system::sim_tick,
            super::sync::sync_agents,
            super::camera::camera_rotate,
            super::camera::camera_zoom,
            super::camera::camera_pan,
            super::interaction::handle_selection,
            super::interaction::click_select_agent,
            super::overlay::update_agent_labels,
            super::overlay::update_sidebar,
            super::overlay::update_status_bar,
            super::overlay::toggle_sidebar,
            super::overlay::handle_message_input,
        )
            .chain()
            .run_if(resource_exists::<super::materials::MaterialLib>),
    );

    app.run();

    Ok(())
}
