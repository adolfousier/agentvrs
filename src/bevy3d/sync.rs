use crate::agent::AgentId;
use crate::bevy3d::agents::AgentMarker;
use crate::bevy3d::bridge::WorldBridge;
use crate::bevy3d::materials::MaterialLib;
use crate::bevy3d::meshes::MeshLib;
use crate::world::{Position, Tile, WallKind};
use bevy::prelude::*;
use std::collections::HashMap;

/// Marker for a tile entity in the world.
#[derive(Component)]
pub struct TileMarker;

/// Tracks which tiles have been spawned so we only create them once.
#[derive(Resource, Default)]
pub struct SyncState {
    pub tiles_spawned: bool,
    pub tile_entities: HashMap<(u16, u16), Entity>,
    pub agent_entities: HashMap<AgentId, Entity>,
}

/// One-shot system: spawn all tile entities from the grid on startup.
pub fn spawn_tiles(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    mat_lib: Res<MaterialLib>,
    mesh_lib: Res<MeshLib>,
    mut sync: ResMut<SyncState>,
) {
    if sync.tiles_spawned {
        return;
    }

    let grid = bridge.grid.read().unwrap();
    let (w, h) = grid.bounds();

    // Center offset so the world is centered at origin
    let cx = w as f32 / 2.0;
    let cz = h as f32 / 2.0;

    for gy in 0..h {
        for gx in 0..w {
            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                let world_x = gx as f32 - cx;
                let world_z = gy as f32 - cz;

                let entity = spawn_tile_entity(
                    &mut commands,
                    &cell.tile,
                    world_x,
                    world_z,
                    &mat_lib,
                    &mesh_lib,
                    gx,
                    gy,
                );

                if let Some(e) = entity {
                    sync.tile_entities.insert((gx, gy), e);
                }
            }
        }
    }

    sync.tiles_spawned = true;
}

#[allow(clippy::too_many_arguments)]
fn spawn_tile_entity(
    commands: &mut Commands,
    tile: &Tile,
    x: f32,
    z: f32,
    mat_lib: &MaterialLib,
    mesh_lib: &MeshLib,
    _gx: u16,
    _gy: u16,
) -> Option<Entity> {
    match tile {
        Tile::Floor(kind) => {
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.floor_quad.clone()),
                    MeshMaterial3d(mat_lib.floor_material(kind)),
                    Transform::from_xyz(x, 0.0, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::Wall(kind) => {
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.wall_box.clone()),
                    MeshMaterial3d(mat_lib.wall_material(kind)),
                    Transform::from_xyz(x, 0.3, z),
                    TileMarker,
                ))
                .id();

            // Glass pane for windows
            if matches!(kind, WallKind::Window) {
                commands.spawn((
                    Mesh3d(mesh_lib.wall_window_glass.clone()),
                    MeshMaterial3d(mat_lib.wall_window_glass.clone()),
                    Transform::from_xyz(x, 0.35, z),
                ));
            }

            Some(entity)
        }
        Tile::DoorOpen => {
            // Just a floor tile for open doors
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.floor_quad.clone()),
                    MeshMaterial3d(mat_lib.floor_wood.clone()),
                    Transform::from_xyz(x, 0.0, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::Rug => {
            // Floor underneath
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_wood.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Rug slightly above
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.rug.clone()),
                    MeshMaterial3d(mat_lib.rug.clone()),
                    Transform::from_xyz(x, 0.003, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::Desk => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_wood.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Desk top
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.desk_top.clone()),
                    MeshMaterial3d(mat_lib.desk_wood.clone()),
                    Transform::from_xyz(x, 0.30, z),
                    TileMarker,
                ))
                .id();
            // Legs
            for (lx, lz) in &[(-0.28, -0.18), (0.28, -0.18), (-0.28, 0.18), (0.28, 0.18)] {
                commands.spawn((
                    Mesh3d(mesh_lib.desk_leg.clone()),
                    MeshMaterial3d(mat_lib.desk_leg.clone()),
                    Transform::from_xyz(x + lx, 0.14, z + lz),
                ));
            }
            // Monitor at back of desk (-z, away from camera), rotated to face +z
            let mon_rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            commands.spawn((
                Mesh3d(mesh_lib.monitor_screen.clone()),
                MeshMaterial3d(mat_lib.monitor_screen.clone()),
                Transform::from_xyz(x, 0.48, z - 0.18).with_rotation(mon_rot),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.monitor_base.clone()),
                MeshMaterial3d(mat_lib.monitor_body.clone()),
                Transform::from_xyz(x, 0.38, z - 0.18).with_rotation(mon_rot),
            ));
            // Keyboard on desk surface (toward camera, +z side)
            commands.spawn((
                Mesh3d(mesh_lib.keyboard.clone()),
                MeshMaterial3d(mat_lib.desk_leg.clone()),
                Transform::from_xyz(x, 0.32, z + 0.05),
            ));
            Some(entity)
        }
        Tile::Couch => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_carpet.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Seat
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.couch_seat.clone()),
                    MeshMaterial3d(mat_lib.couch_fabric.clone()),
                    Transform::from_xyz(x, 0.10, z),
                    TileMarker,
                ))
                .id();
            // Back
            commands.spawn((
                Mesh3d(mesh_lib.couch_back.clone()),
                MeshMaterial3d(mat_lib.couch_cushion.clone()),
                Transform::from_xyz(x, 0.25, z - 0.22),
            ));
            // Arms
            commands.spawn((
                Mesh3d(mesh_lib.couch_arm.clone()),
                MeshMaterial3d(mat_lib.couch_fabric.clone()),
                Transform::from_xyz(x - 0.40, 0.18, z),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.couch_arm.clone()),
                MeshMaterial3d(mat_lib.couch_fabric.clone()),
                Transform::from_xyz(x + 0.40, 0.18, z),
            ));
            Some(entity)
        }
        Tile::Plant => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_wood.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Pot
            commands.spawn((
                Mesh3d(mesh_lib.plant_pot.clone()),
                MeshMaterial3d(mat_lib.plant_pot.clone()),
                Transform::from_xyz(x, 0.07, z),
            ));
            // Foliage sphere
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.plant_leaf_sphere.clone()),
                    MeshMaterial3d(mat_lib.plant_leaves.clone()),
                    Transform::from_xyz(x, 0.30, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::VendingMachine => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_tile.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Body (rotated 90° so front faces +z toward camera)
            let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.vending_body.clone()),
                    MeshMaterial3d(mat_lib.vending_body.clone()),
                    Transform::from_xyz(x, 0.40, z).with_rotation(rot),
                    TileMarker,
                ))
                .id();
            // Glass front (faces +z toward camera)
            commands.spawn((
                Mesh3d(mesh_lib.vending_glass.clone()),
                MeshMaterial3d(mat_lib.vending_glass.clone()),
                Transform::from_xyz(x, 0.45, z + 0.31),
            ));
            // Shelves (3 rows)
            for row in 0..3 {
                let shelf_y = 0.25 + row as f32 * 0.18;
                commands.spawn((
                    Mesh3d(mesh_lib.vending_shelf.clone()),
                    MeshMaterial3d(mat_lib.vending_shelf.clone()),
                    Transform::from_xyz(x, shelf_y, z).with_rotation(rot),
                ));
                // Cans on each shelf (4 per row)
                let can_colors = [
                    &mat_lib.vending_can_red,
                    &mat_lib.vending_can_blue,
                    &mat_lib.vending_can_green,
                    &mat_lib.vending_can_yellow,
                ];
                for col in 0..4 {
                    let cx = x - 0.10 + col as f32 * 0.07;
                    commands.spawn((
                        Mesh3d(mesh_lib.vending_can.clone()),
                        MeshMaterial3d(can_colors[(row + col) % 4].clone()),
                        Transform::from_xyz(cx, shelf_y + 0.04, z),
                    ));
                }
            }
            // Dispensing slot at bottom (faces +z)
            commands.spawn((
                Mesh3d(mesh_lib.vending_slot.clone()),
                MeshMaterial3d(mat_lib.vending_slot.clone()),
                Transform::from_xyz(x, 0.08, z + 0.21),
            ));
            // Price display (green LCD, faces +z)
            commands.spawn((
                Mesh3d(mesh_lib.vending_display.clone()),
                MeshMaterial3d(mat_lib.vending_display.clone()),
                Transform::from_xyz(x, 0.72, z + 0.31),
            ));
            Some(entity)
        }
        Tile::CoffeeMachine => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_tile.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Counter
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.coffee_counter.clone()),
                    MeshMaterial3d(mat_lib.coffee_counter.clone()),
                    Transform::from_xyz(x, 0.175, z),
                    TileMarker,
                ))
                .id();
            // Machine on top
            commands.spawn((
                Mesh3d(mesh_lib.coffee_machine_box.clone()),
                MeshMaterial3d(mat_lib.coffee_machine.clone()),
                Transform::from_xyz(x + 0.1, 0.50, z),
            ));
            Some(entity)
        }
        Tile::PinballMachine => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_concrete.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Cabinet body (rotated 90° so front faces +z toward camera)
            let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.arcade_cabinet.clone()),
                    MeshMaterial3d(mat_lib.arcade_body.clone()),
                    Transform::from_xyz(x, 0.375, z).with_rotation(rot),
                    TileMarker,
                ))
                .id();
            // Glowing marquee on top
            commands.spawn((
                Mesh3d(mesh_lib.arcade_marquee.clone()),
                MeshMaterial3d(mat_lib.arcade_marquee.clone()),
                Transform::from_xyz(x, 0.79, z).with_rotation(rot),
            ));
            // Screen (glowing green, faces +z toward camera)
            commands.spawn((
                Mesh3d(mesh_lib.arcade_screen.clone()),
                MeshMaterial3d(mat_lib.arcade_screen.clone()),
                Transform::from_xyz(x, 0.52, z + 0.26),
            ));
            // Control panel (angled below screen, faces +z)
            commands.spawn((
                Mesh3d(mesh_lib.arcade_panel.clone()),
                MeshMaterial3d(mat_lib.arcade_panel.clone()),
                Transform::from_xyz(x, 0.30, z + 0.26)
                    .with_rotation(Quat::from_rotation_x(0.3)),
            ));
            // Buttons on control panel (red, blue, green)
            for (i, btn_mat) in [
                &mat_lib.arcade_btn_red,
                &mat_lib.arcade_btn_blue,
                &mat_lib.arcade_btn_green,
            ]
            .iter()
            .enumerate()
            {
                let bx = x - 0.06 + i as f32 * 0.06;
                commands.spawn((
                    Mesh3d(mesh_lib.arcade_button.clone()),
                    MeshMaterial3d((*btn_mat).clone()),
                    Transform::from_xyz(bx, 0.32, z + 0.30),
                ));
            }
            // Coin slot (faces +z)
            commands.spawn((
                Mesh3d(mesh_lib.arcade_coin_slot.clone()),
                MeshMaterial3d(mat_lib.arcade_coin_slot.clone()),
                Transform::from_xyz(x, 0.15, z + 0.26),
            ));
            Some(entity)
        }
        Tile::GymTreadmill => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_concrete.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Base/belt
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.treadmill_base.clone()),
                    MeshMaterial3d(mat_lib.treadmill_belt.clone()),
                    Transform::from_xyz(x, 0.05, z),
                    TileMarker,
                ))
                .id();
            // Uprights
            commands.spawn((
                Mesh3d(mesh_lib.treadmill_upright.clone()),
                MeshMaterial3d(mat_lib.treadmill_frame.clone()),
                Transform::from_xyz(x - 0.20, 0.32, z + 0.35),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.treadmill_upright.clone()),
                MeshMaterial3d(mat_lib.treadmill_frame.clone()),
                Transform::from_xyz(x + 0.20, 0.32, z + 0.35),
            ));
            Some(entity)
        }
        Tile::WeightBench => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_concrete.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Bench
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.bench_base.clone()),
                    MeshMaterial3d(mat_lib.bench_pad.clone()),
                    Transform::from_xyz(x, 0.12, z),
                    TileMarker,
                ))
                .id();
            // Bar
            commands.spawn((
                Mesh3d(mesh_lib.bench_bar.clone()),
                MeshMaterial3d(mat_lib.bench_frame.clone()),
                Transform::from_xyz(x, 0.45, z)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ));
            Some(entity)
        }
        Tile::YogaMat => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_concrete.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Mat
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.yoga_mat.clone()),
                    MeshMaterial3d(mat_lib.yoga_mat.clone()),
                    Transform::from_xyz(x, 0.006, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::FloorLamp => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_wood.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Pole
            commands.spawn((
                Mesh3d(mesh_lib.lamp_pole.clone()),
                MeshMaterial3d(mat_lib.lamp_pole.clone()),
                Transform::from_xyz(x, 0.35, z),
            ));
            // Shade
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.lamp_shade.clone()),
                    MeshMaterial3d(mat_lib.lamp_shade.clone()),
                    Transform::from_xyz(x, 0.72, z),
                    TileMarker,
                ))
                .id();
            // Point light for warm glow
            commands.spawn((
                PointLight {
                    color: Color::srgb(1.0, 0.92, 0.65),
                    intensity: 800.0,
                    range: 3.0,
                    shadows_enabled: false,
                    ..default()
                },
                Transform::from_xyz(x, 0.68, z),
            ));
            Some(entity)
        }
        Tile::MeetingTable => {
            // Carpet floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_carpet.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Round table top
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.meeting_table_top.clone()),
                    MeshMaterial3d(mat_lib.meeting_table.clone()),
                    Transform::from_xyz(x, 0.30, z),
                    TileMarker,
                ))
                .id();
            // 4 table legs at diagonal positions
            for (lx, lz) in &[(0.18, 0.18), (-0.18, 0.18), (0.18, -0.18), (-0.18, -0.18)] {
                commands.spawn((
                    Mesh3d(mesh_lib.meeting_table_leg.clone()),
                    MeshMaterial3d(mat_lib.meeting_table.clone()),
                    Transform::from_xyz(x + lx, 0.14, z + lz),
                ));
            }
            // 4 chairs at cardinal directions
            let chair_offsets = [
                (0.45_f32, 0.0_f32, 0.0_f32),   // +x
                (-0.45, 0.0, std::f32::consts::PI), // -x
                (0.0, 0.45, std::f32::consts::FRAC_PI_2), // +z
                (0.0, -0.45, -std::f32::consts::FRAC_PI_2), // -z
            ];
            for (cx, cz, rot_y) in &chair_offsets {
                // Chair seat
                commands.spawn((
                    Mesh3d(mesh_lib.meeting_chair_seat.clone()),
                    MeshMaterial3d(mat_lib.meeting_chair.clone()),
                    Transform::from_xyz(x + cx, 0.18, z + cz),
                ));
                // Chair back (offset away from table center)
                let back_dist = 0.08;
                let back_x = cx + back_dist * rot_y.sin();
                let back_z = cz + back_dist * rot_y.cos();
                commands.spawn((
                    Mesh3d(mesh_lib.meeting_chair_back.clone()),
                    MeshMaterial3d(mat_lib.meeting_chair.clone()),
                    Transform::from_xyz(x + back_x, 0.27, z + back_z)
                        .with_rotation(Quat::from_rotation_y(*rot_y)),
                ));
            }
            Some(entity)
        }
        Tile::SmallArmchair => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_carpet.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Seat
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.armchair_seat.clone()),
                    MeshMaterial3d(mat_lib.armchair_fabric.clone()),
                    Transform::from_xyz(x, 0.10, z),
                    TileMarker,
                ))
                .id();
            // Back
            commands.spawn((
                Mesh3d(mesh_lib.armchair_back.clone()),
                MeshMaterial3d(mat_lib.armchair_fabric.clone()),
                Transform::from_xyz(x, 0.24, z - 0.20),
            ));
            // Arms
            commands.spawn((
                Mesh3d(mesh_lib.armchair_arm.clone()),
                MeshMaterial3d(mat_lib.armchair_fabric.clone()),
                Transform::from_xyz(x - 0.22, 0.16, z),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.armchair_arm.clone()),
                MeshMaterial3d(mat_lib.armchair_fabric.clone()),
                Transform::from_xyz(x + 0.22, 0.16, z),
            ));
            Some(entity)
        }
        Tile::Whiteboard => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_wood.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Legs
            commands.spawn((
                Mesh3d(mesh_lib.wb_leg.clone()),
                MeshMaterial3d(mat_lib.whiteboard_frame.clone()),
                Transform::from_xyz(x - 0.25, 0.275, z),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.wb_leg.clone()),
                MeshMaterial3d(mat_lib.whiteboard_frame.clone()),
                Transform::from_xyz(x + 0.25, 0.275, z),
            ));
            // Board
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.wb_board.clone()),
                    MeshMaterial3d(mat_lib.whiteboard_surface.clone()),
                    Transform::from_xyz(x, 0.45, z),
                    TileMarker,
                ))
                .id();
            Some(entity)
        }
        Tile::KitchenCounter => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_tile.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Counter body
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.kitchen_counter.clone()),
                    MeshMaterial3d(mat_lib.kitchen_counter.clone()),
                    Transform::from_xyz(x, 0.175, z),
                    TileMarker,
                ))
                .id();
            // Countertop
            commands.spawn((
                Mesh3d(mesh_lib.kitchen_top.clone()),
                MeshMaterial3d(mat_lib.kitchen_top.clone()),
                Transform::from_xyz(x, 0.365, z),
            ));
            Some(entity)
        }
    }
}

/// System: sync agent positions from the world bridge every frame.
pub fn sync_agents(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    mat_lib: Res<MaterialLib>,
    mesh_lib: Res<MeshLib>,
    mut sync: ResMut<SyncState>,
    mut agent_q: Query<&mut Transform, With<AgentMarker>>,
    time: Res<Time>,
) {
    // Simulation runs as a chained Bevy system (sim_tick → sync_agents),
    // so locks are never contended from the Bevy side.
    let (cx, cz, current_agents) = {
        let grid = bridge.grid.read().unwrap();
        let registry = bridge.registry.read().unwrap();
        let (w, h) = grid.bounds();
        let mut agents: HashMap<AgentId, (Position, u8, Option<Position>)> = HashMap::new();
        for agent in registry.agents() {
            let goal_target = agent.goal.as_ref().map(|g| g.target());
            agents.insert(agent.id, (agent.position, agent.color_index, goal_target));
        }
        (w as f32 / 2.0, h as f32 / 2.0, agents)
    };

    // Remove agents that no longer exist
    let stale: Vec<AgentId> = sync
        .agent_entities
        .keys()
        .filter(|id| !current_agents.contains_key(id))
        .copied()
        .collect();
    for id in stale {
        if let Some(entity) = sync.agent_entities.remove(&id) {
            commands.entity(entity).despawn_recursive();
        }
    }

    let t = time.elapsed_secs();

    // Update or spawn agents
    for (agent_id, (pos, color_index, goal_target)) in &current_agents {
        let world_x = pos.x as f32 - cx;
        let world_z = pos.y as f32 - cz;

        if let Some(&entity) = sync.agent_entities.get(agent_id) {
            if let Ok(mut transform) = agent_q.get_mut(entity) {
                let target = Vec3::new(world_x, 0.0, world_z);
                let old_xz = Vec2::new(transform.translation.x, transform.translation.z);
                let new_xz = Vec2::new(target.x, target.z);
                let dist = (old_xz - new_xz).length();

                // Smooth lerp toward target (faster to reduce sliding feel)
                let lerp_factor = if dist > 0.5 { 0.25 } else { 0.18 };
                let lerped = transform.translation.lerp(target, lerp_factor);

                // Walking animation when moving
                let (bob, tilt) = if dist > 0.02 {
                    let bounce = (t * 14.0).sin().abs() * 0.06;
                    let lean = 0.05;
                    (bounce, lean)
                } else {
                    (0.0, 0.0)
                };

                transform.translation = Vec3::new(lerped.x, bob, lerped.z);

                if dist > 0.02 {
                    // Face movement direction while walking
                    let dir = new_xz - old_xz;
                    let angle = dir.x.atan2(dir.y);
                    transform.rotation =
                        Quat::from_rotation_y(angle) * Quat::from_rotation_x(tilt);
                } else if let Some(furniture_pos) = goal_target {
                    // Stationary — face toward the furniture being used
                    let fx = furniture_pos.x as f32 - cx;
                    let fz = furniture_pos.y as f32 - cz;
                    let dx = fx - world_x;
                    let dz = fz - world_z;
                    if dx.abs() > 0.01 || dz.abs() > 0.01 {
                        let angle = dx.atan2(dz);
                        transform.rotation = Quat::from_rotation_y(angle);
                    }
                }
            }
        } else {
            let entity = super::agents::spawn_agent(
                &mut commands,
                &mesh_lib,
                &mat_lib,
                *agent_id,
                *color_index,
                world_x,
                world_z,
            );
            sync.agent_entities.insert(*agent_id, entity);
        }
    }
}
