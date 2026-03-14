use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build a spacious office world with proper spatial design.
/// Uses generous spacing so furniture has breathing room and
/// nothing clips into walls.
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

    // Main hallway runs horizontally through the golden ratio point (~62%)
    let hall_y = (h as f64 * 0.58) as u16;
    let div_x = (w as f64 * 0.55) as u16;

    // Hallway (3 tiles wide for spaciousness)
    for x in 1..w - 1 {
        for dy in 0..3u16 {
            if hall_y + dy < h - 1 {
                g.set_tile(
                    Position::new(x, hall_y + dy),
                    Tile::Floor(FloorKind::Concrete),
                );
            }
        }
    }

    // Vertical divider wall
    for y in 1..hall_y {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }
    for y in hall_y + 3..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors (2-wide for each room, centered on divider)
    let door1_y = hall_y / 2;
    place_door(&mut g, div_x, door1_y);
    let door2_y = hall_y + 3 + (h - hall_y - 4) / 2;
    place_door(&mut g, div_x, door2_y);

    // Four rooms with generous interiors
    office_area(&mut g, 1, 1, div_x, hall_y);
    break_room(&mut g, div_x + 1, 1, w - 1, hall_y);
    lounge(&mut g, 1, hall_y + 3, div_x, h - 1);
    gym_arcade(&mut g, div_x + 1, hall_y + 3, w - 1, h - 1);

    g
}

fn place_door(g: &mut Grid, x: u16, y: u16) {
    g.set_tile(Position::new(x, y), Tile::DoorOpen);
    g.set_tile(Position::new(x, y + 1), Tile::DoorOpen);
}

fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let room_h = y2 - y1;

    // Desk grid: start 3 tiles from walls, space every 4 tiles horizontally, 4 vertically
    let start_x = x1 + 4;
    let start_y = y1 + 3;
    let mut dy = start_y;
    while dy < y2 - 3 {
        let mut dx = start_x;
        while dx < x2 - 3 {
            g.set_tile(Position::new(dx, dy), Tile::Desk);
            dx += 5; // 5-tile spacing between desks
        }
        dy += 4; // 4-tile row spacing
    }

    // Whiteboard — centered on left wall area, well inside
    let wb_y = y1 + room_h / 2;
    if x1 + 3 < x2 && wb_y < y2 - 2 {
        g.set_tile(Position::new(x1 + 3, wb_y), Tile::Whiteboard);
    }

    // Plant in far corner for visual balance
    if x2 - 4 > x1 && y2 - 3 > y1 {
        g.set_tile(Position::new(x2 - 4, y2 - 3), Tile::Plant);
    }
}

fn break_room(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let room_w = x2 - x1;
    let room_h = y2 - y1;

    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    // Vending machines along back wall — spaced apart, 4 tiles from top
    let vend_y = y1 + 4;
    if x1 + 3 < x2 {
        g.set_tile(Position::new(x1 + 3, vend_y), Tile::VendingMachine);
    }
    if x1 + 7 < x2 {
        g.set_tile(Position::new(x1 + 7, vend_y), Tile::VendingMachine);
    }

    // Coffee machine — offset from vending, different row for visual variety
    if x1 + 5 < x2 && vend_y + 3 < y2 {
        g.set_tile(Position::new(x1 + 5, vend_y + 3), Tile::CoffeeMachine);
    }

    // Couches — center of room, facing each other
    let mid_x = x1 + room_w / 2;
    let mid_y = y1 + room_h * 2 / 3;
    if mid_x - 2 > x1 && mid_y < y2 - 2 {
        g.set_tile(Position::new(mid_x - 2, mid_y), Tile::Couch);
    }
    if mid_x + 2 < x2 && mid_y < y2 - 2 {
        g.set_tile(Position::new(mid_x + 2, mid_y), Tile::Couch);
    }

    // Plants — corners for framing (diagonal placement)
    if x2 - 3 > x1 && y2 - 3 > y1 {
        g.set_tile(Position::new(x2 - 3, y2 - 3), Tile::Plant);
    }
    if x1 + 2 < x2 && y2 - 3 > y1 {
        g.set_tile(Position::new(x1 + 2, y2 - 3), Tile::Plant);
    }
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let room_w = x2 - x1;
    let room_h = y2 - y1;

    // Carpet floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }

    // Couches in an L-shape arrangement, well inside the room
    let cx = x1 + 4;
    let cy = y1 + 3;
    if cx < x2 && cy < y2 {
        g.set_tile(Position::new(cx, cy), Tile::Couch);
    }
    if cx + 4 < x2 && cy < y2 {
        g.set_tile(Position::new(cx + 4, cy), Tile::Couch);
    }
    if cx + 8 < x2 && cy < y2 {
        g.set_tile(Position::new(cx + 8, cy), Tile::Couch);
    }

    // Rug in center of room (golden ratio positioning)
    let rug_x = x1 + (room_w * 2 / 5);
    let rug_y = y1 + room_h / 2 + 1;
    for dy in 0..2u16 {
        for dx in 0..4u16 {
            if rug_x + dx < x2 - 2 && rug_y + dy < y2 - 2 {
                g.set_tile(Position::new(rug_x + dx, rug_y + dy), Tile::Rug);
            }
        }
    }

    // Plant accents — asymmetric placement
    if x1 + 3 < x2 && y2 - 3 > y1 {
        g.set_tile(Position::new(x1 + 3, y2 - 3), Tile::Plant);
    }
    if x2 - 4 > x1 && y1 + 2 < y2 {
        g.set_tile(Position::new(x2 - 4, y1 + 2), Tile::Plant);
    }
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let room_h = y2 - y1;

    // Concrete floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }

    // Treadmills — left side of room, spaced 5 apart
    let gym_y = y1 + 3;
    if x1 + 4 < x2 && gym_y < y2 - 2 {
        g.set_tile(Position::new(x1 + 4, gym_y), Tile::GymTreadmill);
    }
    if x1 + 9 < x2 && gym_y < y2 - 2 {
        g.set_tile(Position::new(x1 + 9, gym_y), Tile::GymTreadmill);
    }

    // Arcade machines — right/bottom area, spaced apart
    let arcade_y = y2 - 4;
    if x1 + 4 < x2 && arcade_y > y1 + 2 {
        g.set_tile(Position::new(x1 + 4, arcade_y), Tile::PinballMachine);
    }
    if x1 + 9 < x2 && arcade_y > y1 + 2 {
        g.set_tile(Position::new(x1 + 9, arcade_y), Tile::PinballMachine);
    }

    // Plant for visual softening
    let mid_y = y1 + room_h / 2;
    if x2 - 4 > x1 && mid_y < y2 {
        g.set_tile(Position::new(x2 - 4, mid_y), Tile::Plant);
    }
}
