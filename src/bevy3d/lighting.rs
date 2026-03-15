use bevy::prelude::*;

/// Set up scene lighting: directional sun + ambient fill.
pub fn setup_lighting(mut commands: Commands) {
    // Directional light (sun) — angled to cast shadows toward bottom-right
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.98, 0.95), // warm white
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4 * 1.2, // tilt down ~54 degrees
            std::f32::consts::FRAC_PI_4,         // 45 degrees around Y
            0.0,
        )),
    ));

    // Soft ambient fill
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.85, 0.87, 0.95), // cool blue-ish fill
        brightness: 300.0,
    });
}
