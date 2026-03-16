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
                    resolution: bevy::window::WindowResolution::new(1400, 900),
                    ..default()
                }),
                ..default()
            })
            .disable::<bevy::log::LogPlugin>(),
    );

    // Detect system dark/light mode
    let is_dark = detect_system_dark_mode();

    // Resources
    app.insert_resource(ClearColor(theme_clear_color(is_dark)));
    app.insert_resource(ThemeState {
        is_dark,
        last_check: std::time::Instant::now(),
    });
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

    // Per-frame systems: simulation tick runs first, then sync, then everything else.
    // Split into two chained groups to stay under the 12-tuple limit.
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
        )
            .chain()
            .run_if(resource_exists::<super::materials::MaterialLib>),
    );
    app.add_systems(
        Update,
        (
            super::overlay::update_agent_labels,
            super::overlay::update_sidebar,
            super::overlay::update_status_bar,
            super::overlay::toggle_sidebar,
            super::overlay::handle_message_input,
            poll_system_theme,
            super::overlay::update_ui_theme,
        )
            .chain()
            .after(super::interaction::click_select_agent)
            .run_if(resource_exists::<super::materials::MaterialLib>),
    );

    app.run();

    Ok(())
}

/// Bevy resource tracking the current OS theme.
#[derive(Resource)]
pub struct ThemeState {
    pub is_dark: bool,
    pub last_check: std::time::Instant,
}

/// Returns the background clear color for the given theme.
fn theme_clear_color(is_dark: bool) -> Color {
    if is_dark {
        Color::srgb(0.12, 0.12, 0.14) // dark background
    } else {
        Color::srgb(0.85, 0.87, 0.90) // light background
    }
}

/// Polls OS dark/light mode every 2 seconds and updates ClearColor, lighting, and materials.
#[allow(clippy::too_many_arguments)]
fn poll_system_theme(
    mut theme: ResMut<ThemeState>,
    mut clear: ResMut<ClearColor>,
    mut ambient_q: Query<&mut AmbientLight>,
    mut lights: Query<&mut DirectionalLight>,
    mat_lib: Option<Res<super::materials::MaterialLib>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let now = std::time::Instant::now();
    if now.duration_since(theme.last_check).as_secs() < 2 {
        return;
    }
    theme.last_check = now;

    let is_dark = detect_system_dark_mode();
    if is_dark == theme.is_dark {
        return;
    }
    theme.is_dark = is_dark;

    // Update clear color
    clear.0 = theme_clear_color(is_dark);

    // Update ambient light
    if let Ok(mut ambient) = ambient_q.single_mut() {
        if is_dark {
            ambient.color = Color::srgb(0.85, 0.87, 0.95);
            ambient.brightness = 300.0;
        } else {
            ambient.color = Color::srgb(1.0, 0.98, 0.95);
            ambient.brightness = 800.0;
        }
    }

    // Update directional light
    for mut light in lights.iter_mut() {
        if is_dark {
            light.illuminance = 8000.0;
        } else {
            light.illuminance = 15000.0;
        }
    }

    // Update floor/wall material colors for theme
    if let Some(lib) = mat_lib {
        let tint = if is_dark { 0.65 } else { 1.0 };
        let updates: Vec<(Handle<StandardMaterial>, [f32; 3])> = vec![
            (lib.floor_tile.clone(), [0.88, 0.88, 0.90]),
            (lib.floor_wood.clone(), [0.78, 0.62, 0.42]),
            (lib.floor_carpet.clone(), [0.42, 0.38, 0.55]),
            (lib.floor_concrete.clone(), [0.55, 0.55, 0.55]),
            (lib.wall_solid.clone(), [0.50, 0.48, 0.45]),
            (lib.wall_window.clone(), [0.50, 0.48, 0.45]),
            (lib.whiteboard_surface.clone(), [0.95, 0.95, 0.95]),
            (lib.kitchen_top.clone(), [0.85, 0.85, 0.87]),
        ];
        for (handle, [r, g, b]) in updates {
            if let Some(m) = mats.get_mut(&handle) {
                m.base_color = Color::srgb(r * tint, g * tint, b * tint);
            }
        }
    }
}

/// Detect system dark/light mode preference.
/// Returns true for dark mode (default), false for light mode.
fn detect_system_dark_mode() -> bool {
    #[cfg(target_os = "macos")]
    {
        // macOS: `defaults read -g AppleInterfaceStyle` returns "Dark" in dark mode
        std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(true)
    }
    #[cfg(target_os = "windows")]
    {
        // Windows: registry key AppsUseLightTheme = 0 means dark mode
        std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
            .map(|o| {
                let out = String::from_utf8_lossy(&o.stdout);
                out.contains("0x0")
            })
            .unwrap_or(true)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        // Linux/other: default to dark
        true
    }
}
