use crate::agent::AgentId;
use crate::world::*;

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
