use crate::agent::AgentId;
use crate::bevy3d::agents::AgentMarker;
use crate::bevy3d::camera::CameraState;
use bevy::prelude::*;

/// Currently selected agent.
#[derive(Resource, Default)]
pub struct SelectedAgent {
    pub agent_id: Option<AgentId>,
}

/// System: click to select agents, Escape to deselect.
pub fn handle_selection(keys: Res<ButtonInput<KeyCode>>, mut selected: ResMut<SelectedAgent>) {
    if keys.just_pressed(KeyCode::Escape) {
        selected.agent_id = None;
    }
}

/// System: click-to-select agents via raycasting.
pub fn click_select_agent(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    cam_state: Res<CameraState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<super::camera::MainCamera>>,
    agent_q: Query<(&GlobalTransform, &AgentMarker)>,
    mut selected: ResMut<SelectedAgent>,
) {
    // Only select on left-click release when it was a click (not a drag)
    if !mouse_btn.just_released(MouseButton::Left) || cam_state.dragged {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        return;
    };

    // Cast a ray from the cursor
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) else {
        return;
    };

    // Find closest agent to the ray
    let mut closest: Option<(AgentId, f32)> = None;
    for (agent_transform, marker) in agent_q.iter() {
        let agent_pos = agent_transform.translation();
        // Check distance from ray to agent center (with some height offset for the body center)
        let agent_center = agent_pos + Vec3::Y * 0.25;
        let to_agent = agent_center - ray.origin;
        let t = to_agent.dot(*ray.direction);
        if t < 0.0 {
            continue;
        }
        let closest_point = ray.origin + *ray.direction * t;
        let dist = (closest_point - agent_center).length();

        // Selection radius
        if dist < 0.35 && (closest.is_none() || t < closest.unwrap().1) {
            closest = Some((marker.agent_id, t));
        }
    }

    selected.agent_id = closest.map(|(id, _)| id);
}
