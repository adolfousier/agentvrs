use crate::gui::iso::{TILE_H, TILE_W, WALL_HEIGHT};
use crate::world::{FloorKind, Tile, WallKind};
use std::f64::consts::TAU;

/// Agent reference: total height ~54px at zoom=1.
/// All furniture scaled relative to that (desk=waist, vending=taller than person, etc.)

pub fn draw_tile(cr: &gtk4::cairo::Context, sx: f64, sy: f64, tile: &Tile, zoom: f64) {
    match tile {
        Tile::Floor(kind) => draw_floor(cr, sx, sy, kind, zoom),
        Tile::Wall(kind) => draw_wall(cr, sx, sy, kind, zoom),
        Tile::DoorOpen => draw_door(cr, sx, sy, zoom),
        Tile::Rug => draw_rug(cr, sx, sy, zoom),
        Tile::Desk => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.65, 0.45, 0.25);
            draw_desk(cr, sx, sy, zoom);
        }
        Tile::VendingMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_vending(cr, sx, sy, zoom);
        }
        Tile::CoffeeMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_coffee(cr, sx, sy, zoom);
        }
        Tile::Couch => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_couch(cr, sx, sy, zoom);
        }
        Tile::Plant => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_plant(cr, sx, sy, zoom);
        }
        Tile::PinballMachine => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_arcade(cr, sx, sy, zoom);
        }
        Tile::GymTreadmill => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_treadmill(cr, sx, sy, zoom);
        }
        Tile::Whiteboard => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.5, 0.5, 0.52);
            draw_whiteboard(cr, sx, sy, zoom);
        }
    }
}

// ─── Floor ───

fn draw_floor(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &FloorKind, zoom: f64) {
    let (r, g, b) = match kind {
        FloorKind::Wood => (0.72, 0.52, 0.30),
        FloorKind::Tile => (0.82, 0.82, 0.86),
        FloorKind::Carpet => (0.30, 0.30, 0.52),
        FloorKind::Concrete => (0.55, 0.55, 0.57),
    };
    draw_floor_diamond(cr, sx, sy, zoom, r, g, b);

    match kind {
        FloorKind::Wood => {
            cr.set_source_rgba(0.5, 0.35, 0.18, 0.3);
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
            cr.set_source_rgba(0.7, 0.7, 0.74, 0.4);
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

fn draw_floor_diamond(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64, r: f64, g: f64, b: f64) {
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
        WallKind::Solid => (0.62, 0.60, 0.58),
        WallKind::Window => (0.60, 0.62, 0.65),
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
        cr.set_source_rgba(0.4, 0.38, 0.36, 0.3);
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
    draw_floor_diamond(cr, sx, sy, zoom, 0.50, 0.50, 0.52);
    let hw = TILE_W / 2.0 * zoom * 0.55;
    let hh = TILE_H / 2.0 * zoom * 0.55;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(0.42, 0.42, 0.44);
    let _ = cr.fill();
}

fn draw_rug(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.62, 0.16, 0.16);
    // Border
    let hw = TILE_W / 2.0 * zoom * 0.85;
    let hh = TILE_H / 2.0 * zoom * 0.85;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgba(0.85, 0.65, 0.15, 0.6);
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
    cr.set_source_rgb(0.78, 0.28, 0.12);
    let _ = cr.fill();
    // Medallion
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 5.0 * zoom, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.65, 0.15);
    let _ = cr.fill();
}

// ─── Isometric block helper ───

fn iso_block(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64, w_ratio: f64, h_ratio: f64, height: f64, r: f64, g: f64, b: f64) {
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
    // Desk legs (4 thin columns, waist height ~24px)
    let leg_h = 22.0;
    let hw = TILE_W / 2.0 * z * 0.7;
    let hh = TILE_H / 2.0 * z * 0.7;
    let leg_w = 2.0 * z;
    cr.set_source_rgb(0.40, 0.28, 0.14);
    for &(lx, ly) in &[
        (sx - hw + leg_w, sy - hh * 0.3),
        (sx + hw - leg_w, sy - hh * 0.3),
        (sx - hw * 0.3, sy + hh - leg_w),
        (sx + hw * 0.3, sy + hh - leg_w),
    ] {
        cr.rectangle(lx - leg_w / 2.0, ly - leg_h * z, leg_w, leg_h * z);
        let _ = cr.fill();
    }

    // Table surface (waist height = ~24px)
    iso_block(cr, sx, sy, z, 0.75, 0.75, 24.0, 0.58, 0.38, 0.18);

    let desk_top = sy - 24.0 * z;

    // Keyboard
    let kb_w = 12.0 * z;
    let kb_h = 3.0 * z;
    cr.move_to(sx - kb_w / 2.0, desk_top - 0.5 * z);
    cr.line_to(sx + kb_w / 2.0, desk_top - 0.5 * z);
    cr.line_to(sx + kb_w / 2.0 - 1.0 * z, desk_top - 0.5 * z - kb_h);
    cr.line_to(sx - kb_w / 2.0 + 1.0 * z, desk_top - 0.5 * z - kb_h);
    cr.close_path();
    cr.set_source_rgb(0.20, 0.20, 0.23);
    let _ = cr.fill();
    // Keys
    cr.set_source_rgba(0.40, 0.40, 0.45, 0.7);
    for row in 0..2 {
        for i in 0..6 {
            let kx = sx - 5.0 * z + i as f64 * 1.8 * z;
            let ky = desk_top - 1.5 * z - row as f64 * 1.4 * z;
            cr.rectangle(kx, ky, 1.2 * z, 0.9 * z);
            let _ = cr.fill();
        }
    }

    // Mouse (right of keyboard)
    cr.save().unwrap();
    cr.translate(sx + 8.0 * z, desk_top - 1.5 * z);
    cr.scale(1.0, 0.6);
    cr.arc(0.0, 0.0, 1.8 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.22, 0.22, 0.25);
    let _ = cr.fill();

    // Monitor stand
    cr.rectangle(sx - 2.0 * z, desk_top - 8.0 * z, 4.0 * z, 8.0 * z);
    cr.set_source_rgb(0.25, 0.25, 0.28);
    let _ = cr.fill();
    // Stand base
    cr.save().unwrap();
    cr.translate(sx, desk_top);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 5.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.22, 0.22, 0.26);
    let _ = cr.fill();

    // Monitor (24" equivalent: ~24px wide, ~16px tall)
    let mon_w = 24.0 * z;
    let mon_h = 16.0 * z;
    let mon_y = desk_top - 8.0 * z - mon_h;

    // Bezel back shadow
    cr.rectangle(sx - mon_w / 2.0 - 0.8 * z, mon_y - 0.8 * z, mon_w + 1.6 * z, mon_h + 1.6 * z);
    cr.set_source_rgb(0.10, 0.10, 0.12);
    let _ = cr.fill();
    // Bezel
    cr.rectangle(sx - mon_w / 2.0, mon_y, mon_w, mon_h);
    cr.set_source_rgb(0.15, 0.15, 0.18);
    let _ = cr.fill();

    // Screen
    let b = 1.8 * z;
    let scr_x = sx - mon_w / 2.0 + b;
    let scr_y = mon_y + b;
    let scr_w = mon_w - b * 2.0;
    let scr_h = mon_h - b * 2.5;
    cr.rectangle(scr_x, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.10, 0.14, 0.22);
    let _ = cr.fill();

    // IDE code lines
    cr.set_line_width(1.2 * z);
    let colors: [(f64, f64, f64, f64); 7] = [
        (0.55, 0.85, 0.55, 0.8),
        (0.85, 0.75, 0.45, 0.8),
        (0.55, 0.70, 0.90, 0.8),
        (0.80, 0.55, 0.80, 0.7),
        (0.55, 0.85, 0.55, 0.6),
        (0.85, 0.65, 0.45, 0.7),
        (0.55, 0.70, 0.90, 0.6),
    ];
    for (i, (lr, lg, lb, la)) in colors.iter().enumerate() {
        let ly = scr_y + 2.0 * z + i as f64 * 2.0 * z;
        if ly > scr_y + scr_h - 1.0 * z { break; }
        let indent = if i % 3 == 1 || i % 3 == 2 { 4.0 * z } else { 1.5 * z };
        let lw = scr_w * (0.78 - (i % 4) as f64 * 0.1) - indent;
        cr.set_source_rgba(*lr, *lg, *lb, *la);
        cr.move_to(scr_x + indent, ly);
        cr.line_to(scr_x + indent + lw, ly);
        let _ = cr.stroke();
    }

    // Line numbers gutter
    cr.set_source_rgba(0.4, 0.4, 0.5, 0.4);
    cr.rectangle(scr_x, scr_y, 3.0 * z, scr_h);
    let _ = cr.fill();

    // Power LED
    cr.arc(sx + mon_w / 2.0 - 3.0 * z, mon_y + mon_h - 2.0 * z, 0.8 * z, 0.0, TAU);
    cr.set_source_rgb(0.1, 0.85, 0.3);
    let _ = cr.fill();
}

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Vending machine: taller than a person (~58px)
    let height = 58.0;
    iso_block(cr, sx, sy, z, 0.6, 0.6, height, 0.72, 0.12, 0.12);

    let top = sy - height * z;
    let hw = TILE_W / 2.0 * z * 0.6;

    // Brand panel
    cr.rectangle(sx - hw + 2.0 * z, top + 2.0 * z, hw * 2.0 - 4.0 * z, 6.0 * z);
    cr.set_source_rgb(0.90, 0.18, 0.18);
    let _ = cr.fill();
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_font_size(4.5 * z);
    let _ = cr.move_to(sx - 7.0 * z, top + 7.0 * z);
    let _ = cr.show_text("DRINKS");

    // Glass panel
    cr.rectangle(sx - hw + 2.0 * z, top + 10.0 * z, hw * 2.0 - 4.0 * z, 28.0 * z);
    cr.set_source_rgba(0.25, 0.30, 0.35, 0.55);
    let _ = cr.fill();

    // Shelves with cans/bottles (5 rows now)
    let can_colors: [(f64, f64, f64); 6] = [
        (0.9, 0.2, 0.1), (0.1, 0.5, 0.9), (0.1, 0.7, 0.2),
        (0.9, 0.7, 0.0), (0.6, 0.1, 0.7), (0.9, 0.5, 0.1),
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
        cr.arc(sx + hw - 4.0 * z, top + 46.0 * z + i as f64 * 3.0 * z, 1.0 * z, 0.0, TAU);
        cr.set_source_rgb(0.4, 0.4, 0.42);
        let _ = cr.fill();
    }
}

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Counter (~waist height, 24px)
    iso_block(cr, sx, sy, z, 0.65, 0.65, 24.0, 0.52, 0.50, 0.47);

    let counter_top = sy - 24.0 * z;

    // Coffee machine on counter (~20px tall on counter = 44px total)
    iso_block(cr, sx - 4.0 * z, counter_top + 4.0 * z, z, 0.35, 0.35, 22.0, 0.22, 0.22, 0.25);

    let machine_top = counter_top + 4.0 * z - 22.0 * z;

    // Machine display
    cr.rectangle(sx - 9.0 * z, machine_top + 3.0 * z, 8.0 * z, 4.5 * z);
    cr.set_source_rgb(0.08, 0.55, 0.75);
    let _ = cr.fill();
    // Display text
    cr.set_source_rgb(0.3, 0.9, 0.5);
    cr.set_font_size(3.0 * z);
    let _ = cr.move_to(sx - 8.0 * z, machine_top + 6.5 * z);
    let _ = cr.show_text("READY");

    // Drip area
    cr.rectangle(sx - 7.0 * z, machine_top + 12.0 * z, 5.0 * z, 3.0 * z);
    cr.set_source_rgb(0.15, 0.15, 0.18);
    let _ = cr.fill();

    // Cup on counter
    let cup_x = sx + 6.0 * z;
    let cup_w = 5.0 * z;
    let cup_h = 6.0 * z;
    cr.rectangle(cup_x - cup_w / 2.0, counter_top - cup_h, cup_w, cup_h);
    cr.set_source_rgb(0.92, 0.92, 0.90);
    let _ = cr.fill();
    // Cup rim
    cr.save().unwrap();
    cr.translate(cup_x, counter_top - cup_h);
    cr.scale(1.0, 0.35);
    cr.arc(0.0, 0.0, cup_w / 2.0, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.85, 0.83);
    let _ = cr.fill();
    // Coffee inside
    cr.save().unwrap();
    cr.translate(cup_x, counter_top - cup_h + 0.5 * z);
    cr.scale(1.0, 0.35);
    cr.arc(0.0, 0.0, cup_w / 2.0 - 0.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.32, 0.18, 0.08);
    let _ = cr.fill();

    // Cup handle
    cr.set_source_rgb(0.88, 0.88, 0.86);
    cr.set_line_width(1.2 * z);
    cr.arc(cup_x + cup_w / 2.0 + 1.0 * z, counter_top - cup_h / 2.0, 2.0 * z, -1.2, 1.2);
    let _ = cr.stroke();

    // Steam
    cr.set_source_rgba(0.9, 0.9, 0.9, 0.35);
    cr.set_line_width(1.2 * z);
    for offset in &[-1.5, 0.0, 1.5] {
        cr.move_to(cup_x + offset * z, counter_top - cup_h - 1.0 * z);
        cr.curve_to(
            cup_x + offset * z - 2.5 * z, counter_top - cup_h - 6.0 * z,
            cup_x + offset * z + 2.5 * z, counter_top - cup_h - 11.0 * z,
            cup_x + offset * z, counter_top - cup_h - 16.0 * z,
        );
        let _ = cr.stroke();
    }
}

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Couch legs
    cr.set_source_rgb(0.25, 0.15, 0.08);
    let hw = TILE_W / 2.0 * z * 0.75;
    for &lx in &[sx - hw + 2.0 * z, sx + hw - 2.0 * z] {
        cr.rectangle(lx - 1.0 * z, sy - 4.0 * z, 2.0 * z, 4.0 * z);
        let _ = cr.fill();
    }

    // Seat cushion (sitting height ~16px)
    iso_block(cr, sx, sy, z, 0.75, 0.75, 16.0, 0.58, 0.14, 0.10);

    let seat_top = sy - 16.0 * z;

    // Back rest (~22px above seat = 38px total)
    let back_h = 22.0 * z;
    // Left face of back
    cr.move_to(sx - hw, seat_top - back_h);
    cr.line_to(sx - hw, seat_top);
    cr.line_to(sx - hw + 4.0 * z, seat_top);
    cr.line_to(sx - hw + 4.0 * z, seat_top - back_h);
    cr.close_path();
    cr.set_source_rgb(0.48, 0.10, 0.07);
    let _ = cr.fill();

    // Front face of back
    cr.move_to(sx - hw + 4.0 * z, seat_top - back_h);
    cr.line_to(sx + hw, seat_top - back_h);
    cr.line_to(sx + hw, seat_top);
    cr.line_to(sx - hw + 4.0 * z, seat_top);
    cr.close_path();
    cr.set_source_rgb(0.55, 0.12, 0.09);
    let _ = cr.fill();

    // Top edge of back
    cr.move_to(sx - hw, seat_top - back_h);
    cr.line_to(sx - hw + 4.0 * z, seat_top - back_h);
    cr.line_to(sx + hw, seat_top - back_h);
    cr.line_to(sx + hw - 4.0 * z, seat_top - back_h - 2.0 * z);
    cr.close_path();
    cr.set_source_rgb(0.52, 0.11, 0.08);
    let _ = cr.fill();

    // Armrests
    let arm_h = 10.0 * z;
    let arm_w = 5.0 * z;
    // Left
    cr.rectangle(sx - hw, seat_top - arm_h, arm_w, arm_h);
    cr.set_source_rgb(0.45, 0.09, 0.06);
    let _ = cr.fill();
    // Right
    cr.rectangle(sx + hw - arm_w, seat_top - arm_h, arm_w, arm_h);
    cr.set_source_rgb(0.50, 0.11, 0.08);
    let _ = cr.fill();

    // Cushion tufting (stitch lines)
    cr.set_source_rgba(0.42, 0.08, 0.05, 0.5);
    cr.set_line_width(0.8 * z);
    let third = hw * 2.0 / 3.0;
    for i in 1..3 {
        let cx = sx - hw + third * i as f64;
        cr.move_to(cx, seat_top - 2.0 * z);
        cr.line_to(cx, seat_top + 4.0 * z);
        let _ = cr.stroke();
    }

    // Throw pillow
    cr.save().unwrap();
    cr.translate(sx - hw / 3.0, seat_top - 4.0 * z);
    cr.scale(1.0, 0.6);
    cr.arc(0.0, 0.0, 4.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.75, 0.25);
    let _ = cr.fill();
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Pot (tall planter, ~12px)
    iso_block(cr, sx, sy, z, 0.3, 0.3, 12.0, 0.55, 0.32, 0.16);
    // Pot rim
    cr.save().unwrap();
    cr.translate(sx, sy - 12.0 * z);
    cr.scale(1.0, 0.45);
    cr.arc(0.0, 0.0, TILE_W / 2.0 * z * 0.32, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.50, 0.28, 0.12);
    let _ = cr.fill();

    // Soil
    cr.save().unwrap();
    cr.translate(sx, sy - 12.0 * z);
    cr.scale(1.0, 0.45);
    cr.arc(0.0, 0.0, TILE_W / 2.0 * z * 0.27, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.28, 0.20, 0.09);
    let _ = cr.fill();

    // Trunk (thick, tall)
    cr.set_source_rgb(0.36, 0.24, 0.10);
    cr.rectangle(sx - 2.0 * z, sy - 32.0 * z, 4.0 * z, 20.0 * z);
    let _ = cr.fill();
    // Trunk texture
    cr.set_source_rgba(0.28, 0.18, 0.06, 0.4);
    cr.set_line_width(0.5 * z);
    for i in 0..4 {
        let ty = sy - 14.0 * z - i as f64 * 5.0 * z;
        cr.move_to(sx - 2.0 * z, ty);
        cr.line_to(sx + 2.0 * z, ty);
        let _ = cr.stroke();
    }

    // Branch
    cr.set_source_rgb(0.36, 0.24, 0.10);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx, sy - 28.0 * z);
    cr.line_to(sx + 8.0 * z, sy - 35.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx, sy - 25.0 * z);
    cr.line_to(sx - 6.0 * z, sy - 32.0 * z);
    let _ = cr.stroke();

    // Foliage (large overlapping spheres for a real tree canopy, ~50px total height)
    let leaves: [(f64, f64, f64, f64); 7] = [
        (0.0, -42.0, 12.0, 0.85),   // top center
        (-7.0, -36.0, 9.0, 0.75),   // left mid
        (8.0, -38.0, 9.0, 0.90),    // right mid
        (-4.0, -46.0, 8.0, 0.95),   // top left
        (5.0, -44.0, 8.0, 0.80),    // top right
        (-9.0, -30.0, 7.0, 0.70),   // lower left
        (10.0, -32.0, 7.0, 0.82),   // lower right
    ];
    for (dx, dy, r, shade) in &leaves {
        cr.arc(sx + dx * z, sy + dy * z, r * z, 0.0, TAU);
        cr.set_source_rgb(0.12 * shade, 0.55 * shade, 0.12 * shade);
        let _ = cr.fill();
    }

    // Light highlights
    for &(dx, dy, r) in &[(1.0, -44.0, 4.0), (-3.0, -38.0, 3.0)] {
        cr.arc(sx + dx * z, sy + dy * z, r * z, 0.0, TAU);
        cr.set_source_rgba(0.25, 0.72, 0.25, 0.25);
        let _ = cr.fill();
    }
}

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    // Full-size arcade cabinet (~56px, roughly person height)
    let height = 56.0;
    iso_block(cr, sx, sy, z, 0.5, 0.5, height, 0.32, 0.08, 0.48);

    let top = sy - height * z;
    let hw = TILE_W / 2.0 * z * 0.5;

    // Marquee (lit header)
    cr.rectangle(sx - hw + 1.5 * z, top + 1.5 * z, hw * 2.0 - 3.0 * z, 6.0 * z);
    cr.set_source_rgb(1.0, 0.85, 0.1);
    let _ = cr.fill();
    cr.set_source_rgb(0.12, 0.04, 0.28);
    cr.set_font_size(4.0 * z);
    let _ = cr.move_to(sx - 7.0 * z, top + 6.5 * z);
    let _ = cr.show_text("ARCADE");

    // Screen bezel
    cr.rectangle(sx - hw + 2.0 * z, top + 9.0 * z, hw * 2.0 - 4.0 * z, 18.0 * z);
    cr.set_source_rgb(0.06, 0.06, 0.08);
    let _ = cr.fill();

    // CRT screen
    let scr_x = sx - hw + 3.0 * z;
    let scr_y = top + 10.0 * z;
    let scr_w = hw * 2.0 - 6.0 * z;
    let scr_h = 16.0 * z;
    cr.rectangle(scr_x, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.04, 0.10, 0.04);
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
    cr.rectangle(sx - hw + 2.0 * z, top + 28.0 * z, hw * 2.0 - 4.0 * z, 8.0 * z);
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
    let btn_colors = [(1.0, 0.2, 0.2), (0.2, 0.2, 1.0), (0.2, 0.8, 0.2), (0.9, 0.9, 0.1)];
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
    // Belt/base (raised platform, ~8px)
    iso_block(cr, sx, sy, z, 0.75, 0.45, 8.0, 0.26, 0.26, 0.28);

    let belt_top = sy - 8.0 * z;

    // Belt treads
    cr.set_source_rgba(0.20, 0.20, 0.22, 0.6);
    cr.set_line_width(0.8 * z);
    for i in 0..6 {
        let bx = sx - 10.0 * z + i as f64 * 4.0 * z;
        cr.move_to(bx, belt_top - 1.0 * z);
        cr.line_to(bx + 2.0 * z, belt_top + 1.5 * z);
        let _ = cr.stroke();
    }

    // Upright posts (metallic, tall — handle height ~40px from floor)
    let post_h = 38.0 * z;
    cr.set_source_rgb(0.52, 0.52, 0.55);
    cr.set_line_width(3.0 * z);
    cr.move_to(sx - 10.0 * z, belt_top);
    cr.line_to(sx - 10.0 * z, belt_top - post_h);
    let _ = cr.stroke();
    cr.move_to(sx + 10.0 * z, belt_top);
    cr.line_to(sx + 10.0 * z, belt_top - post_h);
    let _ = cr.stroke();

    // Top handlebar
    cr.set_line_width(2.5 * z);
    cr.move_to(sx - 10.0 * z, belt_top - post_h);
    cr.line_to(sx + 10.0 * z, belt_top - post_h);
    let _ = cr.stroke();

    // Grip wraps (rubber)
    cr.set_source_rgb(0.15, 0.15, 0.17);
    cr.set_line_width(4.0 * z);
    cr.move_to(sx - 10.0 * z, belt_top - post_h + 4.0 * z);
    cr.line_to(sx - 10.0 * z, belt_top - post_h + 8.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 10.0 * z, belt_top - post_h + 4.0 * z);
    cr.line_to(sx + 10.0 * z, belt_top - post_h + 8.0 * z);
    let _ = cr.stroke();

    // Console display (between handles at top)
    let console_w = 12.0 * z;
    let console_h = 7.0 * z;
    let console_y = belt_top - post_h - 2.0 * z;
    cr.rectangle(sx - console_w / 2.0, console_y, console_w, console_h);
    cr.set_source_rgb(0.12, 0.12, 0.14);
    let _ = cr.fill();
    // Screen
    cr.rectangle(sx - console_w / 2.0 + 1.0 * z, console_y + 1.0 * z, console_w - 2.0 * z, console_h - 2.0 * z);
    cr.set_source_rgb(0.08, 0.55, 0.28);
    let _ = cr.fill();
    // Speed + distance readout
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
    cr.move_to(sx - 10.0 * z, belt_top - 18.0 * z);
    cr.line_to(sx - 12.0 * z, belt_top - 8.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 10.0 * z, belt_top - 18.0 * z);
    cr.line_to(sx + 12.0 * z, belt_top - 8.0 * z);
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
    cr.rectangle(board_x - 1.5 * z, board_y - 1.5 * z, board_w + 3.0 * z, board_h + 3.0 * z);
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
    cr.rectangle(board_x + 3.0 * z, board_y + board_h + 1.0 * z, board_w - 6.0 * z, 2.5 * z);
    cr.set_source_rgb(0.42, 0.42, 0.45);
    let _ = cr.fill();

    // Markers
    let marker_colors = [(0.08, 0.08, 0.75), (0.75, 0.08, 0.08), (0.08, 0.55, 0.08), (0.0, 0.0, 0.0)];
    for (i, (mr, mg, mb)) in marker_colors.iter().enumerate() {
        cr.rectangle(board_x + 5.0 * z + i as f64 * 4.0 * z, board_y + board_h + 1.0 * z, 3.0 * z, 2.0 * z);
        cr.set_source_rgb(*mr, *mg, *mb);
        let _ = cr.fill();
    }

    // Eraser
    cr.rectangle(board_x + board_w - 10.0 * z, board_y + board_h + 1.0 * z, 5.0 * z, 2.0 * z);
    cr.set_source_rgb(0.9, 0.9, 0.85);
    let _ = cr.fill();
}
