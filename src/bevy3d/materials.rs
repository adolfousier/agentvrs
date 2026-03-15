use crate::world::{FloorKind, WallKind};
use bevy::prelude::*;

/// Pre-built material handles for all tile and agent types.
#[derive(Resource)]
pub struct MaterialLib {
    // Floors
    pub floor_wood: Handle<StandardMaterial>,
    pub floor_tile: Handle<StandardMaterial>,
    pub floor_carpet: Handle<StandardMaterial>,
    pub floor_concrete: Handle<StandardMaterial>,
    // Walls
    pub wall_solid: Handle<StandardMaterial>,
    pub wall_window: Handle<StandardMaterial>,
    pub wall_window_glass: Handle<StandardMaterial>,
    // Furniture
    pub desk_wood: Handle<StandardMaterial>,
    pub desk_leg: Handle<StandardMaterial>,
    pub monitor_body: Handle<StandardMaterial>,
    pub monitor_screen: Handle<StandardMaterial>,
    pub couch_fabric: Handle<StandardMaterial>,
    pub couch_cushion: Handle<StandardMaterial>,
    pub plant_pot: Handle<StandardMaterial>,
    pub plant_leaves: Handle<StandardMaterial>,
    pub vending_body: Handle<StandardMaterial>,
    pub vending_glass: Handle<StandardMaterial>,
    pub coffee_counter: Handle<StandardMaterial>,
    pub coffee_machine: Handle<StandardMaterial>,
    pub arcade_body: Handle<StandardMaterial>,
    pub arcade_screen: Handle<StandardMaterial>,
    pub treadmill_frame: Handle<StandardMaterial>,
    pub treadmill_belt: Handle<StandardMaterial>,
    pub bench_frame: Handle<StandardMaterial>,
    pub bench_pad: Handle<StandardMaterial>,
    pub yoga_mat: Handle<StandardMaterial>,
    pub lamp_pole: Handle<StandardMaterial>,
    pub lamp_shade: Handle<StandardMaterial>,
    pub meeting_table: Handle<StandardMaterial>,
    pub meeting_chair: Handle<StandardMaterial>,
    pub armchair_fabric: Handle<StandardMaterial>,
    pub whiteboard_frame: Handle<StandardMaterial>,
    pub whiteboard_surface: Handle<StandardMaterial>,
    pub rug: Handle<StandardMaterial>,
    pub kitchen_counter: Handle<StandardMaterial>,
    pub kitchen_top: Handle<StandardMaterial>,
    // Vending machine details
    pub vending_shelf: Handle<StandardMaterial>,
    pub vending_can_red: Handle<StandardMaterial>,
    pub vending_can_blue: Handle<StandardMaterial>,
    pub vending_can_green: Handle<StandardMaterial>,
    pub vending_can_yellow: Handle<StandardMaterial>,
    pub vending_slot: Handle<StandardMaterial>,
    pub vending_display: Handle<StandardMaterial>,
    // Arcade details
    pub arcade_marquee: Handle<StandardMaterial>,
    pub arcade_panel: Handle<StandardMaterial>,
    pub arcade_btn_red: Handle<StandardMaterial>,
    pub arcade_btn_blue: Handle<StandardMaterial>,
    pub arcade_btn_green: Handle<StandardMaterial>,
    pub arcade_coin_slot: Handle<StandardMaterial>,
    // Agents
    pub eye_white: Handle<StandardMaterial>,
    pub eye_pupil: Handle<StandardMaterial>,
    pub pants: Handle<StandardMaterial>,
    pub shadow: Handle<StandardMaterial>,
    // Shirt colors (indexed 0-7)
    pub shirts: [Handle<StandardMaterial>; 8],
    pub skins: [Handle<StandardMaterial>; 4],
    pub hairs: [Handle<StandardMaterial>; 4],
}

fn mat(mats: &mut Assets<StandardMaterial>, r: f32, g: f32, b: f32) -> Handle<StandardMaterial> {
    mats.add(StandardMaterial {
        base_color: Color::srgb(r, g, b),
        perceptual_roughness: 0.8,
        metallic: 0.0,
        ..default()
    })
}

fn mat_alpha(
    mats: &mut Assets<StandardMaterial>,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> Handle<StandardMaterial> {
    mats.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, a),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.5,
        metallic: 0.0,
        ..default()
    })
}

fn mat_emissive(
    mats: &mut Assets<StandardMaterial>,
    r: f32,
    g: f32,
    b: f32,
    emit: f32,
) -> Handle<StandardMaterial> {
    mats.add(StandardMaterial {
        base_color: Color::srgb(r, g, b),
        emissive: bevy::color::LinearRgba::new(r * emit, g * emit, b * emit, 1.0),
        perceptual_roughness: 0.3,
        metallic: 0.0,
        ..default()
    })
}

pub fn setup_materials(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    let lib = MaterialLib {
        // Floors
        floor_wood: mat(&mut mats, 0.78, 0.62, 0.42),
        floor_tile: mat(&mut mats, 0.88, 0.88, 0.90),
        floor_carpet: mat(&mut mats, 0.42, 0.38, 0.55),
        floor_concrete: mat(&mut mats, 0.55, 0.55, 0.55),
        // Walls
        wall_solid: mat(&mut mats, 0.50, 0.48, 0.45),
        wall_window: mat(&mut mats, 0.50, 0.48, 0.45),
        wall_window_glass: mat_alpha(&mut mats, 0.7, 0.85, 0.95, 0.4),
        // Furniture
        desk_wood: mat(&mut mats, 0.72, 0.55, 0.35),
        desk_leg: mat(&mut mats, 0.35, 0.35, 0.38),
        monitor_body: mat(&mut mats, 0.18, 0.18, 0.20),
        monitor_screen: mat_emissive(&mut mats, 0.15, 0.35, 0.55, 2.0),
        couch_fabric: mat(&mut mats, 0.25, 0.22, 0.45),
        couch_cushion: mat(&mut mats, 0.30, 0.27, 0.50),
        plant_pot: mat(&mut mats, 0.55, 0.35, 0.18),
        plant_leaves: mat(&mut mats, 0.20, 0.55, 0.20),
        vending_body: mat(&mut mats, 0.75, 0.15, 0.10),
        vending_glass: mat_alpha(&mut mats, 0.8, 0.9, 1.0, 0.3),
        coffee_counter: mat(&mut mats, 0.45, 0.30, 0.15),
        coffee_machine: mat(&mut mats, 0.30, 0.30, 0.32),
        arcade_body: mat(&mut mats, 0.40, 0.15, 0.55),
        arcade_screen: mat_emissive(&mut mats, 0.1, 0.4, 0.1, 3.0),
        treadmill_frame: mat(&mut mats, 0.40, 0.40, 0.42),
        treadmill_belt: mat(&mut mats, 0.15, 0.15, 0.15),
        bench_frame: mat(&mut mats, 0.45, 0.45, 0.48),
        bench_pad: mat(&mut mats, 0.20, 0.20, 0.22),
        yoga_mat: mat(&mut mats, 0.55, 0.30, 0.65),
        lamp_pole: mat(&mut mats, 0.60, 0.55, 0.40),
        lamp_shade: mat_emissive(&mut mats, 1.0, 0.92, 0.65, 1.5),
        meeting_table: mat(&mut mats, 0.70, 0.52, 0.32),
        meeting_chair: mat(&mut mats, 0.28, 0.28, 0.30),
        armchair_fabric: mat(&mut mats, 0.50, 0.18, 0.22),
        whiteboard_frame: mat(&mut mats, 0.40, 0.40, 0.42),
        whiteboard_surface: mat(&mut mats, 0.95, 0.95, 0.95),
        rug: mat(&mut mats, 0.55, 0.35, 0.25),
        kitchen_counter: mat(&mut mats, 0.40, 0.35, 0.30),
        kitchen_top: mat(&mut mats, 0.85, 0.85, 0.87),
        // Vending machine details
        vending_shelf: mat(&mut mats, 0.50, 0.50, 0.52),
        vending_can_red: mat(&mut mats, 0.90, 0.15, 0.10),
        vending_can_blue: mat(&mut mats, 0.15, 0.30, 0.85),
        vending_can_green: mat(&mut mats, 0.10, 0.70, 0.20),
        vending_can_yellow: mat(&mut mats, 0.90, 0.80, 0.10),
        vending_slot: mat(&mut mats, 0.10, 0.10, 0.10),
        vending_display: mat_emissive(&mut mats, 0.2, 0.8, 0.2, 3.0),
        // Arcade details
        arcade_marquee: mat_emissive(&mut mats, 1.0, 0.85, 0.0, 4.0),
        arcade_panel: mat(&mut mats, 0.15, 0.15, 0.18),
        arcade_btn_red: mat_emissive(&mut mats, 0.9, 0.1, 0.1, 2.0),
        arcade_btn_blue: mat_emissive(&mut mats, 0.1, 0.3, 0.9, 2.0),
        arcade_btn_green: mat_emissive(&mut mats, 0.1, 0.8, 0.2, 2.0),
        arcade_coin_slot: mat(&mut mats, 0.60, 0.55, 0.10),
        // Agents
        eye_white: mat(&mut mats, 0.95, 0.95, 0.95),
        eye_pupil: mat(&mut mats, 0.05, 0.05, 0.08),
        pants: mat(&mut mats, 0.20, 0.20, 0.35),
        shadow: mat_alpha(&mut mats, 0.0, 0.0, 0.0, 0.25),
        shirts: [
            mat(&mut mats, 0.26, 0.52, 0.96), // blue
            mat(&mut mats, 0.92, 0.26, 0.21), // red
            mat(&mut mats, 0.98, 0.74, 0.02), // yellow
            mat(&mut mats, 0.20, 0.66, 0.33), // green
            mat(&mut mats, 0.61, 0.35, 0.71), // purple
            mat(&mut mats, 0.90, 0.49, 0.13), // orange
            mat(&mut mats, 0.10, 0.74, 0.61), // teal
            mat(&mut mats, 0.95, 0.77, 0.06), // gold
        ],
        skins: [
            mat(&mut mats, 1.00, 0.85, 0.73), // light
            mat(&mut mats, 0.82, 0.67, 0.47), // medium
            mat(&mut mats, 0.63, 0.43, 0.27), // tan
            mat(&mut mats, 0.39, 0.27, 0.16), // dark
        ],
        hairs: [
            mat(&mut mats, 0.16, 0.12, 0.08), // black
            mat(&mut mats, 0.71, 0.47, 0.20), // brown
            mat(&mut mats, 0.78, 0.24, 0.12), // red
            mat(&mut mats, 0.24, 0.24, 0.24), // gray
        ],
    };

    commands.insert_resource(lib);
}

impl MaterialLib {
    pub fn floor_material(&self, kind: &FloorKind) -> Handle<StandardMaterial> {
        match kind {
            FloorKind::Wood => self.floor_wood.clone(),
            FloorKind::Tile => self.floor_tile.clone(),
            FloorKind::Carpet => self.floor_carpet.clone(),
            FloorKind::Concrete => self.floor_concrete.clone(),
        }
    }

    pub fn wall_material(&self, kind: &WallKind) -> Handle<StandardMaterial> {
        match kind {
            WallKind::Solid => self.wall_solid.clone(),
            WallKind::Window => self.wall_window.clone(),
        }
    }

    pub fn shirt(&self, index: u8) -> Handle<StandardMaterial> {
        self.shirts[(index % 8) as usize].clone()
    }

    pub fn skin(&self, index: u8) -> Handle<StandardMaterial> {
        self.skins[(index % 4) as usize].clone()
    }

    pub fn hair(&self, index: u8) -> Handle<StandardMaterial> {
        self.hairs[(index % 4) as usize].clone()
    }
}
