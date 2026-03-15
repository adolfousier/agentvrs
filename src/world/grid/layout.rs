use super::{FloorKind, Grid, Tile};
use crate::world::Position;

/// Build a compact, densely furnished office.
/// Every zone has furniture but also walkable paths between items.
pub fn build_office_world(w: u16, h: u16) -> Grid {
    let mut g = Grid::new(w, h);

    let hx = w / 2;
    let hy = h / 2;

    // ── Top-left: Office (wood floor — default) ─────────────────────
    // Desks in rows with walkable aisles between them
    p(&mut g, w, h, 0, 0, Tile::Desk);
    p(&mut g, w, h, 2, 0, Tile::Desk);
    p(&mut g, w, h, 4, 0, Tile::Desk);
    p(&mut g, w, h, 0, 2, Tile::Desk);
    p(&mut g, w, h, 2, 2, Tile::Desk);
    p(&mut g, w, h, 4, 2, Tile::Whiteboard);
    // row 1 and 3 are walkable aisles

    // ── Top-right: Kitchen/break (tile floor) ───────────────────────
    for y in 0..hy {
        for x in hx..w {
            p(&mut g, w, h, x, y, Tile::Floor(FloorKind::Tile));
        }
    }
    p(&mut g, w, h, hx, 0, Tile::KitchenCounter);
    p(&mut g, w, h, hx + 1, 0, Tile::CoffeeMachine);
    if hx + 2 < w {
        p(&mut g, w, h, hx + 2, 0, Tile::VendingMachine);
    }
    p(&mut g, w, h, w - 1, 0, Tile::Plant);
    p(&mut g, w, h, hx, 2, Tile::Couch);
    p(&mut g, w, h, hx + 1, 2, Tile::SmallArmchair);
    if hx + 2 < w {
        p(&mut g, w, h, hx + 2, 2, Tile::Plant);
    }

    // ── Bottom-left: Lounge (carpet) ────────────────────────────────
    for y in hy..h {
        for x in 0..hx {
            p(&mut g, w, h, x, y, Tile::Floor(FloorKind::Carpet));
        }
    }
    p(&mut g, w, h, 0, hy, Tile::FloorLamp);
    p(&mut g, w, h, 1, hy, Tile::Couch);
    p(&mut g, w, h, 2, hy, Tile::Couch);
    p(&mut g, w, h, 0, hy + 1, Tile::Rug);
    p(&mut g, w, h, 1, hy + 1, Tile::Rug);
    p(&mut g, w, h, 3, hy, Tile::SmallArmchair);
    if hy + 2 < h {
        p(&mut g, w, h, 0, hy + 2, Tile::Plant);
        p(&mut g, w, h, 3, hy + 2, Tile::MeetingTable);
    }

    // ── Bottom-right: Gym/arcade (concrete) ─────────────────────────
    for y in hy..h {
        for x in hx..w {
            p(&mut g, w, h, x, y, Tile::Floor(FloorKind::Concrete));
        }
    }
    // Keep equipment 1+ tile inward from the zone boundary so agents
    // stand on concrete, not carpet.
    p(&mut g, w, h, hx + 1, hy, Tile::GymTreadmill);
    p(&mut g, w, h, hx + 3, hy, Tile::WeightBench);
    p(&mut g, w, h, w - 1, hy, Tile::VendingMachine);
    if hy + 1 < h {
        p(&mut g, w, h, hx + 1, hy + 1, Tile::YogaMat);
    }
    if hy + 2 < h {
        p(&mut g, w, h, hx + 1, hy + 2, Tile::PinballMachine);
        p(&mut g, w, h, hx + 3, hy + 2, Tile::PinballMachine);
        p(&mut g, w, h, w - 1, hy + 2, Tile::Plant);
    }

    g
}

fn p(g: &mut Grid, w: u16, h: u16, x: u16, y: u16, tile: Tile) {
    if x < w && y < h {
        g.set_tile(Position::new(x, y), tile);
    }
}
