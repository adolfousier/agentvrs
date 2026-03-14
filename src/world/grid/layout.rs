use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build a Google-office style world with rooms and furniture.
pub fn build_office_world() -> Grid {
    let w: u16 = 40;
    let h: u16 = 22;
    let mut g = Grid::new(w, h);

    outer_walls(&mut g, w, h);
    office_area(&mut g);
    break_room(&mut g, w);
    hallway(&mut g, w);
    lounge(&mut g, h);
    gym_arcade(&mut g, w, h);

    g
}

fn outer_walls(g: &mut Grid, w: u16, h: u16) {
    for x in 0..w {
        g.set_tile(Position::new(x, 0), Tile::Wall(WallKind::Window));
        g.set_tile(Position::new(x, h - 1), Tile::Wall(WallKind::Solid));
    }
    for y in 0..h {
        g.set_tile(Position::new(0, y), Tile::Wall(WallKind::Solid));
        g.set_tile(Position::new(w - 1, y), Tile::Wall(WallKind::Window));
    }
}

fn office_area(g: &mut Grid) {
    for row in 0..3 {
        for col in 0..3 {
            g.set_tile(Position::new(3 + col * 4, 2 + row * 3), Tile::Desk);
        }
    }
    g.set_tile(Position::new(1, 3), Tile::Whiteboard);
}

fn break_room(g: &mut Grid, w: u16) {
    // Tile floor
    for y in 1..10 {
        for x in 22..w - 1 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }
    // Divider with door
    for y in 1..10 {
        g.set_tile(Position::new(21, y), Tile::Wall(WallKind::Solid));
    }
    g.set_tile(Position::new(21, 5), Tile::DoorOpen);
    g.set_tile(Position::new(21, 6), Tile::DoorOpen);

    // Furniture
    g.set_tile(Position::new(23, 2), Tile::VendingMachine);
    g.set_tile(Position::new(26, 2), Tile::VendingMachine);
    g.set_tile(Position::new(29, 2), Tile::CoffeeMachine);
    g.set_tile(Position::new(24, 5), Tile::Couch);
    g.set_tile(Position::new(27, 5), Tile::Couch);
    g.set_tile(Position::new(30, 7), Tile::Plant);
    g.set_tile(Position::new(24, 8), Tile::Plant);
}

fn hallway(g: &mut Grid, w: u16) {
    for x in 1..w - 1 {
        g.set_tile(Position::new(x, 10), Tile::Floor(FloorKind::Concrete));
        g.set_tile(Position::new(x, 11), Tile::Floor(FloorKind::Concrete));
    }
}

fn lounge(g: &mut Grid, h: u16) {
    for y in 12..h - 1 {
        for x in 1..20 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }
    g.set_tile(Position::new(3, 13), Tile::Couch);
    g.set_tile(Position::new(6, 13), Tile::Couch);
    g.set_tile(Position::new(10, 13), Tile::Plant);
    for dy in 0..2 {
        for dx in 0..3 {
            g.set_tile(Position::new(3 + dx, 16 + dy), Tile::Rug);
        }
    }
}

fn gym_arcade(g: &mut Grid, w: u16, h: u16) {
    for y in 12..h - 1 {
        for x in 22..w - 1 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }
    // Divider with door
    for y in 12..h - 1 {
        g.set_tile(Position::new(21, y), Tile::Wall(WallKind::Solid));
    }
    g.set_tile(Position::new(21, 14), Tile::DoorOpen);
    g.set_tile(Position::new(21, 15), Tile::DoorOpen);

    g.set_tile(Position::new(24, 13), Tile::GymTreadmill);
    g.set_tile(Position::new(28, 13), Tile::GymTreadmill);
    g.set_tile(Position::new(24, 17), Tile::PinballMachine);
    g.set_tile(Position::new(28, 17), Tile::PinballMachine);
    g.set_tile(Position::new(32, 15), Tile::Plant);
}
