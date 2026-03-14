use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build a Google-office style world scaled to fill the given dimensions.
pub fn build_office_world(w: u16, h: u16) -> Grid {
    let mut g = Grid::new(w, h);

    outer_walls(&mut g, w, h);

    // Horizontal split: hallway at the vertical midpoint
    let hall_y = h / 2;

    // Vertical split: divider between left and right rooms
    let div_x = w / 2;

    // Hallway (2 tiles tall through the middle)
    for x in 1..w - 1 {
        g.set_tile(Position::new(x, hall_y), Tile::Floor(FloorKind::Concrete));
        g.set_tile(
            Position::new(x, hall_y + 1),
            Tile::Floor(FloorKind::Concrete),
        );
    }

    // Room dividers (vertical walls with doors)
    for y in 1..hall_y {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }
    for y in hall_y + 2..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors in dividers
    let door_top = hall_y / 2;
    g.set_tile(Position::new(div_x, door_top), Tile::DoorOpen);
    g.set_tile(Position::new(div_x, door_top + 1), Tile::DoorOpen);

    let door_bot = hall_y + 2 + (h - hall_y - 3) / 2;
    g.set_tile(Position::new(div_x, door_bot), Tile::DoorOpen);
    g.set_tile(Position::new(div_x, door_bot + 1), Tile::DoorOpen);

    // Top-left: Office (wood floor + desks)
    office_area(&mut g, 1, 1, div_x, hall_y);

    // Top-right: Break room (tile floor + vending/coffee)
    break_room(&mut g, div_x + 1, 1, w - 1, hall_y);

    // Bottom-left: Lounge (carpet + couches + rug)
    lounge(&mut g, 1, hall_y + 2, div_x, h - 1);

    // Bottom-right: Gym/Arcade (concrete + treadmills + pinball)
    gym_arcade(&mut g, div_x + 1, hall_y + 2, w - 1, h - 1);

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

fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Wood floors are default, just place desks in a grid pattern
    let room_w = x2 - x1;
    let room_h = y2 - y1;

    // Desk rows: every 3 tiles vertically, every 4 tiles horizontally
    let desk_cols = (room_w.saturating_sub(2)) / 4;
    let desk_rows = (room_h.saturating_sub(1)) / 3;

    for row in 0..desk_rows {
        for col in 0..desk_cols {
            let dx = x1 + 2 + col * 4;
            let dy = y1 + 1 + row * 3;
            if dx < x2 - 1 && dy < y2 - 1 {
                g.set_tile(Position::new(dx, dy), Tile::Desk);
            }
        }
    }

    // Whiteboard on left wall
    if y1 + 2 < y2 {
        g.set_tile(Position::new(x1, y1 + 2), Tile::Whiteboard);
    }
}

fn break_room(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    let room_w = x2 - x1;

    // Vending machines along the top
    let vend_count = (room_w / 4).min(3);
    for i in 0..vend_count {
        let vx = x1 + 1 + i * 3;
        if vx < x2 - 1 {
            g.set_tile(Position::new(vx, y1 + 1), Tile::VendingMachine);
        }
    }

    // Coffee machine
    if x1 + 1 + vend_count * 3 < x2 - 1 {
        g.set_tile(
            Position::new(x1 + 1 + vend_count * 3, y1 + 1),
            Tile::CoffeeMachine,
        );
    }

    // Couches in the middle
    let mid_y = y1 + (y2 - y1) / 2;
    if x1 + 2 < x2 && mid_y < y2 {
        g.set_tile(Position::new(x1 + 2, mid_y), Tile::Couch);
    }
    if x1 + 5 < x2 && mid_y < y2 {
        g.set_tile(Position::new(x1 + 5, mid_y), Tile::Couch);
    }

    // Plants in corners
    if x2 - 2 > x1 && y2 - 2 > y1 {
        g.set_tile(Position::new(x2 - 2, y2 - 2), Tile::Plant);
    }
    if x1 + 1 < x2 && y2 - 2 > y1 {
        g.set_tile(Position::new(x1 + 1, y2 - 2), Tile::Plant);
    }
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Carpet floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }

    // Couches
    if x1 + 2 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 2, y1 + 1), Tile::Couch);
    }
    if x1 + 5 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 5, y1 + 1), Tile::Couch);
    }

    // Plant
    if x1 + 9 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 9, y1 + 1), Tile::Plant);
    }

    // Rug in center
    let rug_x = x1 + 2;
    let rug_y = y1 + (y2 - y1) / 2;
    for dy in 0..2u16 {
        for dx in 0..3u16 {
            if rug_x + dx < x2 && rug_y + dy < y2 {
                g.set_tile(Position::new(rug_x + dx, rug_y + dy), Tile::Rug);
            }
        }
    }
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Concrete floor already default from hallway style
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }

    // Treadmills near top
    if x1 + 2 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 2, y1 + 1), Tile::GymTreadmill);
    }
    if x1 + 6 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 6, y1 + 1), Tile::GymTreadmill);
    }

    // Pinball machines near bottom
    let pb_y = y1 + (y2 - y1) * 2 / 3;
    if x1 + 2 < x2 && pb_y < y2 {
        g.set_tile(Position::new(x1 + 2, pb_y), Tile::PinballMachine);
    }
    if x1 + 6 < x2 && pb_y < y2 {
        g.set_tile(Position::new(x1 + 6, pb_y), Tile::PinballMachine);
    }

    // Plant
    if x2 - 3 > x1 {
        let plant_y = y1 + (y2 - y1) / 2;
        if plant_y < y2 {
            g.set_tile(Position::new(x2 - 3, plant_y), Tile::Plant);
        }
    }
}
