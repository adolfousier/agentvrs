use crate::agent::AgentId;
use crate::bevy3d::materials::MaterialLib;
use crate::bevy3d::meshes::MeshLib;
use bevy::prelude::*;

/// Marker component for agent root entities.
#[derive(Component)]
pub struct AgentMarker {
    pub agent_id: AgentId,
}

/// Spawn a low-poly humanoid agent at the given world position.
/// Returns the root entity ID.
pub fn spawn_agent(
    commands: &mut Commands,
    mesh_lib: &MeshLib,
    mat_lib: &MaterialLib,
    agent_id: AgentId,
    color_index: u8,
    x: f32,
    z: f32,
) -> Entity {
    let shirt_mat = mat_lib.shirt(color_index);
    let skin_mat = mat_lib.skin(color_index);
    let hair_mat = mat_lib.hair(color_index);
    let pants_mat = mat_lib.pants.clone();
    let shadow_mat = mat_lib.shadow.clone();

    // Spawn root entity (invisible transform anchor at ground level)
    let root = commands
        .spawn((
            Transform::from_xyz(x, 0.0, z),
            Visibility::default(),
            AgentMarker { agent_id },
        ))
        .with_children(|parent| {
            // Shadow disc on ground
            parent.spawn((
                Mesh3d(mesh_lib.agent_shadow.clone()),
                MeshMaterial3d(shadow_mat),
                Transform::from_xyz(0.0, 0.002, 0.0),
            ));

            // Left leg
            parent.spawn((
                Mesh3d(mesh_lib.agent_leg.clone()),
                MeshMaterial3d(pants_mat.clone()),
                Transform::from_xyz(-0.05, 0.06, 0.0),
            ));

            // Right leg
            parent.spawn((
                Mesh3d(mesh_lib.agent_leg.clone()),
                MeshMaterial3d(pants_mat),
                Transform::from_xyz(0.05, 0.06, 0.0),
            ));

            // Body/torso
            parent.spawn((
                Mesh3d(mesh_lib.agent_body.clone()),
                MeshMaterial3d(shirt_mat.clone()),
                Transform::from_xyz(0.0, 0.23, 0.0),
            ));

            // Left arm
            parent.spawn((
                Mesh3d(mesh_lib.agent_arm.clone()),
                MeshMaterial3d(shirt_mat.clone()),
                Transform::from_xyz(-0.12, 0.20, 0.0),
            ));

            // Right arm
            parent.spawn((
                Mesh3d(mesh_lib.agent_arm.clone()),
                MeshMaterial3d(shirt_mat),
                Transform::from_xyz(0.12, 0.20, 0.0),
            ));

            // Head
            parent.spawn((
                Mesh3d(mesh_lib.agent_head.clone()),
                MeshMaterial3d(skin_mat),
                Transform::from_xyz(0.0, 0.41, 0.0),
            ));

            // Left eye (white)
            parent.spawn((
                Mesh3d(mesh_lib.agent_eye_white.clone()),
                MeshMaterial3d(mat_lib.eye_white.clone()),
                Transform::from_xyz(-0.03, 0.43, 0.065),
            ));
            // Left pupil
            parent.spawn((
                Mesh3d(mesh_lib.agent_eye_pupil.clone()),
                MeshMaterial3d(mat_lib.eye_pupil.clone()),
                Transform::from_xyz(-0.03, 0.43, 0.07),
            ));
            // Right eye (white)
            parent.spawn((
                Mesh3d(mesh_lib.agent_eye_white.clone()),
                MeshMaterial3d(mat_lib.eye_white.clone()),
                Transform::from_xyz(0.03, 0.43, 0.065),
            ));
            // Right pupil
            parent.spawn((
                Mesh3d(mesh_lib.agent_eye_pupil.clone()),
                MeshMaterial3d(mat_lib.eye_pupil.clone()),
                Transform::from_xyz(0.03, 0.43, 0.07),
            ));

            // Hair
            parent.spawn((
                Mesh3d(mesh_lib.agent_hair.clone()),
                MeshMaterial3d(hair_mat),
                Transform::from_xyz(0.0, 0.505, 0.0),
            ));
        })
        .id();

    root
}
