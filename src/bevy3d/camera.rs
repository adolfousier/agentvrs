use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

/// Marker for our main 3D camera.
#[derive(Component)]
pub struct MainCamera;

/// Persistent camera state for rotation/zoom.
#[derive(Resource)]
pub struct CameraState {
    /// 0-3 rotation steps (each 90 degrees around Y).
    pub rotation_step: u8,
    /// Orthographic zoom (lower = more zoomed in).
    pub zoom: f32,
    /// Camera focus target (world center).
    pub focus: Vec3,
    /// Is the user panning?
    pub panning: bool,
    /// Was there significant drag movement? (distinguishes click from drag)
    pub dragged: bool,
    pub last_cursor: Option<Vec2>,
    pub drag_start: Option<Vec2>,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            rotation_step: 0,
            zoom: 12.0,
            focus: Vec3::ZERO,
            panning: false,
            dragged: false,
            last_cursor: None,
            drag_start: None,
        }
    }
}

/// Spawn an orthographic isometric camera.
pub fn setup_camera(mut commands: Commands, cam_state: Res<CameraState>) {
    let (pos, look_at) =
        camera_transform(cam_state.rotation_step, cam_state.zoom, cam_state.focus);

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: cam_state.zoom,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(pos).looking_at(look_at, Vec3::Y),
        MainCamera,
    ));
}

/// Compute camera position for a given rotation step.
/// True isometric: camera looks down at arctan(1/sqrt(2)) ≈ 35.264 degrees,
/// rotated 45+90*step degrees around Y.
fn camera_transform(_rotation_step: u8, _zoom: f32, focus: Vec3) -> (Vec3, Vec3) {
    let dist = 50.0;

    // Base angle: 45 degrees + 90 per rotation step
    let angle_y = std::f32::consts::FRAC_PI_4
        + std::f32::consts::FRAC_PI_2 * _rotation_step as f32;

    // Isometric tilt: arctan(sin(45°)) ≈ 35.264 degrees from horizontal
    // This gives the classic isometric 2:1 diamond ratio
    let tilt = (1.0_f32 / 2.0_f32.sqrt()).atan(); // ~0.6155 rad ≈ 35.26°

    // Camera offset from focus: spherical coordinates
    let horizontal_dist = dist * tilt.cos();
    let vertical_dist = dist * tilt.sin();

    let offset = Vec3::new(
        horizontal_dist * angle_y.sin(),
        vertical_dist,
        horizontal_dist * angle_y.cos(),
    );

    (focus + offset, focus)
}

/// System: handle camera rotation (R key rotates 90 degrees).
pub fn camera_rotate(
    keys: Res<ButtonInput<KeyCode>>,
    mut cam_state: ResMut<CameraState>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        cam_state.rotation_step = (cam_state.rotation_step + 1) % 4;
        update_camera_transform(&cam_state, &mut camera_q);
    }
}

/// System: handle camera zoom (scroll wheel).
pub fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cam_state: ResMut<CameraState>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    for ev in scroll_evr.read() {
        let delta = match ev.unit {
            MouseScrollUnit::Line => ev.y * 1.0,
            MouseScrollUnit::Pixel => ev.y * 0.05,
        };
        cam_state.zoom = (cam_state.zoom - delta).clamp(4.0, 40.0);
        for (_, mut proj) in camera_q.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *proj {
                ortho.scaling_mode = ScalingMode::FixedVertical {
                    viewport_height: cam_state.zoom,
                };
            }
        }
        update_camera_transform(&cam_state, &mut camera_q);
    }
}

/// System: handle camera pan (left-click drag, like GTK4 version).
pub fn camera_pan(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut cam_state: ResMut<CameraState>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    windows: Query<&Window>,
) {
    let panning =
        mouse_btn.pressed(MouseButton::Left) || mouse_btn.pressed(MouseButton::Middle);

    if panning {
        if let Ok(window) = windows.get_single()
            && let Some(cursor_pos) = window.cursor_position()
        {
            // Track drag start for click-vs-drag detection
            if cam_state.drag_start.is_none() {
                cam_state.drag_start = Some(cursor_pos);
                cam_state.dragged = false;
            }

            if let Some(last) = cam_state.last_cursor {
                let delta = cursor_pos - last;
                let pan_speed = cam_state.zoom * 0.003;
                let angle_y = std::f32::consts::FRAC_PI_4
                    + std::f32::consts::FRAC_PI_2 * cam_state.rotation_step as f32;

                cam_state.focus.x -=
                    (delta.x * angle_y.cos() + delta.y * angle_y.sin()) * pan_speed;
                cam_state.focus.z -=
                    (-delta.x * angle_y.sin() + delta.y * angle_y.cos()) * pan_speed;
                update_camera_transform(&cam_state, &mut camera_q);
            }
            cam_state.last_cursor = Some(cursor_pos);

            // Mark as dragged if moved more than 5px from start
            if let Some(start) = cam_state.drag_start {
                if (cursor_pos - start).length() > 5.0 {
                    cam_state.dragged = true;
                }
            }
        }
        cam_state.panning = true;
    } else {
        cam_state.panning = false;
        cam_state.last_cursor = None;
        cam_state.drag_start = None;
    }
}

fn update_camera_transform(
    cam_state: &CameraState,
    camera_q: &mut Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let (pos, look_at) =
        camera_transform(cam_state.rotation_step, cam_state.zoom, cam_state.focus);
    for (mut transform, _) in camera_q.iter_mut() {
        *transform = Transform::from_translation(pos).looking_at(look_at, Vec3::Y);
    }
}
