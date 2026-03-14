use crate::gui::iso::{TILE_H, TILE_W, WALL_HEIGHT};
use crate::world::{FloorKind, Tile, WallKind};
use std::f64::consts::TAU;

// Agent reference: total height ~54px at zoom=1.
// All furniture scaled relative to that (desk=waist, vending=taller than person, etc.)

pub fn draw_tile(cr: &gtk4::cairo::Context, sx: f64, sy: f64, tile: &Tile, zoom: f64) {
    match tile {
        Tile::Floor(kind) => draw_floor(cr, sx, sy, kind, zoom),
        Tile::Wall(kind) => draw_wall(cr, sx, sy, kind, zoom),
        Tile::DoorOpen => draw_door(cr, sx, sy, zoom),
        Tile::Rug => draw_rug(cr, sx, sy, zoom),
        Tile::Desk => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_desk(cr, sx, sy, zoom);
        }
        Tile::VendingMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.88, 0.84, 0.78);
            draw_vending(cr, sx, sy, zoom);
        }
        Tile::CoffeeMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.88, 0.84, 0.78);
            draw_coffee(cr, sx, sy, zoom);
        }
        Tile::Couch => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.42, 0.38, 0.50);
            draw_couch(cr, sx, sy, zoom);
        }
        Tile::Plant => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_plant(cr, sx, sy, zoom);
        }
        Tile::PinballMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_arcade(cr, sx, sy, zoom);
        }
        Tile::GymTreadmill => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_treadmill(cr, sx, sy, zoom);
        }
        Tile::WeightBench => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_weight_bench(cr, sx, sy, zoom);
        }
        Tile::YogaMat => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_yoga_mat(cr, sx, sy, zoom);
        }
        Tile::FloorLamp => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_floor_lamp(cr, sx, sy, zoom);
        }
        Tile::PingPongTable => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_ping_pong_table(cr, sx, sy, zoom);
        }
        Tile::SmallArmchair => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.42, 0.38, 0.50);
            draw_small_armchair(cr, sx, sy, zoom);
        }
        Tile::Whiteboard => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_whiteboard(cr, sx, sy, zoom);
        }
    }
}

// ─── Floor ───

fn draw_floor(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &FloorKind, zoom: f64) {
    let (r, g, b) = match kind {
        FloorKind::Wood => (0.78, 0.62, 0.42),     // Warm beige/tan
        FloorKind::Tile => (0.88, 0.84, 0.78),     // Warm off-white
        FloorKind::Carpet => (0.42, 0.38, 0.50),   // Muted warm purple
        FloorKind::Concrete => (0.32, 0.30, 0.30), // Dark charcoal (gym)
    };
    draw_floor_diamond(cr, sx, sy, zoom, r, g, b);

    match kind {
        FloorKind::Wood => {
            cr.set_source_rgba(0.60, 0.42, 0.22, 0.25);
            cr.set_line_width(0.5 * zoom);
            let hw = TILE_W / 2.0 * zoom;
            for i in 1..4 {
                let t = i as f64 / 4.0;
                let x0 = sx - hw * (1.0 - t);
                let y0 = sy - (TILE_H / 2.0 * zoom) * (1.0 - t) + (TILE_H / 2.0 * zoom) * t;
                let x1 = sx + hw * t;
                let y1 = sy - (TILE_H / 2.0 * zoom) * t + (TILE_H / 2.0 * zoom) * (1.0 - t);
                cr.move_to(x0, y0);
                cr.line_to(x1, y1);
                let _ = cr.stroke();
            }
        }
        FloorKind::Tile => {
            cr.set_source_rgba(0.75, 0.70, 0.64, 0.35);
            cr.set_line_width(0.5 * zoom);
            cr.move_to(sx, sy - TILE_H / 2.0 * zoom);
            cr.line_to(sx, sy + TILE_H / 2.0 * zoom);
            let _ = cr.stroke();
            cr.move_to(sx - TILE_W / 2.0 * zoom, sy);
            cr.line_to(sx + TILE_W / 2.0 * zoom, sy);
            let _ = cr.stroke();
        }
        _ => {}
    }
}

fn draw_floor_diamond(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    zoom: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * zoom;
    let hh = TILE_H / 2.0 * zoom;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.7, g * 0.7, b * 0.7);
    cr.set_line_width(0.5);
    let _ = cr.stroke();
}

// ─── Walls ───

fn draw_wall(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &WallKind, zoom: f64) {
    let hw = TILE_W / 2.0 * zoom;
    let hh = TILE_H / 2.0 * zoom;
    let wh = WALL_HEIGHT * zoom;

    let (r, g, b) = match kind {
        WallKind::Solid => (0.55, 0.45, 0.35),  // Warm brown
        WallKind::Window => (0.52, 0.48, 0.42), // Warm brown-grey
    };

    // Left face
    cr.move_to(sx - hw, sy - wh);
    cr.line_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.55, g * 0.55, b * 0.55);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.35, g * 0.35, b * 0.35);
    cr.set_line_width(0.5);
    let _ = cr.stroke();

    // Right face
    cr.move_to(sx + hw, sy - wh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.72, g * 0.72, b * 0.72);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.45, g * 0.45, b * 0.45);
    let _ = cr.stroke();

    // Top face
    cr.move_to(sx, sy - hh - wh);
    cr.line_to(sx + hw, sy - wh);
    cr.line_to(sx, sy + hh - wh);
    cr.line_to(sx - hw, sy - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.9, g * 0.9, b * 0.9);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.55, g * 0.55, b * 0.55);
    let _ = cr.stroke();

    if matches!(kind, WallKind::Window) {
        let inset = 4.0 * zoom;
        let pane_h = wh * 0.55;
        let pane_top = wh * 0.2;
        let ly_base = sy - wh + pane_top;

        // Right face window
        let rx0 = sx + inset * 0.3;
        let rx1 = sx + hw - inset * 0.7;
        cr.move_to(rx0, ly_base + hh * 0.85);
        cr.line_to(rx1, ly_base + hh * 0.15);
        cr.line_to(rx1, ly_base + hh * 0.15 + pane_h);
        cr.line_to(rx0, ly_base + hh * 0.85 + pane_h);
        cr.close_path();
        cr.set_source_rgba(0.55, 0.78, 0.95, 0.5);
        let _ = cr.fill();
        // Frame divider
        cr.set_source_rgba(0.35, 0.35, 0.4, 0.6);
        cr.set_line_width(1.0 * zoom);
        let mid_rx = (rx0 + rx1) / 2.0;
        let mid_ry = ly_base + hh * 0.5;
        cr.move_to(mid_rx, mid_ry);
        cr.line_to(mid_rx, mid_ry + pane_h);
        let _ = cr.stroke();
        // Sky reflection
        cr.set_source_rgba(0.85, 0.92, 1.0, 0.15);
        cr.move_to(rx0 + 2.0 * zoom, ly_base + hh * 0.85 + 2.0 * zoom);
        cr.line_to(rx1 - 2.0 * zoom, ly_base + hh * 0.15 + 2.0 * zoom);
        cr.line_to(rx1 - 2.0 * zoom, ly_base + hh * 0.15 + pane_h * 0.4);
        cr.line_to(rx0 + 2.0 * zoom, ly_base + hh * 0.85 + pane_h * 0.4);
        cr.close_path();
        let _ = cr.fill();
    }

    if matches!(kind, WallKind::Solid) {
        cr.set_source_rgba(0.35, 0.28, 0.20, 0.3);
        cr.set_line_width(0.5 * zoom);
        for i in 1..3 {
            let t = i as f64 / 3.0;
            let y = sy + hh - wh + t * wh;
            cr.move_to(sx, y);
            cr.line_to(sx + hw, y - hh);
            let _ = cr.stroke();
        }
    }
}

fn draw_door(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.48, 0.40, 0.32);
    let hw = TILE_W / 2.0 * zoom * 0.55;
    let hh = TILE_H / 2.0 * zoom * 0.55;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(0.38, 0.32, 0.26);
    let _ = cr.fill();
}

fn draw_rug(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.55, 0.22, 0.18);
    // Border
    let hw = TILE_W / 2.0 * zoom * 0.85;
    let hh = TILE_H / 2.0 * zoom * 0.85;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgba(0.80, 0.55, 0.18, 0.6);
    cr.set_line_width(2.0 * zoom);
    let _ = cr.stroke();
    // Inner pattern
    let hw2 = TILE_W / 2.0 * zoom * 0.5;
    let hh2 = TILE_H / 2.0 * zoom * 0.5;
    cr.move_to(sx, sy - hh2);
    cr.line_to(sx + hw2, sy);
    cr.line_to(sx, sy + hh2);
    cr.line_to(sx - hw2, sy);
    cr.close_path();
    cr.set_source_rgb(0.70, 0.30, 0.15);
    let _ = cr.fill();
    // Medallion
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 5.0 * zoom, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.80, 0.55, 0.18);
    let _ = cr.fill();
}

// ─── Isometric block helper ───

#[allow(clippy::too_many_arguments)]
fn iso_block(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    height: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let bh = height * z;

    // Left face
    cr.move_to(sx - hw, sy - bh);
    cr.line_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - bh);
    cr.close_path();
    cr.set_source_rgb(r * 0.55, g * 0.55, b * 0.55);
    let _ = cr.fill();

    // Right face
    cr.move_to(sx + hw, sy - bh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - bh);
    cr.close_path();
    cr.set_source_rgb(r * 0.75, g * 0.75, b * 0.75);
    let _ = cr.fill();

    // Top face
    cr.move_to(sx, sy - hh - bh);
    cr.line_to(sx + hw, sy - bh);
    cr.line_to(sx, sy + hh - bh);
    cr.line_to(sx - hw, sy - bh);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
}

// ─── Furniture (scaled to agent height ~54px) ───

fn draw_desk(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // L-shaped desk with office chair
    // Warm brown color: (0.55, 0.40, 0.22)
    let desk_elev = 22.0;

    // ── Desk legs (4 iso_block pillars) ──
    // Back-left leg
    iso_block(cr, sx - 12.0 * z, sy - 2.0 * z, z, 0.06, 0.06, desk_elev, 0.38, 0.38, 0.40);
    // Back-right leg
    iso_block(cr, sx + 12.0 * z, sy - 6.0 * z, z, 0.06, 0.06, desk_elev, 0.38, 0.38, 0.40);
    // Front-left leg
    iso_block(cr, sx - 8.0 * z, sy + 6.0 * z, z, 0.06, 0.06, desk_elev, 0.38, 0.38, 0.40);
    // Front-right leg
    iso_block(cr, sx + 12.0 * z, sy + 6.0 * z, z, 0.06, 0.06, desk_elev, 0.38, 0.38, 0.40);

    // ── L-shape: main desk surface (long part) ──
    let main_sy = sy - desk_elev * z;
    iso_block(cr, sx + 2.0 * z, main_sy, z, 0.80, 0.50, 3.0, 0.55, 0.40, 0.22);

    // ── L-shape: short wing (perpendicular, extends toward front-left) ──
    iso_block(cr, sx - 10.0 * z, main_sy + 2.0 * z, z, 0.35, 0.55, 3.0, 0.55, 0.40, 0.22);

    let desk_top = main_sy - 3.0 * z;

    // ── Monitor stand (small pedestal on desk) ──
    iso_block(cr, sx + 2.0 * z, desk_top, z, 0.10, 0.08, 2.0, 0.22, 0.22, 0.26);
    // Monitor arm
    iso_block(cr, sx + 2.0 * z, desk_top - 2.0 * z, z, 0.04, 0.04, 8.0, 0.22, 0.22, 0.26);

    // ── Monitor screen (iso_block for 3D appearance) ──
    let mon_base = desk_top - 10.0 * z;
    iso_block(cr, sx + 2.0 * z, mon_base, z, 0.50, 0.08, 16.0, 0.12, 0.12, 0.15);

    // Screen face (on the front/right face of the monitor block)
    let mon_hw = TILE_W / 2.0 * z * 0.50;
    let mon_top = mon_base - 16.0 * z;
    let scr_x = sx + 2.0 * z + 1.5 * z;
    let scr_y = mon_top + 2.0 * z;
    let scr_w = mon_hw * 2.0 - 3.0 * z;
    let scr_h = 14.0 * z;
    cr.rectangle(scr_x - scr_w / 2.0, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.10, 0.14, 0.22);
    let _ = cr.fill();

    // IDE code lines on screen
    cr.set_line_width(1.0 * z);
    let colors = [
        (0.55, 0.85, 0.55, 0.8),
        (0.85, 0.75, 0.45, 0.8),
        (0.55, 0.70, 0.90, 0.8),
        (0.80, 0.55, 0.80, 0.7),
        (0.55, 0.85, 0.55, 0.6),
        (0.85, 0.65, 0.45, 0.7),
    ];
    for (i, (lr, lg, lb, la)) in colors.iter().enumerate() {
        let ly = scr_y + 1.5 * z + i as f64 * 2.0 * z;
        if ly > scr_y + scr_h - 1.0 * z {
            break;
        }
        let indent = if i % 3 != 0 { 3.0 * z } else { 1.5 * z };
        let lw = scr_w * (0.7 - (i % 3) as f64 * 0.1) - indent;
        cr.set_source_rgba(*lr, *lg, *lb, *la);
        cr.move_to(scr_x - scr_w / 2.0 + indent, ly);
        cr.line_to(scr_x - scr_w / 2.0 + indent + lw, ly);
        let _ = cr.stroke();
    }

    // Power LED
    cr.arc(
        scr_x + scr_w / 2.0 - 2.0 * z,
        scr_y + scr_h - 1.5 * z,
        0.6 * z,
        0.0,
        TAU,
    );
    cr.set_source_rgb(0.1, 0.85, 0.3);
    let _ = cr.fill();

    // Keyboard on desk surface
    iso_block(cr, sx + 2.0 * z, desk_top + 2.0 * z, z, 0.28, 0.12, 1.0, 0.20, 0.20, 0.23);

    // Mouse on desk surface
    iso_block(cr, sx + 12.0 * z, desk_top + 2.0 * z, z, 0.06, 0.05, 1.0, 0.22, 0.22, 0.25);

    // ── Office chair next to desk (dark navy) ──
    // Chair star base (flat wide block on floor)
    iso_block(cr, sx - 10.0 * z, sy + 8.0 * z, z, 0.22, 0.22, 2.0, 0.25, 0.25, 0.28);

    // Chair pedestal (thin cylinder approximated as small iso_block)
    iso_block(cr, sx - 10.0 * z, sy + 8.0 * z - 2.0 * z, z, 0.06, 0.06, 12.0, 0.30, 0.30, 0.33);

    // Chair seat (navy blue cushion)
    let chair_seat_sy = sy + 8.0 * z - 14.0 * z;
    iso_block(cr, sx - 10.0 * z, chair_seat_sy, z, 0.25, 0.25, 4.0, 0.18, 0.22, 0.35);

    // Chair backrest (taller navy block behind seat)
    let chair_back_sy = chair_seat_sy - 4.0 * z - 1.0 * z;
    iso_block(cr, sx - 12.0 * z, chair_back_sy, z, 0.22, 0.10, 14.0, 0.18, 0.22, 0.35);

    // Chair wheel dots (star base detail)
    cr.set_source_rgb(0.20, 0.20, 0.22);
    for angle_idx in 0..5 {
        let angle = angle_idx as f64 * TAU / 5.0;
        let wx = sx - 10.0 * z + angle.cos() * 5.0 * z;
        let wy = sy + 8.0 * z + angle.sin() * 2.5 * z;
        cr.arc(wx, wy, 1.2 * z, 0.0, TAU);
        let _ = cr.fill();
    }
}

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Vending machine: taller than a person (~58px)
    let height = 58.0;
    iso_block(cr, sx, sy, z, 0.6, 0.6, height, 0.72, 0.12, 0.12);

    let top = sy - height * z;
    let hw = TILE_W / 2.0 * z * 0.6;

    // Brand panel
    cr.rectangle(
        sx - hw + 2.0 * z,
        top + 2.0 * z,
        hw * 2.0 - 4.0 * z,
        6.0 * z,
    );
    cr.set_source_rgb(0.90, 0.18, 0.18);
    let _ = cr.fill();
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_font_size(4.5 * z);
    let _ = cr.move_to(sx - 7.0 * z, top + 7.0 * z);
    let _ = cr.show_text("DRINKS");

    // Glass panel
    cr.rectangle(
        sx - hw + 2.0 * z,
        top + 10.0 * z,
        hw * 2.0 - 4.0 * z,
        28.0 * z,
    );
    cr.set_source_rgba(0.25, 0.30, 0.35, 0.55);
    let _ = cr.fill();

    // Shelves with cans/bottles (5 rows now)
    let can_colors: [(f64, f64, f64); 6] = [
        (0.9, 0.2, 0.1),
        (0.1, 0.5, 0.9),
        (0.1, 0.7, 0.2),
        (0.9, 0.7, 0.0),
        (0.6, 0.1, 0.7),
        (0.9, 0.5, 0.1),
    ];
    for row in 0..5 {
        let ry = top + 11.0 * z + row as f64 * 5.5 * z;
        // Shelf
        cr.set_source_rgba(0.5, 0.5, 0.55, 0.5);
        cr.set_line_width(0.5 * z);
        cr.move_to(sx - hw + 3.0 * z, ry + 4.5 * z);
        cr.line_to(sx + hw - 3.0 * z, ry + 4.5 * z);
        let _ = cr.stroke();
        // Cans
        for col in 0..5 {
            let cx = sx - hw + 4.0 * z + col as f64 * 3.8 * z;
            let ci = (row * 3 + col) % can_colors.len();
            let (cr2, cg, cb) = can_colors[ci];
            cr.rectangle(cx, ry, 3.0 * z, 4.0 * z);
            cr.set_source_rgb(cr2, cg, cb);
            let _ = cr.fill();
            // Highlight
            cr.rectangle(cx + 0.3 * z, ry + 0.4 * z, 0.7 * z, 3.2 * z);
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
            let _ = cr.fill();
        }
    }

    // Dispensing slot
    cr.rectangle(sx - 6.0 * z, top + 40.0 * z, 12.0 * z, 6.0 * z);
    cr.set_source_rgb(0.06, 0.06, 0.08);
    let _ = cr.fill();

    // Coin slot and buttons on right side
    cr.arc(sx + hw - 4.0 * z, top + 42.0 * z, 1.5 * z, 0.0, TAU);
    cr.set_source_rgb(0.6, 0.55, 0.1);
    let _ = cr.fill();
    // Selection buttons
    for i in 0..3 {
        cr.arc(
            sx + hw - 4.0 * z,
            top + 46.0 * z + i as f64 * 3.0 * z,
            1.0 * z,
            0.0,
            TAU,
        );
        cr.set_source_rgb(0.4, 0.4, 0.42);
        let _ = cr.fill();
    }
}

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Kitchen counter: dark charcoal cabinets (0.15, 0.15, 0.18)
    // Full-width dark cabinet base
    iso_block(cr, sx, sy, z, 0.80, 0.65, 24.0, 0.15, 0.15, 0.18);

    let counter_top_y = sy - 24.0 * z;

    // Counter surface (lighter stone/marble top)
    iso_block(cr, sx, counter_top_y, z, 0.82, 0.67, 2.0, 0.35, 0.33, 0.30);
    let surface_y = counter_top_y - 2.0 * z;

    // Cabinet door lines (dark grooves)
    cr.set_source_rgba(0.08, 0.08, 0.10, 0.5);
    cr.set_line_width(0.5 * z);
    let cab_hw = TILE_W / 2.0 * z * 0.80;
    // Vertical divider on right face
    cr.move_to(sx + cab_hw * 0.5, sy - 12.0 * z);
    cr.line_to(sx + cab_hw * 0.5, sy);
    let _ = cr.stroke();

    // Cabinet handles (small bright dots)
    cr.set_source_rgb(0.55, 0.55, 0.58);
    cr.arc(sx + cab_hw * 0.3, sy - 10.0 * z, 0.8 * z, 0.0, TAU);
    let _ = cr.fill();
    cr.arc(sx + cab_hw * 0.7, sy - 10.0 * z, 0.8 * z, 0.0, TAU);
    let _ = cr.fill();

    // ── Coffee machine on counter (white appliance body) ──
    iso_block(cr, sx - 6.0 * z, surface_y, z, 0.22, 0.22, 18.0, 0.88, 0.88, 0.86);

    let machine_top = surface_y - 18.0 * z;

    // Machine display
    cr.rectangle(sx - 11.0 * z, machine_top + 3.0 * z, 8.0 * z, 4.5 * z);
    cr.set_source_rgb(0.08, 0.55, 0.75);
    let _ = cr.fill();
    // Display text
    cr.set_source_rgb(0.3, 0.9, 0.5);
    cr.set_font_size(3.0 * z);
    let _ = cr.move_to(sx - 10.0 * z, machine_top + 6.5 * z);
    let _ = cr.show_text("READY");

    // Drip area
    cr.rectangle(sx - 9.0 * z, machine_top + 12.0 * z, 5.0 * z, 3.0 * z);
    cr.set_source_rgb(0.15, 0.15, 0.18);
    let _ = cr.fill();

    // ── Cup on counter (white) ──
    let cup_x = sx + 8.0 * z;
    let cup_w = 5.0 * z;
    let cup_h = 6.0 * z;
    cr.rectangle(cup_x - cup_w / 2.0, surface_y - cup_h, cup_w, cup_h);
    cr.set_source_rgb(0.92, 0.92, 0.90);
    let _ = cr.fill();
    // Cup rim
    cr.save().unwrap();
    cr.translate(cup_x, surface_y - cup_h);
    cr.scale(1.0, 0.35);
    cr.arc(0.0, 0.0, cup_w / 2.0, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.85, 0.83);
    let _ = cr.fill();
    // Coffee inside
    cr.save().unwrap();
    cr.translate(cup_x, surface_y - cup_h + 0.5 * z);
    cr.scale(1.0, 0.35);
    cr.arc(0.0, 0.0, cup_w / 2.0 - 0.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.32, 0.18, 0.08);
    let _ = cr.fill();

    // Cup handle
    cr.set_source_rgb(0.88, 0.88, 0.86);
    cr.set_line_width(1.2 * z);
    cr.arc(
        cup_x + cup_w / 2.0 + 1.0 * z,
        surface_y - cup_h / 2.0,
        2.0 * z,
        -1.2,
        1.2,
    );
    let _ = cr.stroke();

    // Steam
    cr.set_source_rgba(0.9, 0.9, 0.9, 0.35);
    cr.set_line_width(1.2 * z);
    for offset in &[-1.5, 0.0, 1.5] {
        cr.move_to(cup_x + offset * z, surface_y - cup_h - 1.0 * z);
        cr.curve_to(
            cup_x + offset * z - 2.5 * z,
            surface_y - cup_h - 6.0 * z,
            cup_x + offset * z + 2.5 * z,
            surface_y - cup_h - 11.0 * z,
            cup_x + offset * z,
            surface_y - cup_h - 16.0 * z,
        );
        let _ = cr.stroke();
    }
}

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Wide navy blue couch with proper 3D depth
    // Navy color: (0.20, 0.25, 0.40)

    // ── Short stubby legs (iso_blocks) ──
    iso_block(cr, sx - 14.0 * z, sy + 2.0 * z, z, 0.05, 0.05, 6.0, 0.25, 0.20, 0.12);
    iso_block(cr, sx + 14.0 * z, sy - 2.0 * z, z, 0.05, 0.05, 6.0, 0.25, 0.20, 0.12);
    iso_block(cr, sx - 6.0 * z, sy + 8.0 * z, z, 0.05, 0.05, 6.0, 0.25, 0.20, 0.12);
    iso_block(cr, sx + 6.0 * z, sy + 6.0 * z, z, 0.05, 0.05, 6.0, 0.25, 0.20, 0.12);

    // ── Seat base (wide navy block, elevated on legs) ──
    let seat_sy = sy - 6.0 * z;
    iso_block(cr, sx, seat_sy, z, 0.85, 0.65, 8.0, 0.20, 0.25, 0.40);
    let seat_top_y = seat_sy - 8.0 * z;

    // ── Back rest (tall block behind seat, slightly offset to the back) ──
    iso_block(cr, sx - 4.0 * z, seat_top_y - 2.0 * z, z, 0.80, 0.20, 20.0, 0.18, 0.22, 0.38);

    // ── Left armrest ──
    iso_block(cr, sx - 16.0 * z, seat_top_y, z, 0.15, 0.55, 12.0, 0.16, 0.20, 0.35);

    // ── Right armrest ──
    iso_block(cr, sx + 16.0 * z, seat_top_y - 2.0 * z, z, 0.15, 0.55, 12.0, 0.16, 0.20, 0.35);

    // ── Seat cushion divisions (subtle lines on top) ──
    cr.set_source_rgba(0.12, 0.16, 0.28, 0.5);
    cr.set_line_width(0.8 * z);
    let chw = TILE_W / 2.0 * z * 0.85;
    let chh = TILE_H / 2.0 * z * 0.65;
    // Two cushion dividers
    for i in 1..3 {
        let t = i as f64 / 3.0;
        let cx = sx - chw + chw * 2.0 * t;
        let cy = seat_top_y + chh * (1.0 - 2.0 * t).abs();
        cr.move_to(cx, cy - 2.0 * z);
        cr.line_to(cx + 2.0 * z, cy + 3.0 * z);
        let _ = cr.stroke();
    }

    // Throw pillow (warm orange accent)
    cr.save().unwrap();
    cr.translate(sx - 6.0 * z, seat_top_y - 6.0 * z);
    cr.scale(1.0, 0.6);
    cr.arc(0.0, 0.0, 4.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.55, 0.25);
    let _ = cr.fill();
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Pot (tall planter using iso_block)
    iso_block(cr, sx, sy, z, 0.28, 0.28, 14.0, 0.55, 0.32, 0.16);
    // Pot rim (slightly wider)
    iso_block(cr, sx, sy - 14.0 * z, z, 0.32, 0.32, 2.0, 0.50, 0.28, 0.12);

    // Soil
    cr.save().unwrap();
    cr.translate(sx, sy - 16.0 * z);
    cr.scale(1.0, 0.45);
    cr.arc(0.0, 0.0, TILE_W / 2.0 * z * 0.27, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.28, 0.20, 0.09);
    let _ = cr.fill();

    // Trunk (thick, tall)
    cr.set_source_rgb(0.36, 0.24, 0.10);
    cr.rectangle(sx - 2.0 * z, sy - 38.0 * z, 4.0 * z, 22.0 * z);
    let _ = cr.fill();
    // Trunk texture
    cr.set_source_rgba(0.28, 0.18, 0.06, 0.4);
    cr.set_line_width(0.5 * z);
    for i in 0..4 {
        let ty = sy - 18.0 * z - i as f64 * 5.0 * z;
        cr.move_to(sx - 2.0 * z, ty);
        cr.line_to(sx + 2.0 * z, ty);
        let _ = cr.stroke();
    }

    // Branch
    cr.set_source_rgb(0.36, 0.24, 0.10);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx, sy - 34.0 * z);
    cr.line_to(sx + 8.0 * z, sy - 41.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx, sy - 30.0 * z);
    cr.line_to(sx - 6.0 * z, sy - 38.0 * z);
    let _ = cr.stroke();

    // Foliage: TURQUOISE/CYAN spheres (not green!)
    let leaves: [(f64, f64, f64, f64); 7] = [
        (0.0, -48.0, 13.0, 0.85),  // top center
        (-8.0, -42.0, 10.0, 0.75), // left mid
        (9.0, -44.0, 10.0, 0.90),  // right mid
        (-5.0, -52.0, 9.0, 0.95),  // top left
        (6.0, -50.0, 9.0, 0.80),   // top right
        (-10.0, -36.0, 8.0, 0.70), // lower left
        (11.0, -38.0, 8.0, 0.82),  // lower right
    ];
    for (dx, dy, r, shade) in &leaves {
        cr.arc(sx + dx * z, sy + dy * z, r * z, 0.0, TAU);
        // Turquoise/cyan: base (0.10, 0.70, 0.65)
        cr.set_source_rgb(0.10 * shade, 0.70 * shade, 0.65 * shade);
        let _ = cr.fill();
    }

    // Light highlights (cyan tinted)
    for &(dx, dy, r) in &[(1.0, -50.0, 4.5), (-3.0, -44.0, 3.5)] {
        cr.arc(sx + dx * z, sy + dy * z, r * z, 0.0, TAU);
        cr.set_source_rgba(0.30, 0.85, 0.80, 0.25);
        let _ = cr.fill();
    }
}

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Tall arcade/pinball cabinet using iso_block (~58px, roughly person height)
    let height = 58.0;
    iso_block(cr, sx, sy, z, 0.50, 0.50, height, 0.32, 0.08, 0.48);

    let top = sy - height * z;
    let hw = TILE_W / 2.0 * z * 0.5;

    // Marquee (lit header)
    cr.rectangle(
        sx - hw + 1.5 * z,
        top + 1.5 * z,
        hw * 2.0 - 3.0 * z,
        6.0 * z,
    );
    cr.set_source_rgb(1.0, 0.85, 0.1);
    let _ = cr.fill();
    cr.set_source_rgb(0.12, 0.04, 0.28);
    cr.set_font_size(4.0 * z);
    let _ = cr.move_to(sx - 7.0 * z, top + 6.5 * z);
    let _ = cr.show_text("ARCADE");

    // Screen bezel
    cr.rectangle(
        sx - hw + 2.0 * z,
        top + 9.0 * z,
        hw * 2.0 - 4.0 * z,
        18.0 * z,
    );
    cr.set_source_rgb(0.06, 0.06, 0.08);
    let _ = cr.fill();

    // CRT screen (glowing)
    let scr_x = sx - hw + 3.0 * z;
    let scr_y = top + 10.0 * z;
    let scr_w = hw * 2.0 - 6.0 * z;
    let scr_h = 16.0 * z;
    cr.rectangle(scr_x, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.04, 0.10, 0.04);
    let _ = cr.fill();

    // Screen glow effect
    cr.rectangle(scr_x - 1.0 * z, scr_y - 1.0 * z, scr_w + 2.0 * z, scr_h + 2.0 * z);
    cr.set_source_rgba(0.2, 0.9, 0.3, 0.08);
    let _ = cr.fill();

    // Game graphics
    cr.set_source_rgb(0.2, 0.9, 0.3);
    cr.set_font_size(5.0 * z);
    let _ = cr.move_to(scr_x + 2.0 * z, scr_y + 6.0 * z);
    let _ = cr.show_text("▼ ▼ ▼");
    cr.set_source_rgb(0.9, 0.9, 0.2);
    cr.set_font_size(4.0 * z);
    let _ = cr.move_to(scr_x + scr_w / 2.0 - 2.0 * z, scr_y + scr_h - 1.5 * z);
    let _ = cr.show_text("▲");

    // Score
    cr.set_source_rgb(0.9, 0.3, 0.3);
    cr.set_font_size(2.5 * z);
    let _ = cr.move_to(scr_x + 1.0 * z, scr_y + scr_h - 1.0 * z);
    let _ = cr.show_text("42100");

    // Scanlines
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.12);
    cr.set_line_width(0.5 * z);
    let mut sly = scr_y;
    while sly < scr_y + scr_h {
        cr.move_to(scr_x, sly);
        cr.line_to(scr_x + scr_w, sly);
        let _ = cr.stroke();
        sly += 1.5 * z;
    }

    // Control panel (angled)
    cr.rectangle(
        sx - hw + 2.0 * z,
        top + 28.0 * z,
        hw * 2.0 - 4.0 * z,
        8.0 * z,
    );
    cr.set_source_rgb(0.18, 0.18, 0.20);
    let _ = cr.fill();

    // Joystick
    cr.rectangle(sx - 3.0 * z, top + 29.0 * z, 2.0 * z, 5.0 * z);
    cr.set_source_rgb(0.12, 0.12, 0.14);
    let _ = cr.fill();
    cr.arc(sx - 2.0 * z, top + 29.0 * z, 2.0 * z, 0.0, TAU);
    cr.set_source_rgb(0.75, 0.1, 0.1);
    let _ = cr.fill();

    // Buttons
    let btn_colors = [
        (1.0, 0.2, 0.2),
        (0.2, 0.2, 1.0),
        (0.2, 0.8, 0.2),
        (0.9, 0.9, 0.1),
    ];
    for (i, (br, bg, bb)) in btn_colors.iter().enumerate() {
        let bx = sx + 2.0 * z + i as f64 * 3.0 * z;
        cr.arc(bx, top + 32.0 * z, 1.3 * z, 0.0, TAU);
        cr.set_source_rgb(*br, *bg, *bb);
        let _ = cr.fill();
        cr.arc(bx - 0.3 * z, top + 31.7 * z, 0.5 * z, 0.0, TAU);
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.3);
        let _ = cr.fill();
    }

    // Coin slot
    cr.rectangle(sx - 1.5 * z, top + 42.0 * z, 3.0 * z, 1.5 * z);
    cr.set_source_rgb(0.6, 0.55, 0.1);
    let _ = cr.fill();

    // Speaker grille
    cr.set_source_rgba(0.2, 0.2, 0.22, 0.6);
    cr.set_line_width(0.5 * z);
    for i in 0..4 {
        let gy = top + 45.0 * z + i as f64 * 2.0 * z;
        cr.move_to(sx - 4.0 * z, gy);
        cr.line_to(sx + 4.0 * z, gy);
        let _ = cr.stroke();
    }
}

fn draw_treadmill(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Dark base platform (iso_block)
    iso_block(cr, sx, sy, z, 0.80, 0.50, 10.0, 0.18, 0.18, 0.20);

    let belt_top = sy - 10.0 * z;

    // Belt surface (slightly different shade on top)
    iso_block(cr, sx, belt_top, z, 0.75, 0.45, 2.0, 0.22, 0.22, 0.24);

    // Belt treads
    cr.set_source_rgba(0.16, 0.16, 0.18, 0.6);
    cr.set_line_width(0.8 * z);
    for i in 0..6 {
        let bx = sx - 10.0 * z + i as f64 * 4.0 * z;
        cr.move_to(bx, belt_top - 1.0 * z);
        cr.line_to(bx + 2.0 * z, belt_top + 1.5 * z);
        let _ = cr.stroke();
    }

    // Upright posts (metallic, tall handles)
    let post_h = 40.0 * z;
    cr.set_source_rgb(0.52, 0.52, 0.55);
    cr.set_line_width(3.0 * z);
    cr.move_to(sx - 11.0 * z, belt_top);
    cr.line_to(sx - 11.0 * z, belt_top - post_h);
    let _ = cr.stroke();
    cr.move_to(sx + 11.0 * z, belt_top);
    cr.line_to(sx + 11.0 * z, belt_top - post_h);
    let _ = cr.stroke();

    // Top handlebar
    cr.set_line_width(2.5 * z);
    cr.move_to(sx - 11.0 * z, belt_top - post_h);
    cr.line_to(sx + 11.0 * z, belt_top - post_h);
    let _ = cr.stroke();

    // Grip wraps (rubber)
    cr.set_source_rgb(0.15, 0.15, 0.17);
    cr.set_line_width(4.0 * z);
    cr.move_to(sx - 11.0 * z, belt_top - post_h + 4.0 * z);
    cr.line_to(sx - 11.0 * z, belt_top - post_h + 8.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 11.0 * z, belt_top - post_h + 4.0 * z);
    cr.line_to(sx + 11.0 * z, belt_top - post_h + 8.0 * z);
    let _ = cr.stroke();

    // Console display (between handles at top)
    let console_w = 14.0 * z;
    let console_h = 8.0 * z;
    let console_y = belt_top - post_h - 2.0 * z;
    cr.rectangle(sx - console_w / 2.0, console_y, console_w, console_h);
    cr.set_source_rgb(0.12, 0.12, 0.14);
    let _ = cr.fill();
    // Screen
    cr.rectangle(
        sx - console_w / 2.0 + 1.0 * z,
        console_y + 1.0 * z,
        console_w - 2.0 * z,
        console_h - 2.0 * z,
    );
    cr.set_source_rgb(0.08, 0.55, 0.28);
    let _ = cr.fill();
    // Speed readout
    cr.set_source_rgb(0.2, 0.95, 0.4);
    cr.set_font_size(3.5 * z);
    let _ = cr.move_to(sx - 4.0 * z, console_y + console_h - 2.0 * z);
    let _ = cr.show_text("5.2");
    // Heart icon
    cr.set_source_rgb(0.9, 0.2, 0.2);
    cr.set_font_size(2.5 * z);
    let _ = cr.move_to(sx + 2.0 * z, console_y + console_h - 2.0 * z);
    let _ = cr.show_text("♥");

    // Side rails (lower hand holds)
    cr.set_source_rgb(0.48, 0.48, 0.50);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 11.0 * z, belt_top - 18.0 * z);
    cr.line_to(sx - 13.0 * z, belt_top - 8.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 11.0 * z, belt_top - 18.0 * z);
    cr.line_to(sx + 13.0 * z, belt_top - 8.0 * z);
    let _ = cr.stroke();
}

fn draw_whiteboard(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Stand legs (A-frame, tall — board top ~48px from floor)
    cr.set_source_rgb(0.42, 0.42, 0.45);
    cr.set_line_width(2.5 * z);
    // Left pair
    cr.move_to(sx - 14.0 * z, sy + 2.0 * z);
    cr.line_to(sx - 10.0 * z, sy - 46.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx - 6.0 * z, sy + 2.0 * z);
    cr.line_to(sx - 10.0 * z, sy - 46.0 * z);
    let _ = cr.stroke();
    // Right pair
    cr.move_to(sx + 14.0 * z, sy + 2.0 * z);
    cr.line_to(sx + 10.0 * z, sy - 46.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 6.0 * z, sy + 2.0 * z);
    cr.line_to(sx + 10.0 * z, sy - 46.0 * z);
    let _ = cr.stroke();

    // Cross brace
    cr.set_line_width(1.5 * z);
    cr.move_to(sx - 10.0 * z, sy - 20.0 * z);
    cr.line_to(sx + 10.0 * z, sy - 20.0 * z);
    let _ = cr.stroke();

    // Board frame
    let board_x = sx - 16.0 * z;
    let board_y = sy - 48.0 * z;
    let board_w = 32.0 * z;
    let board_h = 22.0 * z;
    cr.rectangle(
        board_x - 1.5 * z,
        board_y - 1.5 * z,
        board_w + 3.0 * z,
        board_h + 3.0 * z,
    );
    cr.set_source_rgb(0.48, 0.48, 0.50);
    let _ = cr.fill();

    // Board surface
    cr.rectangle(board_x, board_y, board_w, board_h);
    cr.set_source_rgb(0.96, 0.96, 0.97);
    let _ = cr.fill();

    // Content: architecture diagram
    // Title
    cr.set_source_rgb(0.12, 0.30, 0.70);
    cr.set_line_width(1.5 * z);
    cr.move_to(board_x + 3.0 * z, board_y + 4.0 * z);
    cr.line_to(board_x + 18.0 * z, board_y + 4.0 * z);
    let _ = cr.stroke();

    // Boxes
    cr.set_source_rgb(0.75, 0.18, 0.18);
    cr.set_line_width(1.2 * z);
    cr.rectangle(board_x + 3.0 * z, board_y + 6.0 * z, 8.0 * z, 4.5 * z);
    let _ = cr.stroke();
    cr.rectangle(board_x + 18.0 * z, board_y + 6.0 * z, 8.0 * z, 4.5 * z);
    let _ = cr.stroke();

    // Arrow between boxes
    cr.move_to(board_x + 11.0 * z, board_y + 8.0 * z);
    cr.line_to(board_x + 18.0 * z, board_y + 8.0 * z);
    let _ = cr.stroke();
    // Arrowhead
    cr.move_to(board_x + 18.0 * z, board_y + 8.0 * z);
    cr.line_to(board_x + 16.0 * z, board_y + 7.0 * z);
    let _ = cr.stroke();
    cr.move_to(board_x + 18.0 * z, board_y + 8.0 * z);
    cr.line_to(board_x + 16.0 * z, board_y + 9.0 * z);
    let _ = cr.stroke();

    // Bottom box
    cr.rectangle(board_x + 10.0 * z, board_y + 13.0 * z, 10.0 * z, 4.0 * z);
    let _ = cr.stroke();
    // Vertical arrows to bottom box
    cr.move_to(board_x + 7.0 * z, board_y + 10.5 * z);
    cr.line_to(board_x + 12.0 * z, board_y + 13.0 * z);
    let _ = cr.stroke();
    cr.move_to(board_x + 22.0 * z, board_y + 10.5 * z);
    cr.line_to(board_x + 18.0 * z, board_y + 13.0 * z);
    let _ = cr.stroke();

    // Green notes
    cr.set_source_rgb(0.1, 0.55, 0.18);
    cr.set_line_width(0.8 * z);
    cr.move_to(board_x + 3.0 * z, board_y + 19.0 * z);
    cr.line_to(board_x + 14.0 * z, board_y + 19.0 * z);
    let _ = cr.stroke();
    cr.move_to(board_x + 3.0 * z, board_y + 21.0 * z);
    cr.line_to(board_x + 11.0 * z, board_y + 21.0 * z);
    let _ = cr.stroke();

    // Marker tray
    cr.rectangle(
        board_x + 3.0 * z,
        board_y + board_h + 1.0 * z,
        board_w - 6.0 * z,
        2.5 * z,
    );
    cr.set_source_rgb(0.42, 0.42, 0.45);
    let _ = cr.fill();

    // Markers
    let marker_colors = [
        (0.08, 0.08, 0.75),
        (0.75, 0.08, 0.08),
        (0.08, 0.55, 0.08),
        (0.0, 0.0, 0.0),
    ];
    for (i, (mr, mg, mb)) in marker_colors.iter().enumerate() {
        cr.rectangle(
            board_x + 5.0 * z + i as f64 * 4.0 * z,
            board_y + board_h + 1.0 * z,
            3.0 * z,
            2.0 * z,
        );
        cr.set_source_rgb(*mr, *mg, *mb);
        let _ = cr.fill();
    }

    // Eraser
    cr.rectangle(
        board_x + board_w - 10.0 * z,
        board_y + board_h + 1.0 * z,
        5.0 * z,
        2.0 * z,
    );
    cr.set_source_rgb(0.9, 0.9, 0.85);
    let _ = cr.fill();
}

fn draw_weight_bench(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Metal frame base (low wide block)
    iso_block(cr, sx, sy, z, 0.65, 0.35, 4.0, 0.35, 0.35, 0.38);

    // Frame uprights (two short metal blocks supporting the bench)
    iso_block(cr, sx - 8.0 * z, sy - 1.0 * z, z, 0.08, 0.08, 10.0, 0.38, 0.38, 0.40);
    iso_block(cr, sx + 8.0 * z, sy - 1.0 * z, z, 0.08, 0.08, 10.0, 0.38, 0.38, 0.40);

    // Bench pad (dark, elevated on frame)
    let bench_sy = sy - 10.0 * z;
    iso_block(cr, sx, bench_sy, z, 0.60, 0.25, 5.0, 0.12, 0.12, 0.14);

    // Rack uprights (two tall metal posts)
    let rack_h = 44.0;
    iso_block(cr, sx - 10.0 * z, bench_sy - 5.0 * z, z, 0.05, 0.05, rack_h, 0.40, 0.40, 0.42);
    iso_block(cr, sx + 10.0 * z, bench_sy - 5.0 * z, z, 0.05, 0.05, rack_h, 0.40, 0.40, 0.42);

    // Rack hooks (small blocks)
    let hook_y = bench_sy - 5.0 * z - rack_h * z + 4.0 * z;
    iso_block(cr, sx - 7.0 * z, hook_y, z, 0.08, 0.04, 2.0, 0.42, 0.42, 0.44);
    iso_block(cr, sx + 7.0 * z, hook_y, z, 0.08, 0.04, 2.0, 0.42, 0.42, 0.44);

    // Barbell (resting on rack)
    let bar_y = hook_y - 2.0 * z;
    cr.set_source_rgb(0.50, 0.50, 0.52);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 16.0 * z, bar_y);
    cr.line_to(sx + 16.0 * z, bar_y);
    let _ = cr.stroke();

    // Weight plates (circles on each end)
    for &dx in &[-15.0, -13.0, 13.0, 15.0] {
        cr.arc(sx + dx * z, bar_y, 4.0 * z, 0.0, TAU);
        cr.set_source_rgb(0.15, 0.15, 0.18);
        let _ = cr.fill();
    }
    // Inner plate highlights
    for &dx in &[-15.0, -13.0, 13.0, 15.0] {
        cr.arc(sx + dx * z, bar_y, 2.0 * z, 0.0, TAU);
        cr.set_source_rgba(0.25, 0.25, 0.28, 0.5);
        let _ = cr.fill();
    }
}

fn draw_yoga_mat(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Flat mat on ground — isometric rectangle, slightly raised
    let hw = TILE_W / 2.0 * z * 0.75;
    let hh = TILE_H / 2.0 * z * 0.90;

    // Mat shadow
    cr.move_to(sx, sy - hh + 1.0 * z);
    cr.line_to(sx + hw, sy + 1.0 * z);
    cr.line_to(sx, sy + hh + 1.0 * z);
    cr.line_to(sx - hw, sy + 1.0 * z);
    cr.close_path();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
    let _ = cr.fill();

    // Mat surface (bright color — teal/purple alternating via position hash)
    let color_pick = ((sx * 7.0 + sy * 13.0) as i32).unsigned_abs() % 3;
    let (mr, mg, mb) = match color_pick {
        0 => (0.15, 0.65, 0.60), // teal
        1 => (0.55, 0.30, 0.65), // purple
        _ => (0.20, 0.55, 0.75), // blue
    };

    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(mr, mg, mb);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(mr * 0.7, mg * 0.7, mb * 0.7);
    cr.set_line_width(0.5);
    let _ = cr.stroke();

    // Mat texture lines (lengthwise)
    cr.set_source_rgba(mr * 1.15, mg * 1.15, mb * 1.15, 0.3);
    cr.set_line_width(0.5 * z);
    for i in 1..4 {
        let t = i as f64 / 4.0;
        let x0 = sx - hw * (1.0 - t);
        let y0 = sy - hh * (1.0 - t) + hh * t;
        let x1 = sx + hw * t;
        let y1 = sy - hh * t + hh * (1.0 - t);
        cr.move_to(x0, y0);
        cr.line_to(x1, y1);
        let _ = cr.stroke();
    }

    // Rolled end (small cylinder at one end)
    cr.save().unwrap();
    cr.translate(sx - hw * 0.8, sy - hh * 0.8);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 3.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(mr * 0.8, mg * 0.8, mb * 0.8);
    let _ = cr.fill();
}

fn draw_floor_lamp(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Heavy round base (iso_block)
    iso_block(cr, sx, sy, z, 0.20, 0.20, 3.0, 0.40, 0.40, 0.42);

    // Thin metallic pole (~50px tall)
    let pole_h = 52.0 * z;
    cr.set_source_rgb(0.50, 0.50, 0.52);
    cr.rectangle(sx - 1.0 * z, sy - pole_h, 2.0 * z, pole_h - 3.0 * z);
    let _ = cr.fill();

    // Warm glow circle around lamp top
    cr.save().unwrap();
    cr.translate(sx, sy - pole_h - 2.0 * z);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 14.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(1.0, 0.92, 0.65, 0.15);
    let _ = cr.fill();

    // Lampshade (trapezoid, warm yellow)
    let shade_top = sy - pole_h - 5.0 * z;
    let shade_bot = sy - pole_h + 7.0 * z;
    let top_hw = 5.0 * z;
    let bot_hw = 11.0 * z;
    cr.move_to(sx - top_hw, shade_top);
    cr.line_to(sx + top_hw, shade_top);
    cr.line_to(sx + bot_hw, shade_bot);
    cr.line_to(sx - bot_hw, shade_bot);
    cr.close_path();
    cr.set_source_rgb(0.95, 0.88, 0.55);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(0.80, 0.72, 0.42);
    cr.set_line_width(0.8 * z);
    let _ = cr.stroke();

    // Inner glow on shade
    cr.move_to(sx - top_hw + 1.5 * z, shade_top + 1.0 * z);
    cr.line_to(sx + top_hw - 1.5 * z, shade_top + 1.0 * z);
    cr.line_to(sx + bot_hw - 2.0 * z, shade_bot - 1.0 * z);
    cr.line_to(sx - bot_hw + 2.0 * z, shade_bot - 1.0 * z);
    cr.close_path();
    cr.set_source_rgba(1.0, 0.95, 0.70, 0.45);
    let _ = cr.fill();
}

fn draw_ping_pong_table(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let leg_elev = 22.0;

    // 4 legs as iso_blocks
    iso_block(cr, sx - 12.0 * z, sy + 2.0 * z, z, 0.06, 0.06, leg_elev, 0.35, 0.35, 0.38);
    iso_block(cr, sx + 12.0 * z, sy - 2.0 * z, z, 0.06, 0.06, leg_elev, 0.35, 0.35, 0.38);
    iso_block(cr, sx - 4.0 * z, sy + 7.0 * z, z, 0.06, 0.06, leg_elev, 0.35, 0.35, 0.38);
    iso_block(cr, sx + 4.0 * z, sy + 5.0 * z, z, 0.06, 0.06, leg_elev, 0.35, 0.35, 0.38);

    // Table surface (green slab elevated on legs)
    let table_sy = sy - leg_elev * z;
    iso_block(cr, sx, table_sy, z, 0.85, 0.60, 3.0, 0.15, 0.55, 0.25);
    let table_top = table_sy - 3.0 * z;

    // White border lines on table top
    cr.set_source_rgb(0.95, 0.95, 0.95);
    cr.set_line_width(0.8 * z);
    let thw = TILE_W / 2.0 * z * 0.85 * 0.92;
    let thh = TILE_H / 2.0 * z * 0.60 * 0.92;
    // Outline
    cr.move_to(sx, table_top - thh + 1.0 * z);
    cr.line_to(sx + thw, table_top + 1.0 * z);
    cr.line_to(sx, table_top + thh + 1.0 * z);
    cr.line_to(sx - thw, table_top + 1.0 * z);
    cr.close_path();
    let _ = cr.stroke();

    // Center line
    cr.set_line_width(1.0 * z);
    cr.move_to(sx - thw * 0.5, table_top + 1.0 * z - thh * 0.5);
    cr.line_to(sx + thw * 0.5, table_top + 1.0 * z + thh * 0.5);
    let _ = cr.stroke();

    // Net posts (two small vertical lines at center)
    cr.set_source_rgb(0.40, 0.40, 0.42);
    cr.set_line_width(1.5 * z);
    let net_y = table_top + 1.0 * z;
    cr.move_to(sx - thw * 0.5, net_y - thh * 0.5);
    cr.line_to(sx - thw * 0.5, net_y - thh * 0.5 - 5.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + thw * 0.5, net_y + thh * 0.5);
    cr.line_to(sx + thw * 0.5, net_y + thh * 0.5 - 5.0 * z);
    let _ = cr.stroke();

    // Net (thin line between posts)
    cr.set_source_rgba(0.90, 0.90, 0.90, 0.7);
    cr.set_line_width(0.5 * z);
    cr.move_to(sx - thw * 0.5, net_y - thh * 0.5 - 4.0 * z);
    cr.line_to(sx + thw * 0.5, net_y + thh * 0.5 - 4.0 * z);
    let _ = cr.stroke();
    // Net mesh lines
    for i in 1..5 {
        let t = i as f64 / 5.0;
        let nx = sx - thw * 0.5 + thw * t;
        let ny_top = net_y - thh * 0.5 - 4.0 * z + thh * t;
        let ny_bot = net_y - thh * 0.5 + thh * t;
        cr.move_to(nx, ny_top);
        cr.line_to(nx, ny_bot);
        let _ = cr.stroke();
    }
}

fn draw_small_armchair(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Blocky 3D armchair in orange-red (0.80, 0.35, 0.15)

    // ── Short stubby legs (iso_blocks) ──
    iso_block(cr, sx - 8.0 * z, sy + 1.0 * z, z, 0.05, 0.05, 6.0, 0.30, 0.22, 0.12);
    iso_block(cr, sx + 8.0 * z, sy - 1.0 * z, z, 0.05, 0.05, 6.0, 0.30, 0.22, 0.12);
    iso_block(cr, sx - 4.0 * z, sy + 5.0 * z, z, 0.05, 0.05, 6.0, 0.30, 0.22, 0.12);
    iso_block(cr, sx + 4.0 * z, sy + 4.0 * z, z, 0.05, 0.05, 6.0, 0.30, 0.22, 0.12);

    // ── Seat cushion (blocky, elevated on legs) ──
    let seat_sy = sy - 6.0 * z;
    iso_block(cr, sx, seat_sy, z, 0.50, 0.50, 8.0, 0.80, 0.35, 0.15);
    let seat_top_y = seat_sy - 8.0 * z;

    // ── Backrest (tall block behind seat) ──
    iso_block(cr, sx - 3.0 * z, seat_top_y - 1.0 * z, z, 0.45, 0.15, 16.0, 0.75, 0.32, 0.13);

    // ── Left armrest ──
    iso_block(cr, sx - 10.0 * z, seat_top_y, z, 0.10, 0.40, 10.0, 0.72, 0.30, 0.12);

    // ── Right armrest ──
    iso_block(cr, sx + 10.0 * z, seat_top_y - 1.0 * z, z, 0.10, 0.40, 10.0, 0.72, 0.30, 0.12);

    // Seat cushion detail (tufting circle)
    cr.save().unwrap();
    cr.translate(sx, seat_top_y - 1.5 * z);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 4.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(0.85, 0.40, 0.18, 0.35);
    let _ = cr.fill();
}
