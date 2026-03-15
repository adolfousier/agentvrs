use bevy::prelude::*;

/// Pre-built mesh handles for common primitive shapes.
#[derive(Resource)]
pub struct MeshLib {
    // Floors (flat quads)
    pub floor_quad: Handle<Mesh>,
    // Walls
    pub wall_box: Handle<Mesh>,
    pub wall_window_frame: Handle<Mesh>,
    pub wall_window_glass: Handle<Mesh>,
    // Desk parts
    pub desk_top: Handle<Mesh>,
    pub desk_leg: Handle<Mesh>,
    pub monitor_base: Handle<Mesh>,
    pub monitor_screen: Handle<Mesh>,
    pub keyboard: Handle<Mesh>,
    // Couch
    pub couch_seat: Handle<Mesh>,
    pub couch_back: Handle<Mesh>,
    pub couch_arm: Handle<Mesh>,
    // Plant
    pub plant_pot: Handle<Mesh>,
    pub plant_leaf_sphere: Handle<Mesh>,
    // Vending machine
    pub vending_body: Handle<Mesh>,
    pub vending_glass: Handle<Mesh>,
    // Coffee
    pub coffee_counter: Handle<Mesh>,
    pub coffee_machine_box: Handle<Mesh>,
    // Arcade
    pub arcade_cabinet: Handle<Mesh>,
    pub arcade_screen: Handle<Mesh>,
    // Treadmill
    pub treadmill_base: Handle<Mesh>,
    pub treadmill_upright: Handle<Mesh>,
    // Weight bench
    pub bench_base: Handle<Mesh>,
    pub bench_bar: Handle<Mesh>,
    // Yoga mat
    pub yoga_mat: Handle<Mesh>,
    // Floor lamp
    pub lamp_pole: Handle<Mesh>,
    pub lamp_shade: Handle<Mesh>,
    // Ping pong
    pub table_half: Handle<Mesh>,
    pub table_full: Handle<Mesh>,
    pub table_leg: Handle<Mesh>,
    pub net_post: Handle<Mesh>,
    pub net_mesh: Handle<Mesh>,
    // Armchair
    pub armchair_seat: Handle<Mesh>,
    pub armchair_back: Handle<Mesh>,
    pub armchair_arm: Handle<Mesh>,
    // Whiteboard
    pub wb_board: Handle<Mesh>,
    pub wb_leg: Handle<Mesh>,
    // Rug
    pub rug: Handle<Mesh>,
    // Kitchen
    pub kitchen_counter: Handle<Mesh>,
    pub kitchen_top: Handle<Mesh>,
    // Door
    pub door_frame: Handle<Mesh>,
    // Vending machine details
    pub vending_shelf: Handle<Mesh>,
    pub vending_can: Handle<Mesh>,
    pub vending_slot: Handle<Mesh>,
    pub vending_display: Handle<Mesh>,
    // Arcade details
    pub arcade_marquee: Handle<Mesh>,
    pub arcade_panel: Handle<Mesh>,
    pub arcade_button: Handle<Mesh>,
    pub arcade_coin_slot: Handle<Mesh>,
    // Agent eyes
    pub agent_eye_white: Handle<Mesh>,
    pub agent_eye_pupil: Handle<Mesh>,
    // Agent parts
    pub agent_leg: Handle<Mesh>,
    pub agent_body: Handle<Mesh>,
    pub agent_head: Handle<Mesh>,
    pub agent_hair: Handle<Mesh>,
    pub agent_arm: Handle<Mesh>,
    pub agent_shadow: Handle<Mesh>,
}

pub fn setup_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let lib = MeshLib {
        // Floor: flat plane 1x1
        floor_quad: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(0.5, 0.5))),

        // Wall: box 1 wide, 0.6 tall, 0.15 deep
        wall_box: meshes.add(Cuboid::new(1.0, 0.6, 0.15)),
        wall_window_frame: meshes.add(Cuboid::new(1.0, 0.6, 0.15)),
        wall_window_glass: meshes.add(Cuboid::new(0.7, 0.3, 0.05)),

        // Desk
        desk_top: meshes.add(Cuboid::new(0.7, 0.04, 0.45)),
        desk_leg: meshes.add(Cuboid::new(0.04, 0.28, 0.04)),
        monitor_base: meshes.add(Cuboid::new(0.02, 0.18, 0.14)),
        monitor_screen: meshes.add(Cuboid::new(0.01, 0.15, 0.22)),
        keyboard: meshes.add(Cuboid::new(0.18, 0.01, 0.07)),

        // Couch
        couch_seat: meshes.add(Cuboid::new(0.8, 0.15, 0.45)),
        couch_back: meshes.add(Cuboid::new(0.8, 0.25, 0.08)),
        couch_arm: meshes.add(Cuboid::new(0.08, 0.20, 0.45)),

        // Plant
        plant_pot: meshes.add(Cylinder::new(0.08, 0.14)),
        plant_leaf_sphere: meshes.add(Sphere::new(0.15)),

        // Vending machine
        vending_body: meshes.add(Cuboid::new(0.6, 0.8, 0.4)),
        vending_glass: meshes.add(Cuboid::new(0.45, 0.5, 0.02)),

        // Coffee
        coffee_counter: meshes.add(Cuboid::new(0.7, 0.35, 0.45)),
        coffee_machine_box: meshes.add(Cuboid::new(0.25, 0.30, 0.20)),

        // Arcade
        arcade_cabinet: meshes.add(Cuboid::new(0.5, 0.75, 0.55)),
        arcade_screen: meshes.add(Cuboid::new(0.35, 0.25, 0.02)),

        // Treadmill
        treadmill_base: meshes.add(Cuboid::new(0.5, 0.06, 0.8)),
        treadmill_upright: meshes.add(Cuboid::new(0.04, 0.55, 0.04)),

        // Weight bench
        bench_base: meshes.add(Cuboid::new(0.3, 0.20, 0.7)),
        bench_bar: meshes.add(Cylinder::new(0.015, 0.8)),

        // Yoga mat
        yoga_mat: meshes.add(Cuboid::new(0.4, 0.01, 0.8)),

        // Floor lamp
        lamp_pole: meshes.add(Cylinder::new(0.015, 0.7)),
        lamp_shade: meshes.add(Cylinder::new(0.10, 0.08)),

        // Ping pong table
        table_half: meshes.add(Cuboid::new(0.75, 0.04, 0.85)),
        table_full: meshes.add(Cuboid::new(1.80, 0.04, 0.85)),
        table_leg: meshes.add(Cuboid::new(0.04, 0.30, 0.04)),
        net_post: meshes.add(Cuboid::new(0.02, 0.08, 0.02)),
        net_mesh: meshes.add(Cuboid::new(0.01, 0.06, 0.80)),

        // Armchair
        armchair_seat: meshes.add(Cuboid::new(0.45, 0.12, 0.45)),
        armchair_back: meshes.add(Cuboid::new(0.45, 0.25, 0.06)),
        armchair_arm: meshes.add(Cuboid::new(0.06, 0.15, 0.45)),

        // Whiteboard
        wb_board: meshes.add(Cuboid::new(0.7, 0.5, 0.03)),
        wb_leg: meshes.add(Cuboid::new(0.03, 0.55, 0.03)),

        // Rug
        rug: meshes.add(Cuboid::new(0.85, 0.005, 0.85)),

        // Kitchen
        kitchen_counter: meshes.add(Cuboid::new(0.8, 0.35, 0.45)),
        kitchen_top: meshes.add(Cuboid::new(0.82, 0.03, 0.47)),

        // Door
        door_frame: meshes.add(Cuboid::new(0.08, 0.6, 0.6)),

        // Vending machine details
        vending_shelf: meshes.add(Cuboid::new(0.42, 0.01, 0.30)),
        vending_can: meshes.add(Cylinder::new(0.025, 0.06)),
        vending_slot: meshes.add(Cuboid::new(0.20, 0.08, 0.05)),
        vending_display: meshes.add(Cuboid::new(0.12, 0.05, 0.02)),
        // Arcade details
        arcade_marquee: meshes.add(Cuboid::new(0.48, 0.08, 0.20)),
        arcade_panel: meshes.add(Cuboid::new(0.40, 0.06, 0.25)),
        arcade_button: meshes.add(Cylinder::new(0.02, 0.015)),
        arcade_coin_slot: meshes.add(Cuboid::new(0.04, 0.06, 0.02)),
        // Agent eyes
        agent_eye_white: meshes.add(Cuboid::new(0.04, 0.035, 0.01)),
        agent_eye_pupil: meshes.add(Cuboid::new(0.02, 0.02, 0.005)),
        // Agent parts (all units relative to 1.0 = one grid cell)
        agent_leg: meshes.add(Cuboid::new(0.06, 0.12, 0.06)),
        agent_body: meshes.add(Cuboid::new(0.18, 0.22, 0.10)),
        agent_head: meshes.add(Cuboid::new(0.14, 0.14, 0.12)),
        agent_hair: meshes.add(Cuboid::new(0.15, 0.05, 0.13)),
        agent_arm: meshes.add(Cuboid::new(0.05, 0.17, 0.06)),
        agent_shadow: meshes.add(Cylinder::new(0.18, 0.005)),
    };

    commands.insert_resource(lib);
}
