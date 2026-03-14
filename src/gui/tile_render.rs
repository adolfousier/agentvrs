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
        FloorKind::Wood => (0.78, 0.62, 0.42),
        FloorKind::Tile => (0.88, 0.84, 0.78),
        FloorKind::Carpet => (0.42, 0.38, 0.50),
        FloorKind::Concrete => (0.32, 0.30, 0.30),
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
        WallKind::Solid => (0.55, 0.45, 0.35),
        WallKind::Window => (0.52, 0.48, 0.42),
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
        cr.set_source_rgba(0.35, 0.35, 0.4, 0.6);
        cr.set_line_width(1.0 * zoom);
        let mid_rx = (rx0 + rx1) / 2.0;
        let mid_ry = ly_base + hh * 0.5;
        cr.move_to(mid_rx, mid_ry);
        cr.line_to(mid_rx, mid_ry + pane_h);
        let _ = cr.stroke();
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
    let hw2 = TILE_W / 2.0 * zoom * 0.5;
    let hh2 = TILE_H / 2.0 * zoom * 0.5;
    cr.move_to(sx, sy - hh2);
    cr.line_to(sx + hw2, sy);
    cr.line_to(sx, sy + hh2);
    cr.line_to(sx - hw2, sy);
    cr.close_path();
    cr.set_source_rgb(0.70, 0.30, 0.15);
    let _ = cr.fill();
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 5.0 * zoom, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.80, 0.55, 0.18);
    let _ = cr.fill();
}

// ─── Ground shadow (drawn BEFORE the object) ───

fn draw_ground_shadow(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64, w_ratio: f64, h_ratio: f64) {
    let hw = TILE_W / 2.0 * z * w_ratio * 1.1;
    let hh = TILE_H / 2.0 * z * h_ratio * 1.1;
    // Offset shadow slightly to the right and down (light from top-left)
    let ox = 3.0 * z;
    let oy = 2.0 * z;
    cr.move_to(sx + ox, sy + oy - hh);
    cr.line_to(sx + ox + hw, sy + oy);
    cr.line_to(sx + ox, sy + oy + hh);
    cr.line_to(sx + ox - hw, sy + oy);
    cr.close_path();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.18);
    let _ = cr.fill();
}

// ─── Isometric block with shadows, highlights, and edge outlines ───

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

    // 7 visible vertices
    let bl = (sx - hw, sy);
    let br = (sx + hw, sy);
    let bf = (sx, sy + hh);
    let tl = (sx - hw, sy - bh);
    let tr = (sx + hw, sy - bh);
    let tf = (sx, sy + hh - bh);
    let tb = (sx, sy - hh - bh);

    // Left face (darkest — lit from top-right)
    cr.move_to(tl.0, tl.1);
    cr.line_to(bl.0, bl.1);
    cr.line_to(bf.0, bf.1);
    cr.line_to(tf.0, tf.1);
    cr.close_path();
    cr.set_source_rgb(r * 0.58, g * 0.58, b * 0.58);
    let _ = cr.fill();

    // Right face (medium — catches some light)
    cr.move_to(tr.0, tr.1);
    cr.line_to(br.0, br.1);
    cr.line_to(bf.0, bf.1);
    cr.line_to(tf.0, tf.1);
    cr.close_path();
    cr.set_source_rgb(r * 0.78, g * 0.78, b * 0.78);
    let _ = cr.fill();

    // Top face (brightest — direct light)
    cr.move_to(tb.0, tb.1);
    cr.line_to(tr.0, tr.1);
    cr.line_to(tf.0, tf.1);
    cr.line_to(tl.0, tl.1);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();

    // Highlight on top face — subtle bright gradient near the back edge
    cr.move_to(tb.0, tb.1);
    cr.line_to(tr.0, tr.1);
    // Midpoint of right-to-front edge
    let mid_r = ((tr.0 + tf.0) / 2.0, (tr.1 + tf.1) / 2.0);
    let mid_l = ((tl.0 + tf.0) / 2.0, (tl.1 + tf.1) / 2.0);
    cr.line_to(mid_r.0, mid_r.1);
    cr.line_to(mid_l.0, mid_l.1);
    cr.close_path();
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.08);
    let _ = cr.fill();

    // Dark edge outlines
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.35);
    cr.set_line_width(0.7 * z);

    // Silhouette outline (visible edges only)
    cr.move_to(tl.0, tl.1);
    cr.line_to(bl.0, bl.1);
    cr.line_to(bf.0, bf.1);
    cr.line_to(br.0, br.1);
    cr.line_to(tr.0, tr.1);
    cr.line_to(tb.0, tb.1);
    cr.close_path();
    let _ = cr.stroke();

    // Internal edges (the 3 visible seams)
    cr.move_to(tl.0, tl.1);
    cr.line_to(tf.0, tf.1);
    let _ = cr.stroke();
    cr.move_to(tf.0, tf.1);
    cr.line_to(bf.0, bf.1);
    let _ = cr.stroke();
    cr.move_to(tf.0, tf.1);
    cr.line_to(tr.0, tr.1);
    let _ = cr.stroke();

    // Bright highlight edge on top-back and top-right edges (catches light)
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.15);
    cr.set_line_width(1.0 * z);
    cr.move_to(tl.0, tl.1);
    cr.line_to(tb.0, tb.1);
    cr.line_to(tr.0, tr.1);
    let _ = cr.stroke();
}

// ─── Face detail helpers (parallelograms on iso faces) ───

#[allow(clippy::too_many_arguments)]
fn left_face_rect(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    height: f64,
    top_frac: f64,
    bot_frac: f64,
    left_frac: f64,
    right_frac: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let bh = height * z;
    let x0 = sx - hw + left_frac * hw;
    let x1 = sx - hw + right_frac * hw;
    let y_shift_0 = left_frac * hh;
    let y_shift_1 = right_frac * hh;
    cr.move_to(x0, sy - bh + top_frac * bh + y_shift_0);
    cr.line_to(x0, sy - bh + bot_frac * bh + y_shift_0);
    cr.line_to(x1, sy - bh + bot_frac * bh + y_shift_1);
    cr.line_to(x1, sy - bh + top_frac * bh + y_shift_1);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
}

#[allow(clippy::too_many_arguments)]
fn right_face_rect(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    height: f64,
    top_frac: f64,
    bot_frac: f64,
    left_frac: f64,
    right_frac: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let bh = height * z;
    let x0 = sx + left_frac * hw;
    let x1 = sx + right_frac * hw;
    let y_shift_0 = hh * (1.0 - left_frac);
    let y_shift_1 = hh * (1.0 - right_frac);
    cr.move_to(x0, sy - bh + top_frac * bh + y_shift_0);
    cr.line_to(x0, sy - bh + bot_frac * bh + y_shift_0);
    cr.line_to(x1, sy - bh + bot_frac * bh + y_shift_1);
    cr.line_to(x1, sy - bh + top_frac * bh + y_shift_1);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
}

// ─── Furniture ───

fn draw_desk(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.85, 0.70);

    // Desk body — big chunky brown block
    iso_block(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.65, 0.48, 0.28);

    // Drawer panels on left face
    left_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.08, 0.42, 0.08, 0.92, 0.52, 0.36, 0.18);
    left_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.56, 0.90, 0.08, 0.92, 0.52, 0.36, 0.18);
    // Drawer handles on left face
    left_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.23, 0.27, 0.35, 0.65, 0.72, 0.58, 0.35);
    left_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.71, 0.75, 0.35, 0.65, 0.72, 0.58, 0.35);

    // Drawer panels on right face
    right_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.08, 0.42, 0.08, 0.92, 0.58, 0.42, 0.24);
    right_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.56, 0.90, 0.08, 0.92, 0.58, 0.42, 0.24);
    // Drawer handles on right face
    right_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.23, 0.27, 0.35, 0.65, 0.72, 0.58, 0.35);
    right_face_rect(cr, sx, sy, z, 0.85, 0.70, 22.0, 0.71, 0.75, 0.35, 0.65, 0.72, 0.58, 0.35);

    // Monitor — centered on desk top
    let top = sy - 22.0 * z;
    iso_block(cr, sx, top, z, 0.40, 0.08, 16.0, 0.12, 0.12, 0.15);

    // Screen on left face (blue glow)
    left_face_rect(cr, sx, top, z, 0.40, 0.08, 16.0, 0.06, 0.90, 0.06, 0.94, 0.10, 0.16, 0.28);

    // Code lines on screen
    let colors = [(0.45, 0.82, 0.45), (0.82, 0.72, 0.40), (0.55, 0.68, 0.88)];
    for (i, &(lr, lg, lb)) in colors.iter().enumerate() {
        let t = (i as f64 + 1.0) / 4.5;
        left_face_rect(
            cr, sx, top, z, 0.40, 0.08, 16.0,
            0.15 + t * 0.55, 0.18 + t * 0.55, 0.12, 0.12 + 0.35 - i as f64 * 0.08,
            lr, lg, lb,
        );
    }

    // Keyboard on desk top
    iso_block(cr, sx, top, z, 0.25, 0.15, 1.5, 0.22, 0.22, 0.25);
}

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let h = 58.0;
    let wr = 0.70;
    let hr = 0.65;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    // Big red cabinet
    iso_block(cr, sx, sy, z, wr, hr, h, 0.78, 0.16, 0.16);

    // Glass panel on left face (darker inset)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.06, 0.68, 0.06, 0.94, 0.18, 0.22, 0.30);

    // Glass highlight (reflection streak)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.08, 0.40, 0.08, 0.15, 0.35, 0.42, 0.52);

    // Shelf lines inside glass
    for row in 0..4 {
        let y_frac = 0.08 + row as f64 * 0.15;
        left_face_rect(cr, sx, sy, z, wr, hr, h, y_frac, y_frac + 0.01, 0.08, 0.92, 0.40, 0.40, 0.45);
    }

    // Cans/bottles on shelves (bigger, brighter)
    let can_colors = [(0.92, 0.28, 0.18), (0.18, 0.58, 0.92), (0.18, 0.75, 0.28), (0.95, 0.75, 0.08), (0.72, 0.18, 0.82)];
    for row in 0..4 {
        let y_top = 0.10 + row as f64 * 0.15;
        for col in 0..3 {
            let x_left = 0.14 + col as f64 * 0.26;
            let ci = (row * 3 + col) as usize % can_colors.len();
            let (cr2, cg, cb) = can_colors[ci];
            left_face_rect(cr, sx, sy, z, wr, hr, h, y_top, y_top + 0.11, x_left, x_left + 0.20, cr2, cg, cb);
        }
    }

    // Dispensing slot (dark hole)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.72, 0.84, 0.22, 0.78, 0.05, 0.05, 0.08);
    // Slot frame
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.71, 0.85, 0.20, 0.22, 0.45, 0.45, 0.48);
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.71, 0.85, 0.78, 0.80, 0.45, 0.45, 0.48);

    // Price display (green LED)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.88, 0.94, 0.30, 0.70, 0.02, 0.02, 0.04);
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.89, 0.93, 0.32, 0.68, 0.08, 0.65, 0.12);

    // Brand stripe on right face (bright red banner)
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.02, 0.12, 0.06, 0.94, 0.95, 0.22, 0.22);

    // Side panel detail on right face
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.15, 0.85, 0.10, 0.90, 0.68, 0.12, 0.12);
    // Highlight stripe
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.40, 0.50, 0.10, 0.90, 0.85, 0.20, 0.20);
}

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let wr = 0.85;
    let hr = 0.70;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    // Kitchen counter — dark charcoal grey (like reference)
    iso_block(cr, sx, sy, z, wr, hr, 24.0, 0.25, 0.25, 0.28);

    // Cabinet doors on left face (two recessed panels)
    left_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.06, 0.92, 0.06, 0.46, 0.18, 0.18, 0.22);
    left_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.06, 0.92, 0.54, 0.94, 0.18, 0.18, 0.22);
    // Door handles
    left_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.40, 0.46, 0.58, 0.58, 0.62);
    left_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.88, 0.94, 0.58, 0.58, 0.62);

    // Cabinet doors on right face
    right_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.06, 0.92, 0.06, 0.46, 0.22, 0.22, 0.26);
    right_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.06, 0.92, 0.54, 0.94, 0.22, 0.22, 0.26);
    // Handles
    right_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.40, 0.46, 0.58, 0.58, 0.62);
    right_face_rect(cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.88, 0.94, 0.58, 0.58, 0.62);

    // Countertop — light stone/marble
    let top = sy - 24.0 * z;
    iso_block(cr, sx, top, z, 0.88, 0.73, 2.5, 0.58, 0.55, 0.50);

    // Coffee machine — white/silver block on counter
    let surface = top - 2.5 * z;
    iso_block(cr, sx, surface, z, 0.38, 0.38, 22.0, 0.90, 0.90, 0.88);

    // Machine display — green "READY" screen on left face
    left_face_rect(cr, sx, surface, z, 0.38, 0.38, 22.0, 0.10, 0.30, 0.10, 0.90, 0.02, 0.02, 0.04);
    // Green "READY" text area
    left_face_rect(cr, sx, surface, z, 0.38, 0.38, 22.0, 0.14, 0.26, 0.15, 0.85, 0.10, 0.72, 0.18);

    // Drip nozzle area (dark recess)
    left_face_rect(cr, sx, surface, z, 0.38, 0.38, 22.0, 0.50, 0.82, 0.18, 0.82, 0.12, 0.12, 0.15);

    // Coffee cup under nozzle (small white block on counter)
    // Cup sits on the counter surface, in front of machine
    iso_block(cr, sx, surface, z, 0.10, 0.10, 6.0, 0.95, 0.95, 0.92);
    // Cup dark coffee inside (top face detail — darker circle)
    let cup_top = surface - 6.0 * z;
    let cup_hw = TILE_W / 2.0 * z * 0.08;
    cr.save().unwrap();
    cr.translate(sx, cup_top);
    cr.scale(cup_hw / (3.0 * z), cup_hw / (6.0 * z));
    cr.arc(0.0, 0.0, 3.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.25, 0.15, 0.05);
    let _ = cr.fill();

    // Steam wisps above cup
    cr.set_source_rgba(0.85, 0.85, 0.85, 0.3);
    cr.set_line_width(0.8 * z);
    for i in 0..3 {
        let ox = (i as f64 - 1.0) * 2.0 * z;
        cr.move_to(sx + ox, cup_top - 1.0 * z);
        cr.curve_to(
            sx + ox + 1.5 * z, cup_top - 4.0 * z,
            sx + ox - 1.5 * z, cup_top - 7.0 * z,
            sx + ox + 1.0 * z, cup_top - 10.0 * z,
        );
        let _ = cr.stroke();
    }

    // Brand/logo on right face of machine
    right_face_rect(cr, sx, surface, z, 0.38, 0.38, 22.0, 0.12, 0.30, 0.18, 0.82, 0.78, 0.78, 0.76);
}

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let wr = 0.90;
    let hr = 0.75;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    // Seat — wide chunky navy block
    iso_block(cr, sx, sy, z, wr, hr, 14.0, 0.24, 0.30, 0.48);

    // Backrest — stacked on top, shallower depth
    let seat_top = sy - 14.0 * z;
    iso_block(cr, sx, seat_top, z, wr, 0.25, 20.0, 0.20, 0.24, 0.40);

    // Armrest details on faces (raised strips)
    left_face_rect(cr, sx, sy, z, wr, hr, 14.0, 0.0, 0.85, 0.0, 0.14, 0.18, 0.22, 0.38);
    right_face_rect(cr, sx, sy, z, wr, hr, 14.0, 0.0, 0.85, 0.86, 1.0, 0.22, 0.28, 0.44);

    // Seat cushion dividers on top face
    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;
    cr.set_source_rgba(0.12, 0.16, 0.28, 0.45);
    cr.set_line_width(1.0 * z);
    // Two divider lines
    for i in 1..3 {
        let t = i as f64 / 3.0;
        let x0 = sx - hw + hw * 2.0 * t;
        let y0 = seat_top - hh + hh * 2.0 * t;
        cr.move_to(x0, y0 - 3.0 * z);
        cr.line_to(x0 - 5.0 * z, y0 + 3.0 * z);
        let _ = cr.stroke();
    }

    // Throw pillow
    iso_block(cr, sx, seat_top, z, 0.15, 0.15, 6.0, 0.90, 0.60, 0.30);
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.35, 0.35);

    // Pot — chunky terracotta block
    iso_block(cr, sx, sy, z, 0.30, 0.30, 14.0, 0.65, 0.38, 0.20);
    // Pot rim
    iso_block(cr, sx, sy - 14.0 * z, z, 0.34, 0.34, 2.0, 0.58, 0.32, 0.16);

    // Trunk
    let pot_top = sy - 16.0 * z;
    cr.set_source_rgb(0.40, 0.28, 0.14);
    cr.set_line_width(3.0 * z);
    cr.move_to(sx, pot_top);
    cr.line_to(sx, sy - 42.0 * z);
    let _ = cr.stroke();

    // Branches
    cr.set_line_width(2.0 * z);
    cr.move_to(sx, sy - 36.0 * z);
    cr.line_to(sx + 9.0 * z, sy - 44.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx, sy - 32.0 * z);
    cr.line_to(sx - 8.0 * z, sy - 41.0 * z);
    let _ = cr.stroke();

    // Foliage — turquoise/cyan spheres with highlight + shadow
    let leaves: [(f64, f64, f64, f64); 7] = [
        (0.0, -52.0, 14.0, 0.88),
        (-10.0, -46.0, 11.0, 0.75),
        (11.0, -48.0, 11.0, 0.92),
        (-7.0, -56.0, 10.0, 0.95),
        (8.0, -54.0, 10.0, 0.82),
        (-12.0, -40.0, 9.0, 0.72),
        (13.0, -42.0, 9.0, 0.85),
    ];
    for (dx, dy, radius, shade) in &leaves {
        // Shadow layer
        cr.arc(sx + dx * z + 1.0 * z, sy + dy * z + 1.0 * z, radius * z, 0.0, TAU);
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.12);
        let _ = cr.fill();
        // Main leaf
        cr.arc(sx + dx * z, sy + dy * z, radius * z, 0.0, TAU);
        cr.set_source_rgb(0.12 * shade, 0.74 * shade, 0.70 * shade);
        let _ = cr.fill();
        // Highlight on upper portion
        cr.arc(sx + dx * z - 1.0 * z, sy + dy * z - 1.5 * z, radius * z * 0.5, 0.0, TAU);
        cr.set_source_rgba(0.40, 0.92, 0.88, 0.18);
        let _ = cr.fill();
    }
}

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let h = 58.0;
    let wr = 0.60;
    let hr = 0.55;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    // Bright purple cabinet (visible on dark floor)
    iso_block(cr, sx, sy, z, wr, hr, h, 0.50, 0.15, 0.62);

    // Bright yellow marquee on left face
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.02, 0.10, 0.05, 0.95, 1.0, 0.90, 0.15);
    // Marquee highlight
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.03, 0.06, 0.10, 0.90, 1.0, 0.95, 0.40);

    // Screen bezel (dark frame)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.11, 0.52, 0.05, 0.95, 0.04, 0.04, 0.06);

    // CRT screen (bright green glow)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.13, 0.50, 0.09, 0.91, 0.02, 0.18, 0.02);

    // Screen glow effect (bright overlay)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.14, 0.49, 0.10, 0.90, 0.05, 0.28, 0.05);

    // Game graphics on screen
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.18, 0.26, 0.25, 0.75, 0.25, 0.85, 0.25);
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.30, 0.38, 0.15, 0.55, 0.85, 0.85, 0.15);
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.40, 0.46, 0.40, 0.80, 0.85, 0.30, 0.15);

    // Control panel (lighter)
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.54, 0.68, 0.06, 0.94, 0.24, 0.24, 0.28);

    // Buttons on control panel
    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;
    let bh = h * z;
    let btns = [
        (0.28, (0.90, 0.15, 0.15)),
        (0.42, (0.15, 0.15, 0.90)),
        (0.56, (0.15, 0.85, 0.15)),
        (0.70, (0.90, 0.90, 0.15)),
    ];
    for (t, (br, bg, bb)) in btns {
        let bx = sx - hw + t * hw;
        let by = sy - bh + 0.62 * bh + t * hh;
        // Button shadow
        cr.arc(bx + 0.5 * z, by + 0.5 * z, 2.0 * z, 0.0, TAU);
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.3);
        let _ = cr.fill();
        // Button
        cr.arc(bx, by, 2.0 * z, 0.0, TAU);
        cr.set_source_rgb(br, bg, bb);
        let _ = cr.fill();
        // Button highlight
        cr.arc(bx - 0.3 * z, by - 0.3 * z, 1.0 * z, 0.0, TAU);
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
        let _ = cr.fill();
    }

    // Coin slot
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.74, 0.78, 0.32, 0.68, 0.65, 0.60, 0.15);
    // Coin slot opening
    left_face_rect(cr, sx, sy, z, wr, hr, h, 0.75, 0.77, 0.42, 0.58, 0.08, 0.08, 0.10);

    // Side art on right face (vibrant)
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.10, 0.88, 0.08, 0.92, 0.42, 0.12, 0.55);
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.25, 0.65, 0.20, 0.80, 0.55, 0.18, 0.65);
    // Lightning bolt shape
    right_face_rect(cr, sx, sy, z, wr, hr, h, 0.35, 0.45, 0.40, 0.60, 1.0, 0.90, 0.15);

    // Screen glow on ground (ambient light)
    cr.save().unwrap();
    cr.translate(sx - 4.0 * z, sy + 4.0 * z);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 10.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(0.10, 0.50, 0.10, 0.08);
    let _ = cr.fill();
}

fn draw_treadmill(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.82, 0.50);

    // Base platform
    iso_block(cr, sx, sy, z, 0.82, 0.50, 10.0, 0.22, 0.22, 0.24);

    // Belt surface on top
    let belt_top = sy - 10.0 * z;
    iso_block(cr, sx, belt_top, z, 0.78, 0.46, 2.0, 0.24, 0.24, 0.26);

    // Belt tread lines on top face
    let hw = TILE_W / 2.0 * z * 0.78;
    let hh = TILE_H / 2.0 * z * 0.46;
    let bt = belt_top - 2.0 * z;
    cr.set_source_rgba(0.16, 0.16, 0.18, 0.5);
    cr.set_line_width(0.6 * z);
    for i in 1..6 {
        let t = i as f64 / 6.0;
        let x0 = sx - hw * (1.0 - t);
        let y0 = bt - hh * (1.0 - t) + hh * t;
        let x1 = sx + hw * t;
        let y1 = bt - hh * t + hh * (1.0 - t);
        cr.move_to(x0, y0);
        cr.line_to(x1, y1);
        let _ = cr.stroke();
    }

    // Upright posts (thin pillars, only slight horizontal offset)
    iso_block(cr, sx - 8.0 * z, belt_top + 2.0 * z, z, 0.04, 0.04, 42.0, 0.52, 0.52, 0.55);
    iso_block(cr, sx + 8.0 * z, belt_top, z, 0.04, 0.04, 42.0, 0.52, 0.52, 0.55);

    // Console display at top center
    let console_y = belt_top - 38.0 * z;
    iso_block(cr, sx, console_y, z, 0.35, 0.10, 8.0, 0.12, 0.12, 0.14);

    // Screen on console left face (orange/red readout)
    left_face_rect(cr, sx, console_y, z, 0.35, 0.10, 8.0, 0.10, 0.88, 0.10, 0.90, 0.10, 0.55, 0.30);
}

fn draw_whiteboard(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.80, 0.20);

    // Two thin leg posts
    iso_block(cr, sx - 6.0 * z, sy + 2.0 * z, z, 0.04, 0.04, 48.0, 0.45, 0.45, 0.48);
    iso_block(cr, sx + 6.0 * z, sy, z, 0.04, 0.04, 48.0, 0.45, 0.45, 0.48);

    // Board — wide white block
    let board_y = sy - 18.0 * z;
    iso_block(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.95, 0.95, 0.97);

    // Frame border on left face (grey strips)
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.0, 1.0, 0.0, 0.05, 0.50, 0.50, 0.52);
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.0, 1.0, 0.95, 1.0, 0.50, 0.50, 0.52);
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.0, 0.04, 0.0, 1.0, 0.50, 0.50, 0.52);
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.96, 1.0, 0.0, 1.0, 0.50, 0.50, 0.52);

    // Diagram content
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.15, 0.38, 0.10, 0.42, 0.72, 0.18, 0.18);
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.15, 0.38, 0.55, 0.88, 0.18, 0.18, 0.72);
    left_face_rect(cr, sx, board_y, z, 0.80, 0.08, 30.0, 0.52, 0.72, 0.28, 0.72, 0.18, 0.62, 0.18);

    // Marker tray
    iso_block(cr, sx, board_y + 1.0 * z, z, 0.72, 0.08, 2.0, 0.44, 0.44, 0.46);
}

fn draw_weight_bench(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.75, 0.40);

    // Frame base (metal)
    iso_block(cr, sx, sy, z, 0.75, 0.40, 6.0, 0.38, 0.38, 0.40);

    // Bench pad on frame
    iso_block(cr, sx, sy - 6.0 * z, z, 0.65, 0.28, 5.0, 0.14, 0.14, 0.16);

    // Rack uprights (thin posts, moderate offset)
    iso_block(cr, sx - 8.0 * z, sy, z, 0.05, 0.05, 50.0, 0.45, 0.45, 0.48);
    iso_block(cr, sx + 8.0 * z, sy - 2.0 * z, z, 0.05, 0.05, 50.0, 0.45, 0.45, 0.48);

    // Barbell — long horizontal block across top
    let bar_y = sy - 46.0 * z;
    iso_block(cr, sx, bar_y, z, 0.90, 0.03, 2.0, 0.55, 0.55, 0.58);

    // Weight plates on each end
    iso_block(cr, sx - 12.0 * z, bar_y - 1.0 * z, z, 0.06, 0.06, 7.0, 0.14, 0.14, 0.16);
    iso_block(cr, sx + 12.0 * z, bar_y - 3.0 * z, z, 0.06, 0.06, 7.0, 0.14, 0.14, 0.16);
}

fn draw_yoga_mat(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let color_pick = ((sx * 7.0 + sy * 13.0) as i32).unsigned_abs() % 3;
    let (mr, mg, mb) = match color_pick {
        0 => (0.18, 0.68, 0.62),
        1 => (0.58, 0.32, 0.68),
        _ => (0.22, 0.58, 0.78),
    };
    // Very thin block — flat on the floor
    iso_block(cr, sx, sy, z, 0.82, 0.90, 1.5, mr, mg, mb);
}

fn draw_floor_lamp(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.25, 0.25);

    // Circular base
    iso_block(cr, sx, sy, z, 0.22, 0.22, 3.0, 0.42, 0.42, 0.45);

    // Pole — thin tall block, centered
    iso_block(cr, sx, sy - 3.0 * z, z, 0.03, 0.03, 50.0, 0.52, 0.52, 0.55);

    // Lampshade — wider block at top
    let shade_y = sy - 53.0 * z;
    iso_block(cr, sx, shade_y, z, 0.30, 0.30, 10.0, 0.95, 0.90, 0.58);

    // Warm glow below shade
    cr.save().unwrap();
    cr.translate(sx, shade_y + 2.0 * z);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 16.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(1.0, 0.92, 0.65, 0.10);
    let _ = cr.fill();
}

fn draw_ping_pong_table(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.90, 0.70);

    // 4 legs (thin posts at corners, moderate offset)
    iso_block(cr, sx - 8.0 * z, sy + 2.0 * z, z, 0.05, 0.05, 22.0, 0.38, 0.38, 0.40);
    iso_block(cr, sx + 8.0 * z, sy - 2.0 * z, z, 0.05, 0.05, 22.0, 0.38, 0.38, 0.40);
    iso_block(cr, sx - 3.0 * z, sy + 5.0 * z, z, 0.05, 0.05, 22.0, 0.38, 0.38, 0.40);
    iso_block(cr, sx + 3.0 * z, sy + 3.0 * z, z, 0.05, 0.05, 22.0, 0.38, 0.38, 0.40);

    // Table surface — big green block
    let table_y = sy - 22.0 * z;
    iso_block(cr, sx, table_y, z, 0.90, 0.70, 3.0, 0.14, 0.58, 0.24);

    // White border on top face
    let thw = TILE_W / 2.0 * z * 0.85;
    let thh = TILE_H / 2.0 * z * 0.65;
    let tt = table_y - 3.0 * z;
    cr.set_source_rgb(0.95, 0.95, 0.95);
    cr.set_line_width(1.0 * z);
    cr.move_to(sx, tt - thh);
    cr.line_to(sx + thw, tt);
    cr.line_to(sx, tt + thh);
    cr.line_to(sx - thw, tt);
    cr.close_path();
    let _ = cr.stroke();

    // Center line on top face (iso diagonal)
    cr.move_to(sx - thw * 0.5, tt - thh * 0.5);
    cr.line_to(sx + thw * 0.5, tt + thh * 0.5);
    let _ = cr.stroke();

    // Net — thin white block at center
    iso_block(cr, sx, tt + 1.0 * z, z, 0.03, 0.50, 5.0, 0.92, 0.92, 0.92);
}

fn draw_small_armchair(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.60, 0.60);

    // Chunky seat block (burnt orange/leather)
    let wr = 0.60;
    let hr = 0.60;
    iso_block(cr, sx, sy, z, wr, hr, 14.0, 0.82, 0.38, 0.18);

    // Backrest stacked on top
    let seat_top = sy - 14.0 * z;
    iso_block(cr, sx, seat_top, z, wr, 0.18, 18.0, 0.75, 0.32, 0.14);

    // Armrest details as face strips
    left_face_rect(cr, sx, sy, z, wr, hr, 14.0, 0.0, 0.6, 0.0, 0.15, 0.70, 0.30, 0.12);
    right_face_rect(cr, sx, sy, z, wr, hr, 14.0, 0.0, 0.6, 0.85, 1.0, 0.75, 0.32, 0.14);

    // Cushion indent on top face
    let hw = TILE_W / 2.0 * z * wr * 0.6;
    let hh = TILE_H / 2.0 * z * hr * 0.6;
    cr.move_to(sx, seat_top - hh);
    cr.line_to(sx + hw, seat_top);
    cr.line_to(sx, seat_top + hh);
    cr.line_to(sx - hw, seat_top);
    cr.close_path();
    cr.set_source_rgba(0.65, 0.28, 0.10, 0.4);
    let _ = cr.fill();
}
