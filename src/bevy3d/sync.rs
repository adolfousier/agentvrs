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
pub struct TileMarker {
    pub pos: Position,
}

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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
            // Monitor
            commands.spawn((
                Mesh3d(mesh_lib.monitor_screen.clone()),
                MeshMaterial3d(mat_lib.monitor_screen.clone()),
                Transform::from_xyz(x + 0.15, 0.48, z),
            ));
            commands.spawn((
                Mesh3d(mesh_lib.monitor_base.clone()),
                MeshMaterial3d(mat_lib.monitor_body.clone()),
                Transform::from_xyz(x + 0.15, 0.38, z),
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
            // Body
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.vending_body.clone()),
                    MeshMaterial3d(mat_lib.vending_body.clone()),
                    Transform::from_xyz(x, 0.40, z),
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
                ))
                .id();
            // Glass front
            commands.spawn((
                Mesh3d(mesh_lib.vending_glass.clone()),
                MeshMaterial3d(mat_lib.vending_glass.clone()),
                Transform::from_xyz(x - 0.31, 0.45, z),
            ));
            // Shelves (3 rows)
            for row in 0..3 {
                let shelf_y = 0.25 + row as f32 * 0.18;
                commands.spawn((
                    Mesh3d(mesh_lib.vending_shelf.clone()),
                    MeshMaterial3d(mat_lib.vending_shelf.clone()),
                    Transform::from_xyz(x, shelf_y, z),
                ));
                // Cans on each shelf (4 per row)
                let can_colors = [
                    &mat_lib.vending_can_red,
                    &mat_lib.vending_can_blue,
                    &mat_lib.vending_can_green,
                    &mat_lib.vending_can_yellow,
                ];
                for col in 0..4 {
                    let cz = z - 0.10 + col as f32 * 0.07;
                    commands.spawn((
                        Mesh3d(mesh_lib.vending_can.clone()),
                        MeshMaterial3d(can_colors[(row + col) % 4].clone()),
                        Transform::from_xyz(x, shelf_y + 0.04, cz),
                    ));
                }
            }
            // Dispensing slot at bottom
            commands.spawn((
                Mesh3d(mesh_lib.vending_slot.clone()),
                MeshMaterial3d(mat_lib.vending_slot.clone()),
                Transform::from_xyz(x - 0.21, 0.08, z),
            ));
            // Price display (green LCD)
            commands.spawn((
                Mesh3d(mesh_lib.vending_display.clone()),
                MeshMaterial3d(mat_lib.vending_display.clone()),
                Transform::from_xyz(x - 0.31, 0.72, z),
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
            // Cabinet body
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.arcade_cabinet.clone()),
                    MeshMaterial3d(mat_lib.arcade_body.clone()),
                    Transform::from_xyz(x, 0.375, z),
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
                ))
                .id();
            // Glowing marquee on top
            commands.spawn((
                Mesh3d(mesh_lib.arcade_marquee.clone()),
                MeshMaterial3d(mat_lib.arcade_marquee.clone()),
                Transform::from_xyz(x, 0.79, z),
            ));
            // Screen (glowing green)
            commands.spawn((
                Mesh3d(mesh_lib.arcade_screen.clone()),
                MeshMaterial3d(mat_lib.arcade_screen.clone()),
                Transform::from_xyz(x - 0.26, 0.52, z),
            ));
            // Control panel (angled below screen)
            commands.spawn((
                Mesh3d(mesh_lib.arcade_panel.clone()),
                MeshMaterial3d(mat_lib.arcade_panel.clone()),
                Transform::from_xyz(x - 0.26, 0.30, z)
                    .with_rotation(Quat::from_rotation_z(0.3)),
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
                let bz = z - 0.06 + i as f32 * 0.06;
                commands.spawn((
                    Mesh3d(mesh_lib.arcade_button.clone()),
                    MeshMaterial3d((*btn_mat).clone()),
                    Transform::from_xyz(x - 0.30, 0.32, bz),
                ));
            }
            // Coin slot
            commands.spawn((
                Mesh3d(mesh_lib.arcade_coin_slot.clone()),
                MeshMaterial3d(mat_lib.arcade_coin_slot.clone()),
                Transform::from_xyz(x - 0.26, 0.15, z),
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
        Tile::PingPongTableLeft => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_carpet.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Table half
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.table_half.clone()),
                    MeshMaterial3d(mat_lib.table_green.clone()),
                    Transform::from_xyz(x, 0.32, z - 0.05),
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
                ))
                .id();
            // Legs
            for (lx, lz) in &[(-0.30, -0.35), (0.30, -0.35), (-0.30, 0.20), (0.30, 0.20)] {
                commands.spawn((
                    Mesh3d(mesh_lib.table_leg.clone()),
                    MeshMaterial3d(mat_lib.whiteboard_frame.clone()),
                    Transform::from_xyz(x + lx, 0.15, z + lz),
                ));
            }
            // Net post
            commands.spawn((
                Mesh3d(mesh_lib.net_post.clone()),
                MeshMaterial3d(mat_lib.table_white.clone()),
                Transform::from_xyz(x, 0.38, z + 0.40),
            ));
            Some(entity)
        }
        Tile::PingPongTableRight => {
            // Floor
            commands.spawn((
                Mesh3d(mesh_lib.floor_quad.clone()),
                MeshMaterial3d(mat_lib.floor_carpet.clone()),
                Transform::from_xyz(x, 0.0, z),
            ));
            // Table half
            let entity = commands
                .spawn((
                    Mesh3d(mesh_lib.table_half.clone()),
                    MeshMaterial3d(mat_lib.table_green.clone()),
                    Transform::from_xyz(x, 0.32, z + 0.05),
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
                ))
                .id();
            // Legs
            for (lx, lz) in &[(-0.30, -0.20), (0.30, -0.20), (-0.30, 0.35), (0.30, 0.35)] {
                commands.spawn((
                    Mesh3d(mesh_lib.table_leg.clone()),
                    MeshMaterial3d(mat_lib.whiteboard_frame.clone()),
                    Transform::from_xyz(x + lx, 0.15, z + lz),
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
                    TileMarker {
                        pos: Position::new(_gx, _gy),
                    },
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
    let grid = bridge.grid.read().unwrap();
    let registry = bridge.registry.read().unwrap();
    let (w, h) = grid.bounds();
    let cx = w as f32 / 2.0;
    let cz = h as f32 / 2.0;

    // Collect current agent positions and data
    let mut current_agents: HashMap<AgentId, (Position, u8)> = HashMap::new();
    for agent in registry.agents() {
        current_agents.insert(agent.id, (agent.position, agent.color_index));
    }

    // Remove agents that no longer exist
    let stale: Vec<AgentId> = sync
        .agent_entities
        .keys()
        .filter(|id| !current_agents.contains_key(id))
        .copied()
        .collect();
    for id in stale {
        if let Some(entity) = sync.agent_entities.remove(&id) {
            commands.entity(entity).despawn();
        }
    }

    let t = time.elapsed_secs();

    // Update or spawn agents
    for (agent_id, (pos, color_index)) in &current_agents {
        let world_x = pos.x as f32 - cx;
        let world_z = pos.y as f32 - cz;

        if let Some(&entity) = sync.agent_entities.get(agent_id) {
            if let Ok(mut transform) = agent_q.get_mut(entity) {
                let target = Vec3::new(world_x, 0.0, world_z);
                let old_xz = Vec2::new(transform.translation.x, transform.translation.z);
                let new_xz = Vec2::new(target.x, target.z);
                let dist = (old_xz - new_xz).length();

                // Smooth lerp toward target
                let lerped = transform.translation.lerp(target, 0.12);

                // Walking bob: bounce up/down when moving
                let bob = if dist > 0.02 {
                    (t * 12.0).sin().abs() * 0.04
                } else {
                    0.0
                };

                transform.translation = Vec3::new(lerped.x, bob, lerped.z);

                // Face movement direction
                if dist > 0.02 {
                    let dir = new_xz - old_xz;
                    let angle = dir.x.atan2(dir.y);
                    transform.rotation = Quat::from_rotation_y(angle);
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
