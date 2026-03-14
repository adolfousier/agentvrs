use crate::agent::AgentId;
use crate::world::pathfind::find_path;
use crate::world::*;

// ─── Position ───────────────────────────────────────────────

#[test]
fn test_position_new() {
    let pos = Position::new(5, 10);
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 10);
}

#[test]
fn test_position_move_up() {
    let pos = Position::new(5, 5);
    let moved = pos.moved(Direction::Up, (10, 10));
    assert_eq!(moved, Position::new(5, 4));
}

#[test]
fn test_position_move_down() {
    let pos = Position::new(5, 5);
    let moved = pos.moved(Direction::Down, (10, 10));
    assert_eq!(moved, Position::new(5, 6));
}

#[test]
fn test_position_move_left() {
    let pos = Position::new(5, 5);
    let moved = pos.moved(Direction::Left, (10, 10));
    assert_eq!(moved, Position::new(4, 5));
}

#[test]
fn test_position_move_right() {
    let pos = Position::new(5, 5);
    let moved = pos.moved(Direction::Right, (10, 10));
    assert_eq!(moved, Position::new(6, 5));
}

#[test]
fn test_position_move_up_at_boundary() {
    let pos = Position::new(5, 0);
    let moved = pos.moved(Direction::Up, (10, 10));
    assert_eq!(moved, Position::new(5, 0));
}

#[test]
fn test_position_move_left_at_boundary() {
    let pos = Position::new(0, 5);
    let moved = pos.moved(Direction::Left, (10, 10));
    assert_eq!(moved, Position::new(0, 5));
}

#[test]
fn test_position_move_down_at_boundary() {
    let pos = Position::new(5, 9);
    let moved = pos.moved(Direction::Down, (10, 10));
    assert_eq!(moved, Position::new(5, 9));
}

#[test]
fn test_position_move_right_at_boundary() {
    let pos = Position::new(9, 5);
    let moved = pos.moved(Direction::Right, (10, 10));
    assert_eq!(moved, Position::new(9, 5));
}

#[test]
fn test_position_distance() {
    let a = Position::new(0, 0);
    let b = Position::new(3, 4);
    assert!((a.distance_to(&b) - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_position_distance_same() {
    let a = Position::new(5, 5);
    assert!((a.distance_to(&a) - 0.0).abs() < f64::EPSILON);
}

// ─── Grid Basics ────────────────────────────────────────────

#[test]
fn test_grid_new() {
    let grid = Grid::new(10, 8);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 8);
}

#[test]
fn test_grid_with_walls() {
    let grid = Grid::with_walls(10, 8);
    assert_eq!(
        grid.get(Position::new(0, 0)).unwrap().tile,
        Tile::Wall(WallKind::Solid)
    );
    assert_eq!(
        grid.get(Position::new(9, 0)).unwrap().tile,
        Tile::Wall(WallKind::Solid)
    );
    assert_eq!(
        grid.get(Position::new(0, 7)).unwrap().tile,
        Tile::Wall(WallKind::Solid)
    );
    assert_eq!(
        grid.get(Position::new(9, 7)).unwrap().tile,
        Tile::Wall(WallKind::Solid)
    );
    assert_eq!(
        grid.get(Position::new(5, 4)).unwrap().tile,
        Tile::Floor(FloorKind::Wood)
    );
}

#[test]
fn test_grid_get_out_of_bounds() {
    let grid = Grid::new(10, 8);
    assert!(grid.get(Position::new(10, 0)).is_none());
    assert!(grid.get(Position::new(0, 8)).is_none());
    assert!(grid.get(Position::new(100, 100)).is_none());
}

#[test]
fn test_grid_place_agent() {
    let mut grid = Grid::new(10, 8);
    let id = AgentId::new();
    assert!(grid.place_agent(Position::new(5, 4), id));
    assert_eq!(grid.get(Position::new(5, 4)).unwrap().occupant, Some(id));
}

#[test]
fn test_grid_place_agent_on_wall() {
    let mut grid = Grid::with_walls(10, 8);
    let id = AgentId::new();
    assert!(!grid.place_agent(Position::new(0, 0), id));
}

#[test]
fn test_grid_place_agent_on_occupied() {
    let mut grid = Grid::new(10, 8);
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    assert!(grid.place_agent(Position::new(5, 4), id1));
    assert!(!grid.place_agent(Position::new(5, 4), id2));
}

#[test]
fn test_grid_remove_agent() {
    let mut grid = Grid::new(10, 8);
    let id = AgentId::new();
    grid.place_agent(Position::new(5, 4), id);
    let removed = grid.remove_agent(Position::new(5, 4));
    assert_eq!(removed, Some(id));
    assert!(grid.get(Position::new(5, 4)).unwrap().occupant.is_none());
}

#[test]
fn test_grid_remove_agent_empty() {
    let mut grid = Grid::new(10, 8);
    assert!(grid.remove_agent(Position::new(5, 4)).is_none());
}

#[test]
fn test_grid_move_agent() {
    let mut grid = Grid::new(10, 8);
    let id = AgentId::new();
    grid.place_agent(Position::new(5, 4), id);
    assert!(grid.move_agent(Position::new(5, 4), Position::new(6, 4)));
    assert!(grid.get(Position::new(5, 4)).unwrap().occupant.is_none());
    assert_eq!(grid.get(Position::new(6, 4)).unwrap().occupant, Some(id));
}

#[test]
fn test_grid_move_agent_to_wall() {
    let mut grid = Grid::with_walls(10, 8);
    let id = AgentId::new();
    grid.place_agent(Position::new(1, 1), id);
    assert!(!grid.move_agent(Position::new(1, 1), Position::new(0, 0)));
}

#[test]
fn test_grid_move_agent_same_position() {
    let mut grid = Grid::new(10, 8);
    let id = AgentId::new();
    grid.place_agent(Position::new(5, 4), id);
    assert!(grid.move_agent(Position::new(5, 4), Position::new(5, 4)));
}

#[test]
fn test_grid_move_agent_to_occupied() {
    let mut grid = Grid::new(10, 8);
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    grid.place_agent(Position::new(5, 4), id1);
    grid.place_agent(Position::new(6, 4), id2);
    assert!(!grid.move_agent(Position::new(5, 4), Position::new(6, 4)));
}

#[test]
fn test_grid_find_empty_floor() {
    let grid = Grid::with_walls(10, 8);
    let pos = grid.find_empty_floor();
    assert!(pos.is_some());
    let pos = pos.unwrap();
    assert!(pos.x > 0 && pos.x < 9);
    assert!(pos.y > 0 && pos.y < 7);
}

#[test]
fn test_grid_bounds() {
    let grid = Grid::new(16, 12);
    assert_eq!(grid.bounds(), (16, 12));
}

#[test]
fn test_grid_cells_accessor() {
    let grid = Grid::new(4, 3);
    assert_eq!(grid.cells().len(), 12); // 4 * 3
}

// ─── Cell ───────────────────────────────────────────────────

#[test]
fn test_cell_floor_is_walkable() {
    let cell = Cell::floor(FloorKind::Wood);
    assert!(cell.is_walkable());
}

#[test]
fn test_cell_wall_not_walkable() {
    let cell = Cell::wall();
    assert!(!cell.is_walkable());
}

#[test]
fn test_cell_occupied_not_walkable() {
    let mut cell = Cell::floor(FloorKind::Wood);
    cell.occupant = Some(AgentId::new());
    assert!(!cell.is_walkable());
}

#[test]
fn test_direction_random_is_valid() {
    for _ in 0..100 {
        let _ = Direction::random();
    }
}

#[test]
fn test_set_tile() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(5, 4), Tile::Wall(WallKind::Solid));
    assert_eq!(
        grid.get(Position::new(5, 4)).unwrap().tile,
        Tile::Wall(WallKind::Solid)
    );
}

#[test]
fn test_set_tile_furniture() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(3, 3), Tile::Desk);
    assert_eq!(grid.get(Position::new(3, 3)).unwrap().tile, Tile::Desk);
}

// ─── Tile Types ─────────────────────────────────────────────

#[test]
fn test_tile_is_solid_floors() {
    assert!(!Tile::Floor(FloorKind::Wood).is_solid());
    assert!(!Tile::Floor(FloorKind::Tile).is_solid());
    assert!(!Tile::Floor(FloorKind::Carpet).is_solid());
    assert!(!Tile::Floor(FloorKind::Concrete).is_solid());
}

#[test]
fn test_tile_is_solid_walkable_specials() {
    assert!(!Tile::Rug.is_solid());
    assert!(!Tile::DoorOpen.is_solid());
}

#[test]
fn test_tile_is_solid_walls() {
    assert!(Tile::Wall(WallKind::Solid).is_solid());
    assert!(Tile::Wall(WallKind::Window).is_solid());
}

#[test]
fn test_tile_is_solid_furniture() {
    assert!(Tile::Desk.is_solid());
    assert!(Tile::VendingMachine.is_solid());
    assert!(Tile::CoffeeMachine.is_solid());
    assert!(Tile::Couch.is_solid());
    assert!(Tile::Plant.is_solid());
    assert!(Tile::PinballMachine.is_solid());
    assert!(Tile::GymTreadmill.is_solid());
    assert!(Tile::WeightBench.is_solid());
    assert!(Tile::YogaMat.is_solid());
    assert!(Tile::FloorLamp.is_solid());
    assert!(Tile::PingPongTableLeft.is_solid());
    assert!(Tile::PingPongTableRight.is_solid());
    assert!(Tile::SmallArmchair.is_solid());
    assert!(Tile::Whiteboard.is_solid());
    assert!(Tile::KitchenCounter.is_solid());
}

// ─── find_tiles ─────────────────────────────────────────────

#[test]
fn test_find_tiles_empty() {
    let grid = Grid::with_walls(10, 8);
    let desks = grid.find_tiles(&Tile::Desk);
    assert!(desks.is_empty());
}

#[test]
fn test_find_tiles_finds_placed() {
    let mut grid = Grid::with_walls(10, 8);
    grid.set_tile(Position::new(3, 3), Tile::Desk);
    grid.set_tile(Position::new(5, 5), Tile::Desk);
    let desks = grid.find_tiles(&Tile::Desk);
    assert_eq!(desks.len(), 2);
    assert!(desks.contains(&Position::new(3, 3)));
    assert!(desks.contains(&Position::new(5, 5)));
}

#[test]
fn test_find_tiles_distinguishes_types() {
    let mut grid = Grid::with_walls(10, 8);
    grid.set_tile(Position::new(3, 3), Tile::Desk);
    grid.set_tile(Position::new(5, 5), Tile::VendingMachine);
    let desks = grid.find_tiles(&Tile::Desk);
    assert_eq!(desks.len(), 1);
    let vending = grid.find_tiles(&Tile::VendingMachine);
    assert_eq!(vending.len(), 1);
}

// ─── find_adjacent_floor ────────────────────────────────────

#[test]
fn test_find_adjacent_floor() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(5, 4), Tile::Desk);
    let adj = grid.find_adjacent_floor(Position::new(5, 4));
    assert!(adj.is_some());
    let adj = adj.unwrap();
    // Should be one step away
    let dx = (adj.x as i32 - 5).unsigned_abs();
    let dy = (adj.y as i32 - 4).unsigned_abs();
    assert_eq!(dx + dy, 1);
}

#[test]
fn test_find_adjacent_floor_prefers_left_face() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(5, 4), Tile::Desk);
    let adj = grid.find_adjacent_floor(Position::new(5, 4));
    // Should prefer -x (position 4,4) for LEFT face detail
    assert_eq!(adj, Some(Position::new(4, 4)));
}

#[test]
fn test_find_adjacent_floor_avoiding() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(5, 4), Tile::Desk);
    let avoid = vec![Position::new(4, 4)]; // Block the preferred -x spot
    let adj = grid.find_adjacent_floor_avoiding(Position::new(5, 4), &avoid);
    assert!(adj.is_some());
    assert_ne!(adj.unwrap(), Position::new(4, 4)); // Should pick a different spot
}

#[test]
fn test_find_adjacent_floor_avoiding_all_fallback() {
    let mut grid = Grid::new(10, 8);
    grid.set_tile(Position::new(5, 4), Tile::Desk);
    // Avoid all candidates — should fallback to first walkable
    let avoid = vec![
        Position::new(4, 4),
        Position::new(5, 5),
        Position::new(6, 4),
        Position::new(5, 3),
    ];
    let adj = grid.find_adjacent_floor_avoiding(Position::new(5, 4), &avoid);
    assert!(adj.is_some()); // Fallback: picks any walkable
}

#[test]
fn test_find_adjacent_floor_surrounded_by_walls() {
    let mut grid = Grid::with_walls(3, 3);
    // Center is floor, surrounded by walls on all sides
    // Actually with_walls makes (0,0)-(2,0)-(0,2)-(2,2) walls
    // Set center furniture — adjacent positions are (0,1), (2,1), (1,0), (1,2) — all walls
    grid.set_tile(Position::new(1, 1), Tile::Desk);
    let adj = grid.find_adjacent_floor(Position::new(1, 1));
    assert!(adj.is_none());
}

// ─── Pathfinding ────────────────────────────────────────────

#[test]
fn test_pathfind_same_position() {
    let grid = Grid::new(10, 8);
    let path = find_path(&grid, Position::new(5, 4), Position::new(5, 4));
    assert_eq!(path, Some(Vec::new()));
}

#[test]
fn test_pathfind_adjacent() {
    let grid = Grid::new(10, 8);
    let path = find_path(&grid, Position::new(5, 4), Position::new(6, 4));
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], Position::new(6, 4));
}

#[test]
fn test_pathfind_straight_line() {
    let grid = Grid::new(10, 8);
    let path = find_path(&grid, Position::new(1, 1), Position::new(5, 1));
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 4); // 4 steps to go from x=1 to x=5
}

#[test]
fn test_pathfind_around_wall() {
    let mut grid = Grid::new(10, 8);
    // Put a wall blocking direct path
    grid.set_tile(Position::new(3, 2), Tile::Wall(WallKind::Solid));
    grid.set_tile(Position::new(3, 3), Tile::Wall(WallKind::Solid));
    grid.set_tile(Position::new(3, 4), Tile::Wall(WallKind::Solid));

    let path = find_path(&grid, Position::new(2, 3), Position::new(4, 3));
    assert!(path.is_some());
    let path = path.unwrap();
    // Must go around — path should be longer than 2
    assert!(path.len() > 2);
    // Path should not include any wall positions
    for pos in &path {
        assert!(!grid.get(*pos).unwrap().tile.is_solid());
    }
}

#[test]
fn test_pathfind_no_path() {
    let mut grid = Grid::new(5, 5);
    // Completely wall off the target
    for x in 0..5 {
        grid.set_tile(Position::new(x, 2), Tile::Wall(WallKind::Solid));
    }
    let path = find_path(&grid, Position::new(2, 0), Position::new(2, 4));
    assert!(path.is_none());
}

#[test]
fn test_pathfind_excludes_start() {
    let grid = Grid::new(10, 8);
    let path = find_path(&grid, Position::new(1, 1), Position::new(3, 1)).unwrap();
    // Path should NOT include start position
    assert!(!path.contains(&Position::new(1, 1)));
    // But should include end
    assert!(path.contains(&Position::new(3, 1)));
}

#[test]
fn test_pathfind_avoids_occupied_cells() {
    let mut grid = Grid::new(10, 8);
    // Place agent blocking direct path
    let blocker = AgentId::new();
    grid.place_agent(Position::new(3, 1), blocker);

    let path = find_path(&grid, Position::new(1, 1), Position::new(5, 1));
    assert!(path.is_some());
    let path = path.unwrap();
    // Should not go through (3,1)
    assert!(!path.contains(&Position::new(3, 1)));
}

#[test]
fn test_pathfind_target_occupied_still_works() {
    let mut grid = Grid::new(10, 8);
    // Target cell is occupied — pathfinding should still find a path
    // (target is allowed even if occupied)
    let occupant = AgentId::new();
    grid.place_agent(Position::new(5, 1), occupant);

    let path = find_path(&grid, Position::new(1, 1), Position::new(5, 1));
    assert!(path.is_some());
}

// ─── Office World Layout ────────────────────────────────────

#[test]
fn test_build_office_world() {
    let grid = build_office_world(28, 20);
    assert_eq!(grid.width, 28);
    assert_eq!(grid.height, 20);

    // Perimeter should be walls
    assert!(grid.get(Position::new(0, 0)).unwrap().tile.is_solid());
    assert!(grid.get(Position::new(27, 0)).unwrap().tile.is_solid());
    assert!(grid.get(Position::new(0, 19)).unwrap().tile.is_solid());
    assert!(grid.get(Position::new(27, 19)).unwrap().tile.is_solid());
}

#[test]
fn test_office_world_has_furniture() {
    let grid = build_office_world(28, 20);
    // Should have desks
    assert!(!grid.find_tiles(&Tile::Desk).is_empty());
    // Should have vending machines
    assert!(!grid.find_tiles(&Tile::VendingMachine).is_empty());
    // Should have coffee machine
    assert!(!grid.find_tiles(&Tile::CoffeeMachine).is_empty());
}

#[test]
fn test_office_world_has_walkable_space() {
    let grid = build_office_world(28, 20);
    // Should be able to find empty floor
    let floor = grid.find_empty_floor();
    assert!(floor.is_some());
}

#[test]
fn test_office_world_has_ping_pong() {
    let grid = build_office_world(28, 20);
    let left = grid.find_tiles(&Tile::PingPongTableLeft);
    let right = grid.find_tiles(&Tile::PingPongTableRight);
    assert_eq!(left.len(), 1);
    assert_eq!(right.len(), 1);
    // Should be adjacent
    let l = left[0];
    let r = right[0];
    assert_eq!(l.y, r.y);
    assert_eq!(r.x - l.x, 1);
}

// ─── World Events Serialization ─────────────────────────────

#[test]
fn test_world_event_serialize_tick() {
    let event = WorldEvent::Tick { count: 42 };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("42"));
    assert!(json.contains("Tick"));
}

#[test]
fn test_world_event_serialize_agent_spawned() {
    let event = WorldEvent::AgentSpawned {
        agent_id: AgentId::new(),
        position: Position::new(5, 3),
    };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("AgentSpawned"));
}

#[test]
fn test_world_event_serialize_agent_moved() {
    let event = WorldEvent::AgentMoved {
        agent_id: AgentId::new(),
        from: Position::new(1, 1),
        to: Position::new(2, 1),
    };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("AgentMoved"));
}

#[test]
fn test_world_event_serialize_message_sent() {
    let event = WorldEvent::MessageSent {
        from: AgentId::new(),
        to: AgentId::new(),
        text: "hello".to_string(),
    };
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("MessageSent"));
    assert!(json.contains("hello"));
}
