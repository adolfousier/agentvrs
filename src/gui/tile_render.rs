use crate::gui::iso::{TILE_H, TILE_W, WALL_HEIGHT};
use crate::world::{FloorKind, Tile, WallKind};
use std::f64::consts::TAU;

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

    // Wood grain / tile grid detail
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
            // Cross lines for tile grid
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

    // Window panes on both faces
    if matches!(kind, WallKind::Window) {
        let inset = 4.0 * zoom;
        let pane_h = wh * 0.55;
        let pane_top = wh * 0.2;

        // Left face window
        let lx0 = sx - hw + inset * 0.7;
        let lx1 = sx - inset * 0.3;
        let ly_base = sy - wh + pane_top;
        cr.move_to(lx0, ly_base + hh * 0.15);
        cr.line_to(lx1, ly_base + hh * 0.85);
        cr.line_to(lx1, ly_base + hh * 0.85 + pane_h);
        cr.line_to(lx0, ly_base + hh * 0.15 + pane_h);
        cr.close_path();
        cr.set_source_rgba(0.55, 0.78, 0.95, 0.5);
        let _ = cr.fill();
        // Window frame
        cr.set_source_rgba(0.35, 0.35, 0.4, 0.6);
        cr.set_line_width(1.0 * zoom);
        let mid_x = (lx0 + lx1) / 2.0;
        let mid_y_top = ly_base + hh * 0.5;
        let mid_y_bot = mid_y_top + pane_h;
        cr.move_to(mid_x, mid_y_top);
        cr.line_to(mid_x, mid_y_bot);
        let _ = cr.stroke();

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
        let mid_rx = (rx0 + rx1) / 2.0;
        let mid_ry = ly_base + hh * 0.5;
        cr.set_source_rgba(0.35, 0.35, 0.4, 0.6);
        cr.move_to(mid_rx, mid_ry);
        cr.line_to(mid_rx, mid_ry + pane_h);
        let _ = cr.stroke();

        // Sky reflection highlight
        cr.set_source_rgba(0.85, 0.92, 1.0, 0.15);
        cr.move_to(rx0 + 2.0 * zoom, ly_base + hh * 0.85 + 2.0 * zoom);
        cr.line_to(rx1 - 2.0 * zoom, ly_base + hh * 0.15 + 2.0 * zoom);
        cr.line_to(rx1 - 2.0 * zoom, ly_base + hh * 0.15 + pane_h * 0.4);
        cr.line_to(rx0 + 2.0 * zoom, ly_base + hh * 0.85 + pane_h * 0.4);
        cr.close_path();
        let _ = cr.fill();
    }

    // Mortar lines on solid walls
    if matches!(kind, WallKind::Solid) {
        cr.set_source_rgba(0.4, 0.38, 0.36, 0.3);
        cr.set_line_width(0.5 * zoom);
        // Horizontal mortar on right face
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
    // Door mat effect
    let hw = TILE_W / 2.0 * zoom * 0.55;
    let hh = TILE_H / 2.0 * zoom * 0.55;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(0.42, 0.42, 0.44);
    let _ = cr.fill();
    // Threshold lines
    cr.set_source_rgba(0.35, 0.35, 0.37, 0.7);
    cr.set_line_width(1.5 * zoom);
    cr.move_to(sx - hw * 0.7, sy + hh * 0.3);
    cr.line_to(sx + hw * 0.7, sy - hh * 0.3);
    let _ = cr.stroke();
}

fn draw_rug(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.62, 0.16, 0.16);
    // Border ring
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
    // Inner pattern diamond
    let hw2 = TILE_W / 2.0 * zoom * 0.5;
    let hh2 = TILE_H / 2.0 * zoom * 0.5;
    cr.move_to(sx, sy - hh2);
    cr.line_to(sx + hw2, sy);
    cr.line_to(sx, sy + hh2);
    cr.line_to(sx - hw2, sy);
    cr.close_path();
    cr.set_source_rgb(0.78, 0.28, 0.12);
    let _ = cr.fill();
    // Center medallion
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 5.0 * zoom, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.65, 0.15);
    let _ = cr.fill();
}

// ─── Isometric block helper ───

fn iso_block(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    zoom: f64,
    w_ratio: f64,
    h_ratio: f64,
    height: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * zoom * w_ratio;
    let hh = TILE_H / 2.0 * zoom * h_ratio;
    let bh = height * zoom;

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

// ─── Furniture ───

fn draw_desk(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Desk legs (4 thin columns)
    let leg_color = (0.40, 0.28, 0.14);
    let leg_h = 7.0;
    let hw = TILE_W / 2.0 * z * 0.65;
    let hh = TILE_H / 2.0 * z * 0.65;
    let leg_w = 2.0 * z;
    for &(lx, ly) in &[
        (sx - hw + leg_w, sy - hh * 0.3),
        (sx + hw - leg_w, sy - hh * 0.3),
        (sx - hw * 0.3, sy + hh - leg_w),
        (sx + hw * 0.3, sy + hh - leg_w),
    ] {
        cr.rectangle(lx - leg_w / 2.0, ly - leg_h * z, leg_w, leg_h * z);
        cr.set_source_rgb(leg_color.0, leg_color.1, leg_color.2);
        let _ = cr.fill();
    }

    // Table surface (isometric block)
    iso_block(cr, sx, sy, z, 0.7, 0.7, 8.0, 0.60, 0.40, 0.20);

    // Keyboard on desk
    let desk_top = sy - 8.0 * z;
    let kb_w = 8.0 * z;
    let kb_h = 2.0 * z;
    cr.move_to(sx - kb_w / 2.0, desk_top - 0.5 * z);
    cr.line_to(sx + kb_w / 2.0, desk_top - 0.5 * z);
    cr.line_to(sx + kb_w / 2.0 - 1.0 * z, desk_top - 0.5 * z - kb_h);
    cr.line_to(sx - kb_w / 2.0 + 1.0 * z, desk_top - 0.5 * z - kb_h);
    cr.close_path();
    cr.set_source_rgb(0.22, 0.22, 0.25);
    let _ = cr.fill();
    // Key dots
    cr.set_source_rgba(0.45, 0.45, 0.5, 0.7);
    for i in 0..5 {
        let kx = sx - 3.0 * z + i as f64 * 1.5 * z;
        cr.rectangle(kx, desk_top - 1.5 * z, 1.0 * z, 0.8 * z);
        let _ = cr.fill();
    }

    // Monitor stand (thin column)
    cr.rectangle(sx - 1.5 * z, desk_top - 5.0 * z, 3.0 * z, 5.0 * z);
    cr.set_source_rgb(0.28, 0.28, 0.32);
    let _ = cr.fill();

    // Monitor base plate
    cr.save().unwrap();
    cr.translate(sx, desk_top);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 4.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.25, 0.25, 0.3);
    let _ = cr.fill();

    // Monitor body (3D block effect)
    let mon_w = 18.0 * z;
    let mon_h = 12.0 * z;
    let mon_y = desk_top - 5.0 * z - mon_h;
    // Back bezel
    cr.rectangle(sx - mon_w / 2.0 - 0.5 * z, mon_y - 0.5 * z, mon_w + 1.0 * z, mon_h + 1.0 * z);
    cr.set_source_rgb(0.12, 0.12, 0.15);
    let _ = cr.fill();
    // Front bezel
    cr.rectangle(sx - mon_w / 2.0, mon_y, mon_w, mon_h);
    cr.set_source_rgb(0.18, 0.18, 0.22);
    let _ = cr.fill();

    // Screen
    let border = 1.5 * z;
    let scr_x = sx - mon_w / 2.0 + border;
    let scr_y = mon_y + border;
    let scr_w = mon_w - border * 2.0;
    let scr_h = mon_h - border * 2.5;
    cr.rectangle(scr_x, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.12, 0.18, 0.28);
    let _ = cr.fill();

    // Screen glow
    cr.rectangle(scr_x, scr_y, scr_w, scr_h * 0.4);
    cr.set_source_rgba(0.15, 0.22, 0.35, 0.5);
    let _ = cr.fill();

    // Code lines on screen (colored like a real IDE)
    cr.set_line_width(1.0 * z);
    let line_colors: [(f64, f64, f64, f64); 5] = [
        (0.55, 0.85, 0.55, 0.8),  // green - keywords
        (0.85, 0.75, 0.45, 0.8),  // yellow - strings
        (0.55, 0.70, 0.90, 0.8),  // blue - functions
        (0.80, 0.55, 0.80, 0.7),  // purple - types
        (0.55, 0.85, 0.55, 0.6),  // green again
    ];
    for (i, (lr, lg, lb, la)) in line_colors.iter().enumerate() {
        let ly = scr_y + 2.0 * z + i as f64 * 2.2 * z;
        let indent = if i == 1 || i == 2 { 3.0 * z } else { 1.0 * z };
        let lw = scr_w * (0.75 - i as f64 * 0.08) - indent;
        cr.set_source_rgba(*lr, *lg, *lb, *la);
        cr.move_to(scr_x + indent, ly);
        cr.line_to(scr_x + indent + lw, ly);
        let _ = cr.stroke();
    }

    // Power LED
    cr.arc(sx + mon_w / 2.0 - 3.0 * z, mon_y + mon_h - 2.0 * z, 0.8 * z, 0.0, TAU);
    cr.set_source_rgb(0.1, 0.8, 0.3);
    let _ = cr.fill();
}

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Main body (tall block)
    iso_block(cr, sx, sy, z, 0.55, 0.55, 24.0, 0.70, 0.12, 0.12);

    let top = sy - 24.0 * z;
    let hw = TILE_W / 2.0 * z * 0.55;

    // Brand panel at top
    cr.rectangle(sx - hw + 2.0 * z, top + 1.5 * z, hw * 2.0 - 4.0 * z, 4.0 * z);
    cr.set_source_rgb(0.85, 0.15, 0.15);
    let _ = cr.fill();
    // Brand text
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_font_size(3.5 * z);
    let _ = cr.move_to(sx - 5.0 * z, top + 4.5 * z);
    let _ = cr.show_text("DRINKS");

    // Glass front panel
    cr.rectangle(sx - hw + 2.0 * z, top + 6.5 * z, hw * 2.0 - 4.0 * z, 12.0 * z);
    cr.set_source_rgba(0.3, 0.35, 0.4, 0.6);
    let _ = cr.fill();

    // Product shelves (3 rows, colored cans/bottles)
    let can_colors: [(f64, f64, f64); 6] = [
        (0.9, 0.2, 0.1), (0.1, 0.5, 0.9), (0.1, 0.7, 0.2),
        (0.9, 0.7, 0.0), (0.6, 0.1, 0.7), (0.9, 0.5, 0.1),
    ];
    for row in 0..3 {
        let ry = top + 7.5 * z + row as f64 * 3.8 * z;
        // Shelf line
        cr.set_source_rgba(0.5, 0.5, 0.55, 0.5);
        cr.set_line_width(0.5 * z);
        cr.move_to(sx - hw + 2.5 * z, ry + 3.0 * z);
        cr.line_to(sx + hw - 2.5 * z, ry + 3.0 * z);
        let _ = cr.stroke();
        // Cans
        for col in 0..4 {
            let cx = sx - hw + 4.0 * z + col as f64 * 3.5 * z;
            let ci = (row * 2 + col) % can_colors.len();
            let (cr2, cg, cb) = can_colors[ci];
            // Can body
            cr.rectangle(cx, ry, 2.5 * z, 2.8 * z);
            cr.set_source_rgb(cr2, cg, cb);
            let _ = cr.fill();
            // Can highlight
            cr.rectangle(cx + 0.3 * z, ry + 0.3 * z, 0.6 * z, 2.2 * z);
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.3);
            let _ = cr.fill();
        }
    }

    // Dispensing slot at bottom
    cr.rectangle(sx - 5.0 * z, top + 19.5 * z, 10.0 * z, 3.5 * z);
    cr.set_source_rgb(0.08, 0.08, 0.1);
    let _ = cr.fill();
    // Slot highlight
    cr.rectangle(sx - 4.5 * z, top + 20.0 * z, 9.0 * z, 0.5 * z);
    cr.set_source_rgba(0.4, 0.4, 0.45, 0.5);
    let _ = cr.fill();
}

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Counter base
    iso_block(cr, sx, sy, z, 0.6, 0.6, 10.0, 0.50, 0.48, 0.45);

    let counter_top = sy - 10.0 * z;

    // Coffee machine body on counter
    iso_block(cr, sx - 3.0 * z, counter_top + 4.0 * z, z, 0.35, 0.35, 14.0, 0.25, 0.25, 0.28);

    let machine_top = counter_top + 4.0 * z - 14.0 * z;

    // Machine display panel
    cr.rectangle(sx - 7.0 * z, machine_top + 2.0 * z, 6.0 * z, 3.0 * z);
    cr.set_source_rgb(0.1, 0.6, 0.8);
    let _ = cr.fill();

    // Drip nozzle
    cr.rectangle(sx - 5.0 * z, machine_top + 8.0 * z, 3.0 * z, 2.0 * z);
    cr.set_source_rgb(0.18, 0.18, 0.2);
    let _ = cr.fill();

    // Cup on counter (cylinder effect)
    let cup_x = sx + 4.0 * z;
    let cup_y = counter_top;
    cr.rectangle(cup_x - 2.0 * z, cup_y - 4.0 * z, 4.0 * z, 4.0 * z);
    cr.set_source_rgb(0.92, 0.92, 0.90);
    let _ = cr.fill();
    // Cup rim (ellipse)
    cr.save().unwrap();
    cr.translate(cup_x, cup_y - 4.0 * z);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 2.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.85, 0.85, 0.83);
    let _ = cr.fill();
    // Coffee inside
    cr.save().unwrap();
    cr.translate(cup_x, cup_y - 4.0 * z);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 1.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.35, 0.2, 0.1);
    let _ = cr.fill();

    // Steam wisps (curved lines)
    cr.set_source_rgba(0.9, 0.9, 0.9, 0.4);
    cr.set_line_width(1.0 * z);
    for offset in &[-1.5, 1.5] {
        cr.move_to(cup_x + offset * z, cup_y - 5.0 * z);
        cr.curve_to(
            cup_x + offset * z - 2.0 * z, cup_y - 8.0 * z,
            cup_x + offset * z + 2.0 * z, cup_y - 11.0 * z,
            cup_x + offset * z, cup_y - 14.0 * z,
        );
        let _ = cr.stroke();
    }
}

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Seat cushion (wide low block)
    iso_block(cr, sx, sy, z, 0.72, 0.72, 5.0, 0.60, 0.15, 0.12);

    let seat_top = sy - 5.0 * z;
    let hw = TILE_W / 2.0 * z * 0.72;

    // Back rest (tall thin block behind)
    let back_h = 10.0 * z;
    // Left face of backrest
    cr.move_to(sx - hw, seat_top - back_h);
    cr.line_to(sx - hw, seat_top);
    cr.line_to(sx - hw + 3.0 * z, seat_top);
    cr.line_to(sx - hw + 3.0 * z, seat_top - back_h);
    cr.close_path();
    cr.set_source_rgb(0.50, 0.10, 0.08);
    let _ = cr.fill();

    // Back face (connecting)
    cr.move_to(sx - hw, seat_top - back_h);
    cr.line_to(sx - hw + 3.0 * z, seat_top - back_h);
    let hh = TILE_H / 2.0 * z * 0.72;
    cr.line_to(sx - hw + 3.0 * z + hw * 0.15, seat_top - back_h + hh * 0.15);
    cr.line_to(sx - hw + hw * 0.15, seat_top - back_h + hh * 0.15);
    cr.close_path();
    cr.set_source_rgb(0.55, 0.12, 0.10);
    let _ = cr.fill();

    // Armrests
    let arm_h = 3.0 * z;
    // Left armrest
    cr.rectangle(sx - hw, seat_top - arm_h, 3.5 * z, arm_h);
    cr.set_source_rgb(0.48, 0.09, 0.07);
    let _ = cr.fill();
    // Right armrest
    cr.rectangle(sx + hw - 3.5 * z, seat_top - arm_h, 3.5 * z, arm_h);
    cr.set_source_rgb(0.52, 0.11, 0.09);
    let _ = cr.fill();

    // Cushion detail (stitch line)
    cr.set_source_rgba(0.45, 0.08, 0.06, 0.5);
    cr.set_line_width(0.8 * z);
    cr.move_to(sx, seat_top - 1.0 * z);
    cr.line_to(sx, seat_top + 2.0 * z);
    let _ = cr.stroke();
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Pot (isometric cylinder effect)
    iso_block(cr, sx, sy, z, 0.28, 0.28, 7.0, 0.58, 0.34, 0.18);
    // Pot rim
    cr.save().unwrap();
    cr.translate(sx, sy - 7.0 * z);
    cr.scale(1.0, 0.45);
    cr.arc(0.0, 0.0, TILE_W / 2.0 * z * 0.3, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.52, 0.30, 0.14);
    let _ = cr.fill();

    // Soil
    cr.save().unwrap();
    cr.translate(sx, sy - 7.0 * z);
    cr.scale(1.0, 0.45);
    cr.arc(0.0, 0.0, TILE_W / 2.0 * z * 0.25, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.30, 0.22, 0.10);
    let _ = cr.fill();

    // Trunk
    cr.set_source_rgb(0.38, 0.26, 0.12);
    cr.rectangle(sx - 1.5 * z, sy - 16.0 * z, 3.0 * z, 9.0 * z);
    let _ = cr.fill();

    // Foliage layers (overlapping circles for volume)
    let leaves: [(f64, f64, f64, f64); 5] = [
        (0.0, -22.0, 9.0, 0.9),
        (-5.0, -18.0, 6.0, 0.75),
        (5.0, -19.0, 6.0, 0.85),
        (-3.0, -24.0, 5.0, 0.95),
        (3.0, -23.0, 5.5, 0.8),
    ];
    for (dx, dy, r, shade) in &leaves {
        cr.arc(sx + dx * z, sy + dy * z, r * z, 0.0, TAU);
        cr.set_source_rgb(0.15 * shade, 0.58 * shade, 0.15 * shade);
        let _ = cr.fill();
    }
    // Light highlights on top leaves
    cr.arc(sx - 1.0 * z, sy - 24.0 * z, 3.0 * z, 0.0, TAU);
    cr.set_source_rgba(0.3, 0.75, 0.3, 0.3);
    let _ = cr.fill();
}

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Cabinet body
    iso_block(cr, sx, sy, z, 0.5, 0.5, 22.0, 0.35, 0.10, 0.50);

    let top = sy - 22.0 * z;
    let hw = TILE_W / 2.0 * z * 0.5;

    // Marquee (lit header)
    cr.rectangle(sx - hw + 1.0 * z, top + 1.0 * z, hw * 2.0 - 2.0 * z, 3.5 * z);
    cr.set_source_rgb(1.0, 0.85, 0.1);
    let _ = cr.fill();
    cr.set_source_rgb(0.15, 0.05, 0.3);
    cr.set_font_size(2.8 * z);
    let _ = cr.move_to(sx - 5.0 * z, top + 3.8 * z);
    let _ = cr.show_text("ARCADE");

    // Screen bezel
    cr.rectangle(sx - hw + 1.5 * z, top + 5.5 * z, hw * 2.0 - 3.0 * z, 8.0 * z);
    cr.set_source_rgb(0.08, 0.08, 0.1);
    let _ = cr.fill();

    // Screen (CRT glow effect)
    let scr_x = sx - hw + 2.5 * z;
    let scr_y = top + 6.5 * z;
    let scr_w = hw * 2.0 - 5.0 * z;
    let scr_h = 6.0 * z;
    cr.rectangle(scr_x, scr_y, scr_w, scr_h);
    cr.set_source_rgb(0.05, 0.12, 0.05);
    let _ = cr.fill();

    // Game graphics (simple space invader style)
    cr.set_source_rgb(0.2, 0.9, 0.3);
    cr.set_font_size(4.0 * z);
    let _ = cr.move_to(scr_x + 1.0 * z, scr_y + 4.5 * z);
    let _ = cr.show_text("▼ ▼ ▼");
    cr.set_source_rgb(0.9, 0.9, 0.2);
    cr.set_font_size(3.0 * z);
    let _ = cr.move_to(scr_x + scr_w / 2.0 - 1.5 * z, scr_y + scr_h - 0.5 * z);
    let _ = cr.show_text("▲");

    // Scanline effect
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
    cr.set_line_width(0.5 * z);
    let mut scanline_y = scr_y;
    while scanline_y < scr_y + scr_h {
        cr.move_to(scr_x, scanline_y);
        cr.line_to(scr_x + scr_w, scanline_y);
        let _ = cr.stroke();
        scanline_y += 1.5 * z;
    }

    // Control panel
    cr.rectangle(sx - hw + 1.5 * z, top + 14.0 * z, hw * 2.0 - 3.0 * z, 4.0 * z);
    cr.set_source_rgb(0.2, 0.2, 0.22);
    let _ = cr.fill();

    // Joystick
    cr.rectangle(sx - 2.0 * z, top + 14.5 * z, 1.5 * z, 3.0 * z);
    cr.set_source_rgb(0.15, 0.15, 0.15);
    let _ = cr.fill();
    cr.arc(sx - 1.25 * z, top + 14.5 * z, 1.5 * z, 0.0, TAU);
    cr.set_source_rgb(0.7, 0.1, 0.1);
    let _ = cr.fill();

    // Buttons
    let btn_colors = [(1.0, 0.2, 0.2), (0.2, 0.2, 1.0), (0.2, 0.8, 0.2)];
    for (i, (br, bg, bb)) in btn_colors.iter().enumerate() {
        cr.arc(sx + 2.0 * z + i as f64 * 2.5 * z, top + 16.0 * z, 1.0 * z, 0.0, TAU);
        cr.set_source_rgb(*br, *bg, *bb);
        let _ = cr.fill();
        // Button highlight
        cr.arc(sx + 1.7 * z + i as f64 * 2.5 * z, top + 15.7 * z, 0.4 * z, 0.0, TAU);
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.3);
        let _ = cr.fill();
    }

    // Coin slot
    cr.rectangle(sx - 1.0 * z, top + 19.0 * z, 2.0 * z, 1.0 * z);
    cr.set_source_rgb(0.6, 0.55, 0.1);
    let _ = cr.fill();
}

fn draw_treadmill(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Belt/base (low wide block)
    iso_block(cr, sx, sy, z, 0.7, 0.4, 3.0, 0.28, 0.28, 0.30);

    // Belt surface detail
    let belt_top = sy - 3.0 * z;
    cr.set_source_rgba(0.22, 0.22, 0.24, 0.6);
    cr.set_line_width(0.5 * z);
    for i in 0..4 {
        let bx = sx - 8.0 * z + i as f64 * 5.0 * z;
        cr.move_to(bx, belt_top - 1.0 * z);
        cr.line_to(bx + 2.0 * z, belt_top + 1.0 * z);
        let _ = cr.stroke();
    }

    // Upright handles (metallic tubes)
    cr.set_source_rgb(0.55, 0.55, 0.58);
    cr.set_line_width(2.5 * z);
    cr.move_to(sx - 8.0 * z, belt_top);
    cr.line_to(sx - 8.0 * z, belt_top - 16.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 8.0 * z, belt_top);
    cr.line_to(sx + 8.0 * z, belt_top - 16.0 * z);
    let _ = cr.stroke();

    // Top handlebar
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 8.0 * z, belt_top - 16.0 * z);
    cr.line_to(sx + 8.0 * z, belt_top - 16.0 * z);
    let _ = cr.stroke();

    // Grip wraps
    cr.set_source_rgb(0.2, 0.2, 0.22);
    cr.set_line_width(3.0 * z);
    cr.move_to(sx - 8.0 * z, belt_top - 14.0 * z);
    cr.line_to(sx - 8.0 * z, belt_top - 12.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 8.0 * z, belt_top - 14.0 * z);
    cr.line_to(sx + 8.0 * z, belt_top - 12.0 * z);
    let _ = cr.stroke();

    // Console display
    cr.rectangle(sx - 4.0 * z, belt_top - 18.0 * z, 8.0 * z, 4.0 * z);
    cr.set_source_rgb(0.15, 0.15, 0.18);
    let _ = cr.fill();
    cr.rectangle(sx - 3.5 * z, belt_top - 17.5 * z, 7.0 * z, 3.0 * z);
    cr.set_source_rgb(0.1, 0.65, 0.3);
    let _ = cr.fill();
    // Speed readout
    cr.set_source_rgb(0.2, 0.9, 0.4);
    cr.set_font_size(2.5 * z);
    let _ = cr.move_to(sx - 2.5 * z, belt_top - 15.5 * z);
    let _ = cr.show_text("5.2");
}

fn draw_whiteboard(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let z = zoom;

    // Stand legs (A-frame)
    cr.set_source_rgb(0.45, 0.45, 0.48);
    cr.set_line_width(2.0 * z);
    // Left leg pair
    cr.move_to(sx - 10.0 * z, sy + 1.0 * z);
    cr.line_to(sx - 8.0 * z, sy - 20.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx - 6.0 * z, sy + 1.0 * z);
    cr.line_to(sx - 8.0 * z, sy - 20.0 * z);
    let _ = cr.stroke();
    // Right leg pair
    cr.move_to(sx + 10.0 * z, sy + 1.0 * z);
    cr.line_to(sx + 8.0 * z, sy - 20.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 6.0 * z, sy + 1.0 * z);
    cr.line_to(sx + 8.0 * z, sy - 20.0 * z);
    let _ = cr.stroke();

    // Board frame
    let board_x = sx - 12.0 * z;
    let board_y = sy - 22.0 * z;
    let board_w = 24.0 * z;
    let board_h = 14.0 * z;
    cr.rectangle(board_x - 1.0 * z, board_y - 1.0 * z, board_w + 2.0 * z, board_h + 2.0 * z);
    cr.set_source_rgb(0.5, 0.5, 0.52);
    let _ = cr.fill();

    // Board surface
    cr.rectangle(board_x, board_y, board_w, board_h);
    cr.set_source_rgb(0.96, 0.96, 0.97);
    let _ = cr.fill();

    // Scribbles (multi-colored, diagram-like)
    // Blue heading
    cr.set_source_rgb(0.15, 0.35, 0.75);
    cr.set_line_width(1.2 * z);
    cr.move_to(board_x + 2.0 * z, board_y + 3.0 * z);
    cr.line_to(board_x + 12.0 * z, board_y + 3.0 * z);
    let _ = cr.stroke();

    // Red diagram box
    cr.set_source_rgb(0.8, 0.2, 0.2);
    cr.set_line_width(1.0 * z);
    cr.rectangle(board_x + 3.0 * z, board_y + 5.0 * z, 6.0 * z, 3.0 * z);
    let _ = cr.stroke();

    // Arrow
    cr.move_to(board_x + 9.0 * z, board_y + 6.5 * z);
    cr.line_to(board_x + 13.0 * z, board_y + 6.5 * z);
    let _ = cr.stroke();

    // Another box
    cr.rectangle(board_x + 13.0 * z, board_y + 5.0 * z, 6.0 * z, 3.0 * z);
    let _ = cr.stroke();

    // Green notes
    cr.set_source_rgb(0.1, 0.6, 0.2);
    cr.set_line_width(0.8 * z);
    cr.move_to(board_x + 2.0 * z, board_y + 10.0 * z);
    cr.line_to(board_x + 10.0 * z, board_y + 10.0 * z);
    let _ = cr.stroke();
    cr.move_to(board_x + 2.0 * z, board_y + 12.0 * z);
    cr.line_to(board_x + 8.0 * z, board_y + 12.0 * z);
    let _ = cr.stroke();

    // Marker tray
    cr.rectangle(board_x + 2.0 * z, board_y + board_h + 0.5 * z, board_w - 4.0 * z, 1.5 * z);
    cr.set_source_rgb(0.45, 0.45, 0.48);
    let _ = cr.fill();

    // Markers in tray
    let marker_colors = [(0.1, 0.1, 0.8), (0.8, 0.1, 0.1), (0.1, 0.6, 0.1)];
    for (i, (mr, mg, mb)) in marker_colors.iter().enumerate() {
        cr.rectangle(board_x + 4.0 * z + i as f64 * 3.0 * z, board_y + board_h + 0.5 * z, 2.0 * z, 1.2 * z);
        cr.set_source_rgb(*mr, *mg, *mb);
        let _ = cr.fill();
    }
}
