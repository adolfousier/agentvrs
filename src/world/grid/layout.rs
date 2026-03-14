use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

/// Build office world. Furniture stays 3+ tiles from walls to avoid
/// being hidden behind isometric wall extrusions.
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

    // Hallway (2 tiles wide)
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

    // Four rooms — margin=3 from every wall
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

/// Safe zone: furniture only between (x1+3, y1+3) and (x2-3, y2-2)
fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    let fx1 = x1 + 3; // left margin
    let fy1 = y1 + 3; // top margin (behind walls)
    let fx2 = x2 - 2; // right margin (near divider)
    let fy2 = y2 - 2; // bottom margin (near hallway)

    // Desk grid: 4 horizontal, 3 vertical spacing
    let mut dy = fy1;
    while dy <= fy2 {
        let mut dx = fx1;
        while dx <= fx2 {
            g.set_tile(Position::new(dx, dy), Tile::Desk);
            dx += 4;
        }
        dy += 3;
    }

    // Whiteboard — left side, between desk rows
    if fx1 + 1 < fx2 && fy1 + 4 < fy2 {
        g.set_tile(Position::new(fx1 - 1, fy1 + 4), Tile::Whiteboard);
    }

    // Plants in corners of safe zone
    place_if_empty(g, fx1, fy2, Tile::Plant);
    place_if_empty(g, fx2, fy1, Tile::Plant);

    // Floor lamps between desks
    if fx1 + 2 <= fx2 && fy1 + 1 <= fy2 {
        place_if_empty(g, fx1 + 2, fy1 + 1, Tile::FloorLamp);
    }
    if fx1 + 6 <= fx2 && fy1 + 4 <= fy2 {
        place_if_empty(g, fx1 + 6, fy1 + 4, Tile::FloorLamp);
    }
}

fn kitchen_break(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    let fx1 = x1 + 3;
    let fy1 = y1 + 3;
    let fx2 = x2 - 3;
    let fy2 = y2 - 2;

    // Kitchen counter: row of coffee machines along back area
    let mut kx = fx1;
    let mut count = 0;
    while kx <= fx2 && count < 4 {
        g.set_tile(Position::new(kx, fy1), Tile::CoffeeMachine);
        kx += 3;
        count += 1;
    }

    // Vending machines nearby
    if fx1 <= fx2 && fy1 + 3 <= fy2 {
        g.set_tile(Position::new(fx1, fy1 + 3), Tile::VendingMachine);
    }
    if fx1 + 3 <= fx2 && fy1 + 3 <= fy2 {
        g.set_tile(Position::new(fx1 + 3, fy1 + 3), Tile::VendingMachine);
    }

    // Seating: couches + armchairs
    let mid_y = (fy1 + fy2) / 2 + 2;
    if fx1 <= fx2 && mid_y <= fy2 {
        g.set_tile(Position::new(fx1, mid_y), Tile::Couch);
    }
    if fx1 + 3 <= fx2 && mid_y <= fy2 {
        g.set_tile(Position::new(fx1 + 3, mid_y), Tile::Couch);
    }
    if fx1 + 1 <= fx2 && mid_y + 2 <= fy2 {
        g.set_tile(Position::new(fx1 + 1, mid_y + 2), Tile::SmallArmchair);
    }

    // Plants + lamp
    place_if_empty(g, fx2, fy2, Tile::Plant);
    place_if_empty(g, fx2, fy1, Tile::Plant);
    if fx1 + 6 <= fx2 && mid_y <= fy2 {
        place_if_empty(g, fx1 + 6, mid_y, Tile::FloorLamp);
    }
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Carpet floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }

    let fx1 = x1 + 3;
    let fy1 = y1 + 2; // hallway side, no tall wall
    let fx2 = x2 - 2;
    let fy2 = y2 - 3;

    // L-shaped couch
    if fx1 <= fx2 && fy1 <= fy2 {
        g.set_tile(Position::new(fx1, fy1), Tile::Couch);
    }
    if fx1 + 3 <= fx2 && fy1 <= fy2 {
        g.set_tile(Position::new(fx1 + 3, fy1), Tile::Couch);
    }
    if fx1 <= fx2 && fy1 + 2 <= fy2 {
        g.set_tile(Position::new(fx1, fy1 + 2), Tile::Couch);
    }

    // Rug in front of couch
    let rug_x = fx1 + 1;
    let rug_y = fy1 + 3;
    for dy in 0..2u16 {
        for dx in 0..3u16 {
            if rug_x + dx <= fx2 && rug_y + dy <= fy2 {
                g.set_tile(Position::new(rug_x + dx, rug_y + dy), Tile::Rug);
            }
        }
    }

    // Armchair facing couch
    if fx1 + 5 <= fx2 && fy1 + 2 <= fy2 {
        g.set_tile(Position::new(fx1 + 5, fy1 + 2), Tile::SmallArmchair);
    }

    // Floor lamp
    if fx1 + 7 <= fx2 && fy1 <= fy2 {
        g.set_tile(Position::new(fx1 + 7, fy1), Tile::FloorLamp);
    }

    // Ping pong in the open area
    let pp_x = fx1 + 10;
    let pp_y = fy1 + 2;
    if pp_x <= fx2 && pp_y <= fy2 {
        g.set_tile(Position::new(pp_x, pp_y), Tile::PingPongTable);
    }

    // Plants
    place_if_empty(g, fx2, fy2, Tile::Plant);
    place_if_empty(g, fx1, fy2, Tile::Plant);
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Concrete floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }

    let fx1 = x1 + 3;
    let fy1 = y1 + 2; // hallway side
    let fx2 = x2 - 3;
    let fy2 = y2 - 3;

    // Gym row 1: treadmills
    let mut tx = fx1;
    while tx <= fx2 {
        g.set_tile(Position::new(tx, fy1), Tile::GymTreadmill);
        tx += 4;
    }

    // Gym row 2: weight benches + yoga mats
    let row2 = fy1 + 3;
    if row2 <= fy2 {
        let mut bx = fx1;
        while bx <= fx2.min(fx1 + 6) {
            g.set_tile(Position::new(bx, row2), Tile::WeightBench);
            bx += 4;
        }
        let mut mx = bx;
        while mx <= fx2 {
            g.set_tile(Position::new(mx, row2), Tile::YogaMat);
            mx += 3;
        }
    }

    // Arcade row
    let arcade_y = fy2;
    let mut ax = fx1;
    while ax <= fx2 {
        g.set_tile(Position::new(ax, arcade_y), Tile::PinballMachine);
        ax += 4;
    }

    // Plants
    place_if_empty(g, fx2, fy1, Tile::Plant);
    if fy2 - 2 > fy1 {
        place_if_empty(g, fx1, fy2 - 2, Tile::Plant);
    }
}

fn place_if_empty(g: &mut Grid, x: u16, y: u16, tile: Tile) {
    let pos = Position::new(x, y);
    let is_floor = g.get(pos).map_or(false, |c| !c.tile.is_solid());
    if is_floor {
        g.set_tile(pos, tile);
    }
}
