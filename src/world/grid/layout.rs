use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build a Google-office style world scaled to fill the given dimensions.
pub fn build_office_world(w: u16, h: u16) -> Grid {
    let mut g = Grid::new(w, h);

    // Outer walls
    for x in 0..w {
        g.set_tile(Position::new(x, 0), Tile::Wall(WallKind::Window));
        g.set_tile(Position::new(x, h - 1), Tile::Wall(WallKind::Solid));
    }
    for y in 0..h {
        g.set_tile(Position::new(0, y), Tile::Wall(WallKind::Solid));
        g.set_tile(Position::new(w - 1, y), Tile::Wall(WallKind::Window));
    }

    let hall_y = h / 2;
    let div_x = w / 2;

    // Hallway
    for x in 1..w - 1 {
        g.set_tile(Position::new(x, hall_y), Tile::Floor(FloorKind::Concrete));
        if hall_y + 1 < h - 1 {
            g.set_tile(Position::new(x, hall_y + 1), Tile::Floor(FloorKind::Concrete));
        }
    }

    // Vertical dividers
    for y in 1..hall_y {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }
    for y in hall_y + 2..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors
    let door1 = 1 + (hall_y - 1) / 2;
    g.set_tile(Position::new(div_x, door1), Tile::DoorOpen);
    g.set_tile(Position::new(div_x, door1 + 1), Tile::DoorOpen);
    let door2 = hall_y + 2 + (h - hall_y - 3) / 2;
    g.set_tile(Position::new(div_x, door2), Tile::DoorOpen);
    if door2 + 1 < h - 1 {
        g.set_tile(Position::new(div_x, door2 + 1), Tile::DoorOpen);
    }

    office_area(&mut g, 1, 1, div_x, hall_y);
    break_room(&mut g, div_x + 1, 1, w - 1, hall_y);
    lounge(&mut g, 1, hall_y + 2, div_x, h - 1);
    gym_arcade(&mut g, div_x + 1, hall_y + 2, w - 1, h - 1);

    g
}

fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Desks in a grid with walking room
    let mut dy = y1 + 1;
    while dy < y2 - 1 {
        let mut dx = x1 + 2;
        while dx < x2 - 2 {
            g.set_tile(Position::new(dx, dy), Tile::Desk);
            dx += 4;
        }
        dy += 3;
    }
    // Whiteboard on left wall
    if y1 + 2 < y2 {
        g.set_tile(Position::new(x1, y1 + 2), Tile::Whiteboard);
    }
}

fn break_room(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }
    // Vending + coffee along top
    if x1 + 1 < x2 { g.set_tile(Position::new(x1 + 1, y1 + 1), Tile::VendingMachine); }
    if x1 + 4 < x2 { g.set_tile(Position::new(x1 + 4, y1 + 1), Tile::VendingMachine); }
    if x1 + 7 < x2 { g.set_tile(Position::new(x1 + 7, y1 + 1), Tile::CoffeeMachine); }
    // Couches
    let mid_y = y1 + (y2 - y1) / 2;
    if x1 + 2 < x2 { g.set_tile(Position::new(x1 + 2, mid_y), Tile::Couch); }
    if x1 + 5 < x2 { g.set_tile(Position::new(x1 + 5, mid_y), Tile::Couch); }
    // Plants
    if x2 - 2 > x1 && y2 - 2 > y1 { g.set_tile(Position::new(x2 - 2, y2 - 2), Tile::Plant); }
    if x1 + 2 < x2 && y2 - 2 > y1 { g.set_tile(Position::new(x1 + 2, y2 - 2), Tile::Plant); }
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }
    if x1 + 2 < x2 && y1 + 1 < y2 { g.set_tile(Position::new(x1 + 2, y1 + 1), Tile::Couch); }
    if x1 + 5 < x2 && y1 + 1 < y2 { g.set_tile(Position::new(x1 + 5, y1 + 1), Tile::Couch); }
    if x1 + 9 < x2 && y1 + 1 < y2 { g.set_tile(Position::new(x1 + 9, y1 + 1), Tile::Plant); }
    // Rug
    let cx = x1 + (x2 - x1) / 3;
    let cy = y1 + (y2 - y1) / 2;
    for dy in 0..2u16 {
        for dx in 0..3u16 {
            if cx + dx < x2 && cy + dy < y2 {
                g.set_tile(Position::new(cx + dx, cy + dy), Tile::Rug);
            }
        }
    }
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }
    if x1 + 2 < x2 && y1 + 1 < y2 { g.set_tile(Position::new(x1 + 2, y1 + 1), Tile::GymTreadmill); }
    if x1 + 6 < x2 && y1 + 1 < y2 { g.set_tile(Position::new(x1 + 6, y1 + 1), Tile::GymTreadmill); }
    let bot = y2.saturating_sub(2).max(y1 + 2);
    if x1 + 2 < x2 && bot < y2 { g.set_tile(Position::new(x1 + 2, bot), Tile::PinballMachine); }
    if x1 + 6 < x2 && bot < y2 { g.set_tile(Position::new(x1 + 6, bot), Tile::PinballMachine); }
    let mid = y1 + (y2 - y1) / 2;
    if x2 - 2 > x1 && mid < y2 { g.set_tile(Position::new(x2 - 2, mid), Tile::Plant); }
}
