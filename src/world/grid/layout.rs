use super::{FloorKind, Grid, Tile, WallKind};
use crate::world::Position;

pub fn build_office_world(w: u16, h: u16) -> Grid {
    let mut g = Grid::new(w, h);

    // Outer walls (1 tile)
    for x in 0..w {
        g.set_tile(Position::new(x, 0), Tile::Wall(WallKind::Window));
        g.set_tile(Position::new(x, h - 1), Tile::Wall(WallKind::Solid));
    }
    for y in 0..h {
        g.set_tile(Position::new(0, y), Tile::Wall(WallKind::Solid));
        g.set_tile(Position::new(w - 1, y), Tile::Wall(WallKind::Window));
    }

    // Dividers — hallway is just 1 tile wide
    let hall_y = h / 2;
    let div_x = w / 2;

    // Horizontal hallway (1 tile)
    for x in 1..w - 1 {
        g.set_tile(Position::new(x, hall_y), Tile::Floor(FloorKind::Concrete));
    }

    // Vertical divider wall
    for y in 1..hall_y {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }
    for y in hall_y + 1..h - 1 {
        g.set_tile(Position::new(div_x, y), Tile::Wall(WallKind::Solid));
    }

    // Doors (1 tile wide)
    g.set_tile(Position::new(div_x, hall_y / 2), Tile::DoorOpen);
    let d2 = hall_y + 1 + (h - hall_y - 2) / 2;
    if d2 < h - 1 {
        g.set_tile(Position::new(div_x, d2), Tile::DoorOpen);
    }

    // Four rooms — packed tight
    office_area(&mut g, 1, 1, div_x, hall_y);
    kitchen_break(&mut g, div_x + 1, 1, w - 1, hall_y);
    lounge(&mut g, 1, hall_y + 1, div_x, h - 1);
    gym_arcade(&mut g, div_x + 1, hall_y + 1, w - 1, h - 1);

    g
}

fn office_area(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Desks packed tight — every 3 tiles horizontal, every 2 tiles vertical
    let mut y = y1 + 1;
    while y < y2 - 1 {
        let mut x = x1 + 1;
        while x < x2 - 1 {
            g.set_tile(Position::new(x, y), Tile::Desk);
            x += 3;
        }
        y += 2;
    }

    // Whiteboard on left wall
    let wb_y = y1 + (y2 - y1) / 2;
    if wb_y < y2 {
        g.set_tile(Position::new(x1, wb_y), Tile::Whiteboard);
    }

}

fn kitchen_break(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Tile floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Tile));
        }
    }

    // Kitchen counter IN the wall — directly at y1 (touching top wall)
    for x in x1..x2 {
        g.set_tile(Position::new(x, y1), Tile::KitchenCounter);
    }
    // Coffee machine at one end
    g.set_tile(Position::new(x2 - 1, y1), Tile::CoffeeMachine);

    // Vending machines — 1 tile inward from side wall
    if x1 + 1 < x2 && y1 + 2 < y2 {
        g.set_tile(Position::new(x1 + 1, y1 + 2), Tile::VendingMachine);
    }
    if x1 + 1 < x2 && y1 + 4 < y2 {
        g.set_tile(Position::new(x1 + 1, y1 + 4), Tile::VendingMachine);
    }

    // Seating in remaining space
    let mid_x = x1 + (x2 - x1) / 2;
    let seat_y = y1 + (y2 - y1) * 2 / 3;
    if mid_x < x2 && seat_y < y2 - 1 {
        g.set_tile(Position::new(mid_x, seat_y), Tile::Couch);
    }
    if mid_x + 2 < x2 && seat_y < y2 - 1 {
        g.set_tile(Position::new(mid_x + 2, seat_y), Tile::SmallArmchair);
    }

    // Plant — away from wall
    if x2 - 2 > x1 && y2 - 2 > y1 {
        g.set_tile(Position::new(x2 - 2, y2 - 2), Tile::Plant);
    }
}

fn lounge(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Carpet floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Carpet));
        }
    }

    // Couches — L-shape, tight
    if x1 + 1 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 1, y1 + 1), Tile::Couch);
    }
    if x1 + 3 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 3, y1 + 1), Tile::Couch);
    }
    if x1 + 1 < x2 && y1 + 3 < y2 {
        g.set_tile(Position::new(x1 + 1, y1 + 3), Tile::Couch);
    }

    // Rug (2x2)
    for dy in 0..2u16 {
        for dx in 0..2u16 {
            let rx = x1 + 3 + dx;
            let ry = y1 + 3 + dy;
            if rx < x2 && ry < y2 {
                g.set_tile(Position::new(rx, ry), Tile::Rug);
            }
        }
    }

    // Armchair
    if x1 + 5 < x2 && y1 + 2 < y2 {
        g.set_tile(Position::new(x1 + 5, y1 + 2), Tile::SmallArmchair);
    }

    // Ping pong — two tiles side by side for rectangular table
    let pp_x = x2 - 5;
    let pp_y = y1 + (y2 - y1) / 2;
    if pp_x > x1 && pp_x + 1 < x2 && pp_y < y2 {
        g.set_tile(Position::new(pp_x, pp_y), Tile::PingPongTableLeft);
        g.set_tile(Position::new(pp_x + 1, pp_y), Tile::PingPongTableRight);
    }

    // Floor lamp + plant
    if x1 + 6 < x2 && y1 + 1 < y2 {
        g.set_tile(Position::new(x1 + 6, y1 + 1), Tile::FloorLamp);
    }
    if x2 - 2 > x1 && y2 - 2 > y1 {
        g.set_tile(Position::new(x2 - 2, y2 - 2), Tile::Plant);
    }
}

fn gym_arcade(g: &mut Grid, x1: u16, y1: u16, x2: u16, y2: u16) {
    // Concrete floor
    for y in y1..y2 {
        for x in x1..x2 {
            g.set_tile(Position::new(x, y), Tile::Floor(FloorKind::Concrete));
        }
    }

    // Gym row — treadmill, weight bench, treadmill
    let gy = y1 + 1;
    let mut gx = x1 + 1;
    let gym_items = [Tile::GymTreadmill, Tile::WeightBench, Tile::GymTreadmill];
    for item in &gym_items {
        if gx < x2 - 1 && gy < y2 {
            g.set_tile(Position::new(gx, gy), item.clone());
        }
        gx += 2;
    }

    // Yoga mats row
    let my = gy + 2;
    if my < y2 {
        let mut mx = x1 + 1;
        while mx < x2 - 1 {
            g.set_tile(Position::new(mx, my), Tile::YogaMat);
            mx += 2;
        }
    }

    // Arcade row
    let ay = y2 - 2;
    if ay > y1 {
        let mut ax = x1 + 1;
        while ax < x2 - 1 {
            g.set_tile(Position::new(ax, ay), Tile::PinballMachine);
            ax += 3;
        }
    }

}
