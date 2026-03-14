use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build a dense office world inspired by isometric office sim references.
/// Furniture packed tightly — no wasted space.
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

    // Main hallway (2 tiles wide)
    let hall_y = (h as f64 * 0.58) as u16;
    let div_x = (w as f64 * 0.55) as u16;

    for x in 1..w - 1 {
        for dy in 0..2u16 {
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
    for y in hall_y + 2..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors
    let door1_y = hall_y / 2;
    place_door(&mut g, div_x, door1_y);
    let door2_y = hall_y + 2 + (h - hall_y - 3) / 2;
    if door2_y + 1 < h - 1 {
        place_door(&mut g, div_x, door2_y);
    }

    // Four rooms
    office_area(&mut g, 1, 1, div_x, hall_y);
    kitchen_break(&mut g, div_x + 1, 1, w - 1, hall_y);
    lounge(&mut g, 1, hall_y + 2, div_x, h - 1);
    gym_arcade(&mut g, div_x + 1, hall_y + 2, w - 1, h - 1);

    g
}

fn place_door(g: &mut Grid, x: u16, y: u16) {
    g.set_tile(Position::new(x, y), Tile::DoorOpen);
    g.set_tile(Position::new(x, y + 1), Tile::DoorOpen);
}

fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Desk grid: tight spacing — 4 tiles horizontal, 3 tiles vertical
    let start_x = x1 + 3;
    let start_y = y1 + 2;
    let mut dy = start_y;
    while dy < y2 - 2 {
        let mut dx = start_x;
        while dx < x2 - 2 {
            g.set_tile(Position::new(dx, dy), Tile::Desk);
            dx += 4;
        }
        dy += 3;
    }

    // Whiteboard near left wall
    let wb_y = y1 + (y2 - y1) / 2;
    if x1 + 2 < x2 {
        g.set_tile(Position::new(x1 + 2, wb_y), Tile::Whiteboard);
    }

    // Plants scattered
    g.set_tile(Position::new(x1 + 2, y1 + 2), Tile::Plant);
    if x2 - 3 > x1 {
        g.set_tile(Position::new(x2 - 3, y2 - 2), Tile::Plant);
    }
    if x2 - 3 > x1 && y1 + 2 < y2 {
        g.set_tile(Position::new(x2 - 3, y1 + 2), Tile::Plant);
    }

    // Floor lamps between desk rows
    if start_x + 2 < x2 && start_y + 1 < y2 {
        g.set_tile(Position::new(start_x + 2, start_y + 1), Tile::FloorLamp);
    }
    if start_x + 6 < x2 && start_y + 4 < y2 {
        g.set_tile(Position::new(start_x + 6, start_y + 4), Tile::FloorLamp);
    }
}

fn kitchen_break(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let room_w = x2 - x1;
    let room_h = y2 - y1;

    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    // ── Kitchen counter row along top wall (like reference) ──
    // Multiple coffee machines side by side = kitchen counter look
    let kitchen_y = y1 + 2;
    let mut kx = x1 + 2;
    while kx < x2 - 2 && kx < x1 + 2 + 10 {
        g.set_tile(Position::new(kx, kitchen_y), Tile::CoffeeMachine);
        kx += 2;
    }

    // Vending machines next to kitchen
    let vend_y = kitchen_y + 3;
    if x1 + 2 < x2 {
        g.set_tile(Position::new(x1 + 2, vend_y), Tile::VendingMachine);
    }
    if x1 + 5 < x2 {
        g.set_tile(Position::new(x1 + 5, vend_y), Tile::VendingMachine);
    }

    // Seating area — couches and armchairs
    let seat_y = y1 + room_h / 2 + 1;
    let seat_x = x1 + room_w / 2;
    if seat_x - 2 > x1 && seat_y < y2 - 2 {
        g.set_tile(Position::new(seat_x - 2, seat_y), Tile::Couch);
    }
    if seat_x + 1 < x2 && seat_y < y2 - 2 {
        g.set_tile(Position::new(seat_x + 1, seat_y), Tile::Couch);
    }
    // Armchairs facing couches
    if seat_x - 2 > x1 && seat_y + 2 < y2 - 1 {
        g.set_tile(Position::new(seat_x - 2, seat_y + 2), Tile::SmallArmchair);
    }
    if seat_x + 1 < x2 && seat_y + 2 < y2 - 1 {
        g.set_tile(Position::new(seat_x + 1, seat_y + 2), Tile::SmallArmchair);
    }

    // Plants
    if x2 - 3 > x1 && y2 - 2 > y1 {
        g.set_tile(Position::new(x2 - 3, y2 - 2), Tile::Plant);
    }
    if x1 + 2 < x2 && y2 - 2 > y1 {
        g.set_tile(Position::new(x1 + 2, y2 - 2), Tile::Plant);
    }

    // Floor lamp
    if seat_x + 4 < x2 && seat_y < y2 {
        g.set_tile(Position::new(seat_x + 4, seat_y), Tile::FloorLamp);
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

    // L-shaped couch arrangement (tight)
    let cx = x1 + 3;
    let cy = y1 + 2;
    if cx < x2 - 2 && cy < y2 - 2 {
        g.set_tile(Position::new(cx, cy), Tile::Couch);
    }
    if cx + 3 < x2 - 2 && cy < y2 - 2 {
        g.set_tile(Position::new(cx + 3, cy), Tile::Couch);
    }
    if cx < x2 - 2 && cy + 2 < y2 - 2 {
        g.set_tile(Position::new(cx, cy + 2), Tile::Couch);
    }

    // Rug in front of couches
    let rug_x = cx + 1;
    let rug_y = cy + 3;
    for dy in 0..2u16 {
        for dx in 0..3u16 {
            if rug_x + dx < x2 - 2 && rug_y + dy < y2 - 2 {
                g.set_tile(Position::new(rug_x + dx, rug_y + dy), Tile::Rug);
            }
        }
    }

    // Armchair across from couch
    if cx + 5 < x2 - 2 && cy + 2 < y2 - 2 {
        g.set_tile(Position::new(cx + 5, cy + 2), Tile::SmallArmchair);
    }

    // Plants
    if x1 + 2 < x2 && y2 - 2 > y1 {
        g.set_tile(Position::new(x1 + 2, y2 - 2), Tile::Plant);
    }
    if x2 - 3 > x1 && y1 + 2 < y2 {
        g.set_tile(Position::new(x2 - 3, y1 + 2), Tile::Plant);
    }

    // Floor lamp near couches
    if cx + 7 < x2 - 2 && cy < y2 - 2 {
        g.set_tile(Position::new(cx + 7, cy), Tile::FloorLamp);
    }

    // Ping pong table — in open area
    let pp_x = x1 + room_w / 2 + 2;
    let pp_y = y1 + room_h - 3;
    if pp_x < x2 - 2 && pp_y < y2 - 1 && pp_y > y1 {
        g.set_tile(Position::new(pp_x, pp_y), Tile::PingPongTable);
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

    // ── Gym zone (top portion) — tight rows ──
    let gym_y = y1 + 2;

    // Row 1: Treadmills
    let mut tx = x1 + 3;
    while tx < x2 - 2 && tx < x1 + 3 + 12 {
        g.set_tile(Position::new(tx, gym_y), Tile::GymTreadmill);
        tx += 4;
    }

    // Row 2: Weight benches + yoga mats
    let row2_y = gym_y + 3;
    if row2_y < y2 - 2 {
        let mut bx = x1 + 3;
        while bx < x2 - 2 && bx < x1 + 3 + 6 {
            g.set_tile(Position::new(bx, row2_y), Tile::WeightBench);
            bx += 3;
        }
        // Yoga mats next to benches
        let mut mx = bx + 1;
        while mx < x2 - 2 && mx < bx + 8 {
            g.set_tile(Position::new(mx, row2_y), Tile::YogaMat);
            mx += 3;
        }
    }

    // ── Arcade zone (bottom portion) ──
    let arcade_y = if room_h > 6 { y1 + room_h - 4 } else { y1 + 2 };
    let mut ax = x1 + 3;
    while ax < x2 - 2 && ax < x1 + 3 + 12 {
        if arcade_y < y2 - 1 {
            g.set_tile(Position::new(ax, arcade_y), Tile::PinballMachine);
        }
        ax += 4;
    }

    // Plants
    if x2 - 3 > x1 && y1 + room_h / 2 < y2 {
        g.set_tile(Position::new(x2 - 3, y1 + room_h / 2), Tile::Plant);
    }
    if x1 + 2 < x2 && y2 - 2 > y1 {
        g.set_tile(Position::new(x1 + 2, y2 - 2), Tile::Plant);
    }
}
