use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

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

    // Hallway + divider
    let hall_y = (h as f64 * 0.58) as u16;
    let div_x = (w as f64 * 0.55) as u16;

    for x in 1..w - 1 {
        for dy in 0..2u16 {
            if hall_y + dy < h - 1 {
                g.set_tile(Position::new(x, hall_y + dy), Tile::Floor(FloorKind::Concrete));
            }
        }
    }
    for y in 1..hall_y {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }
    for y in hall_y + 2..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors
    place_door(&mut g, div_x, hall_y / 2);
    let d2 = hall_y + 2 + (h - hall_y - 3) / 2;
    if d2 + 1 < h - 1 {
        place_door(&mut g, div_x, d2);
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
    // Room interior (excluding walls)
    let rw = x2 - x1 - 1; // usable width
    let rh = y2 - y1 - 1; // usable height

    // Desk grid — evenly distributed across room
    // Calculate how many desks fit with 4-tile spacing, centered
    let desk_cols = (rw - 2) / 4;
    let desk_rows = (rh - 2) / 3;
    let start_x = x1 + 1 + (rw - desk_cols * 4) / 2 + 2;
    let start_y = y1 + 1 + (rh - desk_rows * 3) / 2 + 1;

    for row in 0..desk_rows {
        for col in 0..desk_cols {
            let dx = start_x + col * 4;
            let dy = start_y + row * 3;
            if dx < x2 - 1 && dy < y2 - 1 {
                g.set_tile(Position::new(dx, dy), Tile::Desk);
            }
        }
    }

    // Whiteboard — left side midway
    let wb_x = x1 + 2;
    let wb_y = y1 + rh / 2;
    if wb_x < x2 - 1 && wb_y < y2 - 1 {
        g.set_tile(Position::new(wb_x, wb_y), Tile::Whiteboard);
    }

    // Plants — back corners
    safe_place(g, x1 + 2, y1 + 2, x2, y2, Tile::Plant);
    safe_place(g, x2 - 3, y1 + 2, x2, y2, Tile::Plant);

    // Floor lamps — scattered between desks
    let lamp1_x = start_x + 2;
    let lamp1_y = start_y + 1;
    safe_place(g, lamp1_x, lamp1_y, x2, y2, Tile::FloorLamp);
    if desk_cols > 2 {
        safe_place(g, start_x + 6, start_y + 4, x2, y2, Tile::FloorLamp);
    }
}

fn kitchen_break(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let rw = x2 - x1 - 1;
    let rh = y2 - y1 - 1;

    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    // Kitchen counter along the top third — coffee machines evenly spaced
    let counter_y = y1 + 2 + rh / 4;
    let counter_start = x1 + 2;
    let counter_end = x2 - 2;
    let counter_count = ((counter_end - counter_start) / 3).min(4);
    let counter_offset = (counter_end - counter_start - counter_count * 3) / 2;
    for i in 0..counter_count {
        let cx = counter_start + counter_offset + i * 3;
        if cx < x2 - 1 && counter_y < y2 - 1 {
            g.set_tile(Position::new(cx, counter_y), Tile::CoffeeMachine);
        }
    }

    // Vending machines — spaced along the side
    let vend_y = counter_y + 3;
    if vend_y < y2 - 1 {
        safe_place(g, x1 + 2 + rw / 5, vend_y, x2, y2, Tile::VendingMachine);
        safe_place(g, x1 + 2 + rw * 3 / 5, vend_y, x2, y2, Tile::VendingMachine);
    }

    // Seating area — bottom third of room, centered
    let seat_y = y1 + 1 + rh * 2 / 3;
    let seat_cx = x1 + 1 + rw / 2;
    if seat_y < y2 - 1 {
        safe_place(g, seat_cx - 2, seat_y, x2, y2, Tile::Couch);
        safe_place(g, seat_cx + 1, seat_y, x2, y2, Tile::Couch);
    }
    if seat_y + 2 < y2 - 1 {
        safe_place(g, seat_cx - 1, seat_y + 2, x2, y2, Tile::SmallArmchair);
        safe_place(g, seat_cx + 2, seat_y + 2, x2, y2, Tile::SmallArmchair);
    }

    // Plants + lamp distributed
    safe_place(g, x2 - 3, y2 - 3, x2, y2, Tile::Plant);
    safe_place(g, x1 + 2, y2 - 3, x2, y2, Tile::Plant);
    safe_place(g, seat_cx + 5, seat_y, x2, y2, Tile::FloorLamp);
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let rw = x2 - x1 - 1;
    let rh = y2 - y1 - 1;

    // Carpet floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }

    // Left third: L-shaped couch arrangement
    let cx = x1 + 2 + rw / 6;
    let cy = y1 + 2 + rh / 4;
    safe_place(g, cx, cy, x2, y2, Tile::Couch);
    safe_place(g, cx + 3, cy, x2, y2, Tile::Couch);
    safe_place(g, cx, cy + 2, x2, y2, Tile::Couch);

    // Rug in front of couch
    for dy in 0..2u16 {
        for dx in 0..3u16 {
            let rx = cx + 1 + dx;
            let ry = cy + 3 + dy;
            if rx < x2 - 1 && ry < y2 - 1 {
                g.set_tile(Position::new(rx, ry), Tile::Rug);
            }
        }
    }

    // Armchair across from couch
    safe_place(g, cx + 5, cy + 1, x2, y2, Tile::SmallArmchair);

    // Floor lamp near couch
    safe_place(g, cx + 7, cy, x2, y2, Tile::FloorLamp);

    // Right third: ping pong table centered
    let pp_x = x1 + 2 + rw * 2 / 3;
    let pp_y = y1 + 1 + rh / 2;
    safe_place(g, pp_x, pp_y, x2, y2, Tile::PingPongTable);

    // Plants in corners
    safe_place(g, x2 - 3, y2 - 2, x2, y2, Tile::Plant);
    safe_place(g, x1 + 2, y2 - 2, x2, y2, Tile::Plant);
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let rw = x2 - x1 - 1;
    let rh = y2 - y1 - 1;

    // Concrete floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }

    // Top third: gym equipment row — centered, evenly spaced
    let gym_y = y1 + 2 + rh / 4;
    let equip_start = x1 + 2 + rw / 6;
    let equip_spacing = 4_u16;
    let items: &[Tile] = &[Tile::GymTreadmill, Tile::WeightBench, Tile::GymTreadmill, Tile::WeightBench];
    for (i, tile) in items.iter().enumerate() {
        let ex = equip_start + i as u16 * equip_spacing;
        if ex < x2 - 1 && gym_y < y2 - 1 {
            g.set_tile(Position::new(ex, gym_y), tile.clone());
        }
    }

    // Middle: yoga mats
    let mat_y = gym_y + 3;
    if mat_y < y2 - 1 {
        safe_place(g, equip_start + 1, mat_y, x2, y2, Tile::YogaMat);
        safe_place(g, equip_start + 5, mat_y, x2, y2, Tile::YogaMat);
        safe_place(g, equip_start + 9, mat_y, x2, y2, Tile::YogaMat);
    }

    // Bottom third: arcade machines — evenly spaced
    let arcade_y = y1 + 1 + rh * 3 / 4;
    if arcade_y < y2 - 1 {
        let arc_start = x1 + 2 + rw / 5;
        safe_place(g, arc_start, arcade_y, x2, y2, Tile::PinballMachine);
        safe_place(g, arc_start + 4, arcade_y, x2, y2, Tile::PinballMachine);
        safe_place(g, arc_start + 8, arcade_y, x2, y2, Tile::PinballMachine);
    }

    // Plants
    safe_place(g, x2 - 3, y1 + 2 + rh / 2, x2, y2, Tile::Plant);
    safe_place(g, x1 + 2, y2 - 2, x2, y2, Tile::Plant);
}

/// Place tile only if position is inside room bounds and currently a floor tile.
fn safe_place(g: &mut Grid, x: u16, y: u16, max_x: u16, max_y: u16, tile: Tile) {
    if x >= max_x - 1 || y >= max_y - 1 {
        return;
    }
    let pos = Position::new(x, y);
    if g.get(pos).map_or(false, |c| !c.tile.is_solid()) {
        g.set_tile(pos, tile);
    }
}
