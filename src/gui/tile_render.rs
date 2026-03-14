use crate::gui::iso::{TILE_H, TILE_W, WALL_HEIGHT};
use crate::world::{FloorKind, Tile, WallKind};
use std::f64::consts::TAU;

pub fn draw_tile(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    tile: &Tile,
    zoom: f64,
    gx: u16,
    gy: u16,
) {
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
            draw_yoga_mat(cr, sx, sy, zoom, gx, gy);
        }
        Tile::FloorLamp => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_floor_lamp(cr, sx, sy, zoom);
        }
        Tile::PingPongTableLeft => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_ping_pong_half(cr, sx, sy, zoom, true);
        }
        Tile::PingPongTableRight => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.32, 0.30, 0.30);
            draw_ping_pong_half(cr, sx, sy, zoom, false);
        }
        Tile::SmallArmchair => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.42, 0.38, 0.50);
            draw_small_armchair(cr, sx, sy, zoom);
        }
        Tile::Whiteboard => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.78, 0.62, 0.42);
            draw_whiteboard(cr, sx, sy, zoom);
        }
        Tile::KitchenCounter => {
            draw_floor_diamond(cr, sx, sy, zoom, 0.88, 0.84, 0.78);
            draw_kitchen_counter(cr, sx, sy, zoom, gx, gy);
        }
    }
}

// ─── Isometric helpers ───

/// Helper: draw an isometric diamond (flat surface) at a given height offset.
/// w_ratio/h_ratio control size relative to tile. Returns the 4 corner points.
#[allow(clippy::too_many_arguments)]
fn iso_diamond(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    lift: f64,
    r: f64,
    g: f64,
    b: f64,
) -> [(f64, f64); 4] {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let y = sy - lift * z;
    let pts = [
        (sx, y - hh), // back
        (sx + hw, y), // right
        (sx, y + hh), // front
        (sx - hw, y), // left
    ];
    cr.move_to(pts[0].0, pts[0].1);
    cr.line_to(pts[1].0, pts[1].1);
    cr.line_to(pts[2].0, pts[2].1);
    cr.line_to(pts[3].0, pts[3].1);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
    pts
}

/// Helper: draw left face of an iso shape (parallelogram from top diamond to bottom diamond)
#[allow(clippy::too_many_arguments)]
fn iso_left_face(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    top_lift: f64,
    bot_lift: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let yt = sy - top_lift * z;
    let yb = sy - bot_lift * z;
    cr.move_to(sx - hw, yt); // top-left
    cr.line_to(sx, yt + hh); // top-front
    cr.line_to(sx, yb + hh); // bot-front
    cr.line_to(sx - hw, yb); // bot-left
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
}

/// Helper: draw right face of an iso shape
#[allow(clippy::too_many_arguments)]
fn iso_right_face(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
    top_lift: f64,
    bot_lift: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let yt = sy - top_lift * z;
    let yb = sy - bot_lift * z;
    cr.move_to(sx + hw, yt); // top-right
    cr.line_to(sx, yt + hh); // top-front
    cr.line_to(sx, yb + hh); // bot-front
    cr.line_to(sx + hw, yb); // bot-right
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();
}

/// Draw a complete iso solid: top diamond + left face + right face + outline
/// High contrast shading: left=0.40, right=0.65, top=1.0 for strong 3D pop
#[allow(clippy::too_many_arguments)]
fn iso_solid(
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
    let top = height;
    // HIGH contrast between faces for 3D look
    iso_left_face(
        cr,
        sx,
        sy,
        z,
        w_ratio,
        h_ratio,
        top,
        0.0,
        r * 0.40,
        g * 0.40,
        b * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        z,
        w_ratio,
        h_ratio,
        top,
        0.0,
        r * 0.65,
        g * 0.65,
        b * 0.65,
    );
    iso_diamond(cr, sx, sy, z, w_ratio, h_ratio, top, r, g, b);
    // Strong outline for definition
    let hw = TILE_W / 2.0 * z * w_ratio;
    let hh = TILE_H / 2.0 * z * h_ratio;
    let yt = sy - top * z;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.40);
    cr.set_line_width(0.8 * z);
    // Silhouette
    cr.move_to(sx - hw, yt);
    cr.line_to(sx, yt - hh);
    cr.line_to(sx + hw, yt);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    let _ = cr.stroke();
    // Front seam
    cr.move_to(sx, yt + hh);
    cr.line_to(sx, sy + hh);
    let _ = cr.stroke();
    // Left-top seam
    cr.move_to(sx - hw, yt);
    cr.line_to(sx, yt + hh);
    let _ = cr.stroke();
    // Right-top seam
    cr.move_to(sx + hw, yt);
    cr.line_to(sx, yt + hh);
    let _ = cr.stroke();
    // Highlight on top back edges
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.12);
    cr.set_line_width(1.0 * z);
    cr.move_to(sx - hw, yt);
    cr.line_to(sx, yt - hh);
    cr.line_to(sx + hw, yt);
    let _ = cr.stroke();
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
    let (r, g, b) = match kind {
        WallKind::Solid => (0.50, 0.42, 0.34),
        WallKind::Window => (0.48, 0.44, 0.38),
    };

    // Full-tile footprint, short height = continuous low wall like reference
    let wh = WALL_HEIGHT;

    iso_left_face(
        cr,
        sx,
        sy,
        zoom,
        1.0,
        1.0,
        wh,
        0.0,
        r * 0.38,
        g * 0.38,
        b * 0.38,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        zoom,
        1.0,
        1.0,
        wh,
        0.0,
        r * 0.62,
        g * 0.62,
        b * 0.62,
    );
    iso_diamond(cr, sx, sy, zoom, 1.0, 1.0, wh, r * 0.90, g * 0.90, b * 0.90);

    // Outline
    let hw = TILE_W / 2.0 * zoom;
    let hh = TILE_H / 2.0 * zoom;
    let whz = wh * zoom;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.18);
    cr.set_line_width(0.5);
    cr.move_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx + hw, sy);
    let _ = cr.stroke();
    cr.move_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - whz);
    let _ = cr.stroke();

    if matches!(kind, WallKind::Window) {
        // Window on right face
        let inset = 3.0 * zoom;
        let pane_h = whz * 0.50;
        let pane_top = whz * 0.20;
        let rx0 = sx + inset * 0.3;
        let rx1 = sx + hw - inset * 0.5;
        let ry_base = sy - whz + pane_top;
        cr.move_to(rx0, ry_base + hh * 0.85);
        cr.line_to(rx1, ry_base + hh * 0.15);
        cr.line_to(rx1, ry_base + hh * 0.15 + pane_h);
        cr.line_to(rx0, ry_base + hh * 0.85 + pane_h);
        cr.close_path();
        cr.set_source_rgba(0.55, 0.78, 0.95, 0.45);
        let _ = cr.fill();
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
}

// ─── Ground shadow ───

fn draw_ground_shadow(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    z: f64,
    w_ratio: f64,
    h_ratio: f64,
) {
    let hw = TILE_W / 2.0 * z * w_ratio * 1.1;
    let hh = TILE_H / 2.0 * z * h_ratio * 1.1;
    let ox = 3.0 * z;
    let oy = 2.0 * z;
    cr.move_to(sx + ox, sy + oy - hh);
    cr.line_to(sx + ox + hw, sy + oy);
    cr.line_to(sx + ox, sy + oy + hh);
    cr.line_to(sx + ox - hw, sy + oy);
    cr.close_path();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
    let _ = cr.fill();
}

// ─── Face detail helpers ───

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

// ─── Desk — L-shaped office workstation with monitor facing front ───

fn draw_desk(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.88, 0.70);

    let desk_h = 12.0; // desk surface height — low like a real desk

    // Desk legs — 4 thin lines at corners
    cr.set_source_rgb(0.45, 0.32, 0.18);
    cr.set_line_width(1.5 * z);
    let hw = TILE_W / 2.0 * z * 0.85;
    let hh = TILE_H / 2.0 * z * 0.65;
    let ins = 0.80;
    let leg_pts = [
        (sx, sy - hh * ins),
        (sx + hw * ins, sy),
        (sx, sy + hh * ins),
        (sx - hw * ins, sy),
    ];
    for &(lx, ly) in &leg_pts {
        cr.move_to(lx, ly);
        cr.line_to(lx, ly - desk_h * z);
        let _ = cr.stroke();
    }

    // Desk surface — wide, thin slab
    let slab = 2.0;
    iso_left_face(
        cr,
        sx,
        sy,
        z,
        0.88,
        0.70,
        desk_h + slab,
        desk_h,
        0.52 * 0.40,
        0.38 * 0.40,
        0.22 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        z,
        0.88,
        0.70,
        desk_h + slab,
        desk_h,
        0.52 * 0.65,
        0.38 * 0.65,
        0.22 * 0.65,
    );
    iso_diamond(cr, sx, sy, z, 0.88, 0.70, desk_h + slab, 0.58, 0.42, 0.25);

    // Monitor — iso_solid on desk, screen visible on BOTH faces
    let top = sy - (desk_h + slab) * z;
    iso_solid(cr, sx, top, z, 0.40, 0.08, 12.0, 0.12, 0.12, 0.16);
    // Screen on left face
    left_face_rect(
        cr, sx, top, z, 0.40, 0.08, 12.0, 0.06, 0.92, 0.06, 0.94, 0.10, 0.18, 0.30,
    );
    // Screen on right face
    right_face_rect(
        cr, sx, top, z, 0.40, 0.08, 12.0, 0.06, 0.92, 0.06, 0.94, 0.10, 0.18, 0.30,
    );
    // Code lines on both faces
    let colors = [(0.45, 0.82, 0.45), (0.82, 0.72, 0.40), (0.55, 0.68, 0.88)];
    for (i, &(lr, lg, lb)) in colors.iter().enumerate() {
        let t = (i as f64 + 1.0) / 4.5;
        left_face_rect(
            cr,
            sx,
            top,
            z,
            0.40,
            0.08,
            12.0,
            0.15 + t * 0.55,
            0.18 + t * 0.55,
            0.12,
            0.12 + 0.35 - i as f64 * 0.08,
            lr,
            lg,
            lb,
        );
        right_face_rect(
            cr,
            sx,
            top,
            z,
            0.40,
            0.08,
            12.0,
            0.15 + t * 0.55,
            0.18 + t * 0.55,
            0.12,
            0.12 + 0.35 - i as f64 * 0.08,
            lr,
            lg,
            lb,
        );
    }

    // Keyboard on desk
    iso_diamond(
        cr,
        sx + 2.0 * z,
        sy,
        z,
        0.30,
        0.18,
        desk_h + slab + 0.5,
        0.25,
        0.25,
        0.28,
    );
}

// ─── Couch — low flat shape, not a cube ───

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.85, 0.65);

    // Couch is wide (0.88) but shallow depth (0.40) and low height
    let wr = 0.88;
    let hr = 0.40; // shallow, not square
    let seat_h = 6.0;

    // Seat — wide, shallow, low
    iso_left_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        seat_h,
        0.0,
        0.20 * 0.40,
        0.24 * 0.40,
        0.40 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        seat_h,
        0.0,
        0.20 * 0.65,
        0.24 * 0.65,
        0.40 * 0.65,
    );
    iso_diamond(cr, sx, sy, z, wr, hr, seat_h, 0.22, 0.26, 0.42);

    // Backrest — same width but even shallower and taller
    let back_cy = sy - TILE_H / 2.0 * z * 0.22;
    let back_h = 12.0;
    iso_left_face(
        cr,
        sx,
        back_cy,
        z,
        wr,
        0.12,
        back_h,
        0.0,
        0.16 * 0.40,
        0.20 * 0.40,
        0.36 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        back_cy,
        z,
        wr,
        0.12,
        back_h,
        0.0,
        0.16 * 0.65,
        0.20 * 0.65,
        0.36 * 0.65,
    );
    iso_diamond(cr, sx, back_cy, z, wr, 0.12, back_h, 0.18, 0.22, 0.38);

    // Outline
    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.20);
    cr.set_line_width(0.5 * z);
    cr.move_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx + hw, sy);
    let _ = cr.stroke();

    // Cushion lines on seat
    cr.set_source_rgba(0.10, 0.14, 0.26, 0.30);
    cr.set_line_width(0.6 * z);
    let ct = sy - seat_h * z;
    let chw = TILE_W / 2.0 * z * 0.80;
    for i in 1..3 {
        let t = i as f64 / 3.0;
        let lx = sx - chw + chw * 2.0 * t;
        cr.move_to(lx, ct - 1.0 * z);
        cr.line_to(lx - 3.0 * z, ct + 2.0 * z);
        let _ = cr.stroke();
    }
}

// ─── Plant — small potted plant ───

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.22, 0.22);

    // Pot
    iso_solid(cr, sx, sy, z, 0.20, 0.20, 8.0, 0.55, 0.35, 0.18);

    // Small spiky leaves
    let pot_top = sy - 8.0 * z;
    let leaves: [(f64, f64); 5] = [
        (0.0, -16.0),
        (-4.0, -14.0),
        (4.0, -14.0),
        (-2.5, -12.0),
        (2.5, -12.0),
    ];
    for &(dx, dy) in &leaves {
        cr.move_to(sx, pot_top);
        cr.line_to(sx + dx * z - 1.5 * z, pot_top + dy * z * 0.5);
        cr.line_to(sx + dx * z, pot_top + dy * z);
        cr.line_to(sx + dx * z + 1.5 * z, pot_top + dy * z * 0.5);
        cr.close_path();
        cr.set_source_rgb(0.15, 0.60, 0.52);
        let _ = cr.fill();
    }
}

// ─── Vending Machine ───

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let h = 50.0;
    let wr = 0.55;
    let hr = 0.35;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);
    iso_solid(cr, sx, sy, z, wr, hr, h, 0.78, 0.16, 0.16);

    // Glass panel — on LEFT face (faces down-left toward viewer)
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.06, 0.65, 0.06, 0.94, 0.18, 0.22, 0.30,
    );
    // Shelf lines
    for row in 0..4 {
        let y_frac = 0.08 + row as f64 * 0.14;
        left_face_rect(
            cr,
            sx,
            sy,
            z,
            wr,
            hr,
            h,
            y_frac,
            y_frac + 0.01,
            0.08,
            0.92,
            0.40,
            0.40,
            0.45,
        );
    }
    // Cans
    let can_colors = [
        (0.92, 0.28, 0.18),
        (0.18, 0.58, 0.92),
        (0.18, 0.75, 0.28),
        (0.95, 0.75, 0.08),
    ];
    for row in 0..4 {
        let y_top = 0.10 + row as f64 * 0.14;
        for col in 0..3 {
            let x_left = 0.14 + col as f64 * 0.26;
            let ci = (row * 3 + col) as usize % can_colors.len();
            let (cr2, cg, cb) = can_colors[ci];
            left_face_rect(
                cr,
                sx,
                sy,
                z,
                wr,
                hr,
                h,
                y_top,
                y_top + 0.10,
                x_left,
                x_left + 0.18,
                cr2,
                cg,
                cb,
            );
        }
    }
    // Dispensing slot
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.70, 0.80, 0.22, 0.78, 0.05, 0.05, 0.08,
    );
    // Price display
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.85, 0.92, 0.30, 0.70, 0.02, 0.02, 0.04,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.86, 0.91, 0.32, 0.68, 0.08, 0.65, 0.12,
    );

    // Same details on RIGHT face — visible from any angle
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.06, 0.65, 0.06, 0.94, 0.18, 0.22, 0.30,
    );
    for row in 0..4 {
        let y_frac = 0.08 + row as f64 * 0.14;
        right_face_rect(
            cr,
            sx,
            sy,
            z,
            wr,
            hr,
            h,
            y_frac,
            y_frac + 0.01,
            0.08,
            0.92,
            0.40,
            0.40,
            0.45,
        );
    }
    for row in 0..4 {
        let y_top = 0.10 + row as f64 * 0.14;
        for col in 0..3 {
            let x_left = 0.14 + col as f64 * 0.26;
            let ci = (row * 3 + col) as usize % can_colors.len();
            let (cr2, cg, cb) = can_colors[ci];
            right_face_rect(
                cr,
                sx,
                sy,
                z,
                wr,
                hr,
                h,
                y_top,
                y_top + 0.10,
                x_left,
                x_left + 0.18,
                cr2,
                cg,
                cb,
            );
        }
    }
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.70, 0.80, 0.22, 0.78, 0.05, 0.05, 0.08,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.85, 0.92, 0.30, 0.70, 0.02, 0.02, 0.04,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.86, 0.91, 0.32, 0.68, 0.08, 0.65, 0.12,
    );
}

// ─── Coffee Machine (on counter) ───

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let wr = 0.85;
    let hr = 0.45; // not square — narrower depth
    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    // Counter base
    iso_solid(cr, sx, sy, z, wr, hr, 22.0, 0.25, 0.25, 0.28);
    // Cabinet doors
    left_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.05, 0.92, 0.04, 0.46, 0.18, 0.18, 0.22,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.05, 0.92, 0.54, 0.96, 0.18, 0.18, 0.22,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.44, 0.50, 0.38, 0.46, 0.55, 0.55, 0.60,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.44, 0.50, 0.88, 0.94, 0.55, 0.55, 0.60,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.05, 0.92, 0.04, 0.46, 0.22, 0.22, 0.26,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, 22.0, 0.05, 0.92, 0.54, 0.96, 0.22, 0.22, 0.26,
    );

    // Countertop
    let top = sy - 22.0 * z;
    iso_diamond(cr, sx, sy, z, 0.88, 0.48, 23.0, 0.58, 0.55, 0.50);

    // Coffee machine on counter — small box
    let surface = top - 1.0 * z;
    iso_solid(cr, sx, surface, z, 0.35, 0.35, 18.0, 0.88, 0.88, 0.86);
    // Display — on BOTH faces
    left_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.10, 0.28, 0.10, 0.90, 0.02, 0.02, 0.04,
    );
    left_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.13, 0.25, 0.15, 0.85, 0.10, 0.72, 0.18,
    );
    left_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.45, 0.78, 0.18, 0.82, 0.12, 0.12, 0.15,
    );
    right_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.10, 0.28, 0.10, 0.90, 0.02, 0.02, 0.04,
    );
    right_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.13, 0.25, 0.15, 0.85, 0.10, 0.72, 0.18,
    );
    right_face_rect(
        cr, sx, surface, z, 0.35, 0.35, 18.0, 0.45, 0.78, 0.18, 0.82, 0.12, 0.12, 0.15,
    );

    // Coffee cup — just a tiny shape
    let cup_y = surface - 1.0 * z;
    cr.save().unwrap();
    cr.translate(sx + 4.0 * z, cup_y);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 2.5 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgb(0.92, 0.92, 0.88);
    let _ = cr.fill();
    // Steam
    cr.set_source_rgba(0.85, 0.85, 0.85, 0.25);
    cr.set_line_width(0.6 * z);
    for i in 0..2 {
        let ox = (i as f64 - 0.5) * 2.0 * z;
        cr.move_to(sx + 4.0 * z + ox, cup_y - 1.0 * z);
        cr.curve_to(
            sx + 4.0 * z + ox + 1.0 * z,
            cup_y - 3.0 * z,
            sx + 4.0 * z + ox - 1.0 * z,
            cup_y - 5.0 * z,
            sx + 4.0 * z + ox + 0.5 * z,
            cup_y - 7.0 * z,
        );
        let _ = cr.stroke();
    }
}

// ─── Armchair — small low shape ───

fn draw_small_armchair(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.50, 0.40);

    // Seat — wider than deep, low
    let wr = 0.50;
    let hr = 0.35;
    let seat_h = 5.0;
    iso_left_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        seat_h,
        0.0,
        0.75 * 0.40,
        0.34 * 0.40,
        0.14 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        seat_h,
        0.0,
        0.75 * 0.65,
        0.34 * 0.65,
        0.14 * 0.65,
    );
    iso_diamond(cr, sx, sy, z, wr, hr, seat_h, 0.82, 0.38, 0.18);

    // Backrest — thin panel
    let back_cy = sy - TILE_H / 2.0 * z * 0.18;
    let back_h = 10.0;
    iso_left_face(
        cr,
        sx,
        back_cy,
        z,
        wr,
        0.10,
        back_h,
        0.0,
        0.70 * 0.40,
        0.28 * 0.40,
        0.10 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        back_cy,
        z,
        wr,
        0.10,
        back_h,
        0.0,
        0.70 * 0.65,
        0.28 * 0.65,
        0.10 * 0.65,
    );
    iso_diamond(cr, sx, back_cy, z, wr, 0.10, back_h, 0.75, 0.32, 0.14);

    // Outline
    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.18);
    cr.set_line_width(0.5 * z);
    cr.move_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx + hw, sy);
    let _ = cr.stroke();
}

// ─── Arcade ───

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    let h = 50.0;
    let wr = 0.45;
    let hr = 0.25;
    draw_ground_shadow(cr, sx, sy, z, wr, hr);
    iso_solid(cr, sx, sy, z, wr, hr, h, 0.50, 0.15, 0.62);

    // Marquee — on LEFT face (faces down toward viewer)
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.02, 0.10, 0.05, 0.95, 1.0, 0.90, 0.15,
    );
    // Screen bezel
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.12, 0.50, 0.05, 0.95, 0.04, 0.04, 0.06,
    );
    // Screen
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.14, 0.48, 0.09, 0.91, 0.02, 0.18, 0.02,
    );
    // Game graphics
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.18, 0.26, 0.25, 0.75, 0.25, 0.85, 0.25,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.30, 0.38, 0.15, 0.55, 0.85, 0.85, 0.15,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.40, 0.46, 0.40, 0.80, 0.85, 0.30, 0.15,
    );
    // Control panel
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.54, 0.66, 0.06, 0.94, 0.24, 0.24, 0.28,
    );
    // Buttons — on left face
    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;
    let bh = h * z;
    let btns = [
        (0.28, (0.90, 0.15, 0.15)),
        (0.50, (0.15, 0.15, 0.90)),
        (0.72, (0.15, 0.85, 0.15)),
    ];
    for (t, (br, bg, bb)) in btns {
        let bx = sx - hw + t * hw;
        let by = sy - bh + 0.61 * bh + t * hh;
        cr.arc(bx, by, 1.8 * z, 0.0, TAU);
        cr.set_source_rgb(br, bg, bb);
        let _ = cr.fill();
    }
    // Coin slot
    left_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.72, 0.76, 0.35, 0.65, 0.65, 0.60, 0.15,
    );

    // Same details on RIGHT face — visible from any angle
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.02, 0.10, 0.05, 0.95, 1.0, 0.90, 0.15,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.12, 0.50, 0.05, 0.95, 0.04, 0.04, 0.06,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.14, 0.48, 0.09, 0.91, 0.02, 0.18, 0.02,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.18, 0.26, 0.25, 0.75, 0.25, 0.85, 0.25,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.30, 0.38, 0.15, 0.55, 0.85, 0.85, 0.15,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.40, 0.46, 0.40, 0.80, 0.85, 0.30, 0.15,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.54, 0.66, 0.06, 0.94, 0.24, 0.24, 0.28,
    );
    // Buttons on right face
    for (t, (br, bg, bb)) in btns {
        let bx = sx + t * hw;
        let by = sy - bh + 0.61 * bh + hh * (1.0 - t);
        cr.arc(bx, by, 1.8 * z, 0.0, TAU);
        cr.set_source_rgb(br, bg, bb);
        let _ = cr.fill();
    }
    right_face_rect(
        cr, sx, sy, z, wr, hr, h, 0.72, 0.76, 0.35, 0.65, 0.65, 0.60, 0.15,
    );
}

// ─── Treadmill — flat base + uprights ───

fn draw_treadmill(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.80, 0.48);

    // Base — flat
    iso_solid(cr, sx, sy, z, 0.80, 0.48, 6.0, 0.22, 0.22, 0.24);

    // Belt surface on top
    iso_diamond(cr, sx, sy, z, 0.75, 0.44, 7.0, 0.18, 0.18, 0.20);
    // Tread lines
    let hw = TILE_W / 2.0 * z * 0.72;
    let hh = TILE_H / 2.0 * z * 0.42;
    let bt = sy - 7.0 * z;
    cr.set_source_rgba(0.14, 0.14, 0.16, 0.4);
    cr.set_line_width(0.5 * z);
    for i in 1..5 {
        let t = i as f64 / 5.0;
        cr.move_to(sx - hw * (1.0 - t), bt - hh * (1.0 - t) + hh * t);
        cr.line_to(sx + hw * t, bt - hh * t + hh * (1.0 - t));
        let _ = cr.stroke();
    }

    // Uprights — thin lines
    cr.set_source_rgb(0.50, 0.50, 0.52);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 7.0 * z, sy + 1.0 * z);
    cr.line_to(sx - 7.0 * z, sy - 36.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 7.0 * z, sy - 1.0 * z);
    cr.line_to(sx + 7.0 * z, sy - 36.0 * z);
    let _ = cr.stroke();

    // Console display
    let cy = sy - 34.0 * z;
    cr.move_to(sx - 8.0 * z, cy);
    cr.line_to(sx + 8.0 * z, cy);
    cr.line_to(sx + 8.0 * z, cy - 5.0 * z);
    cr.line_to(sx - 8.0 * z, cy - 5.0 * z);
    cr.close_path();
    cr.set_source_rgb(0.10, 0.10, 0.12);
    let _ = cr.fill();
    // Display content
    cr.move_to(sx - 6.0 * z, cy - 4.0 * z);
    cr.line_to(sx + 6.0 * z, cy - 4.0 * z);
    cr.line_to(sx + 6.0 * z, cy - 1.0 * z);
    cr.line_to(sx - 6.0 * z, cy - 1.0 * z);
    cr.close_path();
    cr.set_source_rgb(0.10, 0.50, 0.28);
    let _ = cr.fill();
}

// ─── Whiteboard ───

fn draw_whiteboard(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.75, 0.15);

    // Two leg posts — thin lines
    cr.set_source_rgb(0.42, 0.42, 0.45);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 8.0 * z, sy + 2.0 * z);
    cr.line_to(sx - 8.0 * z, sy - 40.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 8.0 * z, sy);
    cr.line_to(sx + 8.0 * z, sy - 40.0 * z);
    let _ = cr.stroke();

    // Board — flat rectangle
    let by = sy - 15.0 * z;
    let bw = 18.0 * z;
    let bh = 24.0 * z;
    cr.move_to(sx - bw, by);
    cr.line_to(sx + bw, by);
    cr.line_to(sx + bw, by - bh);
    cr.line_to(sx - bw, by - bh);
    cr.close_path();
    cr.set_source_rgb(0.94, 0.94, 0.96);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(0.50, 0.50, 0.52);
    cr.set_line_width(1.0 * z);
    let _ = cr.stroke();

    // Diagram content
    cr.set_line_width(1.5 * z);
    // Red box
    cr.set_source_rgb(0.72, 0.18, 0.18);
    cr.move_to(sx - bw * 0.7, by - bh * 0.3);
    cr.line_to(sx - bw * 0.3, by - bh * 0.3);
    cr.line_to(sx - bw * 0.3, by - bh * 0.6);
    cr.line_to(sx - bw * 0.7, by - bh * 0.6);
    cr.close_path();
    let _ = cr.stroke();
    // Blue box
    cr.set_source_rgb(0.18, 0.18, 0.72);
    cr.move_to(sx + bw * 0.3, by - bh * 0.3);
    cr.line_to(sx + bw * 0.7, by - bh * 0.3);
    cr.line_to(sx + bw * 0.7, by - bh * 0.6);
    cr.line_to(sx + bw * 0.3, by - bh * 0.6);
    cr.close_path();
    let _ = cr.stroke();
    // Arrow between
    cr.set_source_rgb(0.18, 0.55, 0.18);
    cr.move_to(sx - bw * 0.25, by - bh * 0.45);
    cr.line_to(sx + bw * 0.25, by - bh * 0.45);
    let _ = cr.stroke();
}

// ─── Weight bench ───

fn draw_weight_bench(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.72, 0.38);

    // Frame base — flat
    iso_solid(cr, sx, sy, z, 0.72, 0.38, 5.0, 0.38, 0.38, 0.40);
    // Bench pad — flat dark
    iso_diamond(cr, sx, sy, z, 0.60, 0.25, 7.0, 0.14, 0.14, 0.16);

    // Rack uprights — thin lines
    cr.set_source_rgb(0.48, 0.48, 0.50);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 9.0 * z, sy + 1.0 * z);
    cr.line_to(sx - 9.0 * z, sy - 40.0 * z);
    let _ = cr.stroke();
    cr.move_to(sx + 9.0 * z, sy - 1.0 * z);
    cr.line_to(sx + 9.0 * z, sy - 40.0 * z);
    let _ = cr.stroke();

    // Barbell — horizontal line
    cr.set_source_rgb(0.55, 0.55, 0.58);
    cr.set_line_width(2.0 * z);
    cr.move_to(sx - 16.0 * z, sy - 36.0 * z);
    cr.line_to(sx + 16.0 * z, sy - 36.0 * z);
    let _ = cr.stroke();

    // Weight plates — small rectangles on ends
    cr.set_source_rgb(0.14, 0.14, 0.16);
    for dx in [-15.0_f64, 14.0] {
        cr.move_to(sx + dx * z - 2.0 * z, sy - 32.0 * z);
        cr.line_to(sx + dx * z + 2.0 * z, sy - 32.0 * z);
        cr.line_to(sx + dx * z + 2.0 * z, sy - 40.0 * z);
        cr.line_to(sx + dx * z - 2.0 * z, sy - 40.0 * z);
        cr.close_path();
        let _ = cr.fill();
    }
}

// ─── Yoga mat — flat on floor ───

fn draw_yoga_mat(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64, gx: u16, gy: u16) {
    let color_pick = ((gx as u32) * 7 + (gy as u32) * 13) % 3;
    let (mr, mg, mb) = match color_pick {
        0 => (0.18, 0.68, 0.62),
        1 => (0.58, 0.32, 0.68),
        _ => (0.22, 0.58, 0.78),
    };
    // Flat diamond, barely raised
    iso_diamond(cr, sx, sy, z, 0.82, 0.88, 0.5, mr, mg, mb);
    // Subtle outline
    let hw = TILE_W / 2.0 * z * 0.82;
    let hh = TILE_H / 2.0 * z * 0.88;
    let y = sy - 0.5 * z;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
    cr.set_line_width(0.5 * z);
    cr.move_to(sx, y - hh);
    cr.line_to(sx + hw, y);
    cr.line_to(sx, y + hh);
    cr.line_to(sx - hw, y);
    cr.close_path();
    let _ = cr.stroke();
}

// ─── Floor lamp ───

fn draw_floor_lamp(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64) {
    draw_ground_shadow(cr, sx, sy, z, 0.20, 0.20);

    // Base — small flat circle
    iso_diamond(cr, sx, sy, z, 0.18, 0.18, 1.0, 0.40, 0.40, 0.42);

    // Pole — thin line
    cr.set_source_rgb(0.50, 0.50, 0.52);
    cr.set_line_width(1.5 * z);
    cr.move_to(sx, sy - 1.0 * z);
    cr.line_to(sx, sy - 48.0 * z);
    let _ = cr.stroke();

    // Lampshade — trapezoid shape
    let shade_y = sy - 48.0 * z;
    cr.move_to(sx - 8.0 * z, shade_y);
    cr.line_to(sx + 8.0 * z, shade_y);
    cr.line_to(sx + 5.0 * z, shade_y - 8.0 * z);
    cr.line_to(sx - 5.0 * z, shade_y - 8.0 * z);
    cr.close_path();
    cr.set_source_rgb(0.92, 0.85, 0.55);
    let _ = cr.fill_preserve();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
    cr.set_line_width(0.5 * z);
    let _ = cr.stroke();

    // Warm glow
    cr.save().unwrap();
    cr.translate(sx, shade_y + 2.0 * z);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 12.0 * z, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(1.0, 0.92, 0.65, 0.08);
    let _ = cr.fill();
}

// ─── Ping pong table ───

/// Each half fills its own tile. Left = back half (x), Right = front half (x+1).
/// Together they form one rectangular ping pong table spanning 2 tiles.
fn draw_ping_pong_half(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64, is_left: bool) {
    let wr = 0.96;
    let hr = 0.96;
    let table_h = 14.0;
    let slab = 2.0;

    draw_ground_shadow(cr, sx, sy, z, wr, hr);

    let hw = TILE_W / 2.0 * z * wr;
    let hh = TILE_H / 2.0 * z * hr;

    // 2 legs per half — back and front corners on the outer side
    cr.set_source_rgb(0.36, 0.36, 0.38);
    cr.set_line_width(2.0 * z);
    let ins = 0.75;
    if is_left {
        // Left half: legs on left (outer) side
        let legs = [
            (sx - hw * ins, sy), // left corner
            (sx, sy - hh * ins), // back corner
        ];
        for &(lx, ly) in &legs {
            cr.move_to(lx, ly);
            cr.line_to(lx, ly - table_h * z);
            let _ = cr.stroke();
        }
    } else {
        // Right half: legs on right (outer) side
        let legs = [
            (sx + hw * ins, sy), // right corner
            (sx, sy + hh * ins), // front corner
        ];
        for &(lx, ly) in &legs {
            cr.move_to(lx, ly);
            cr.line_to(lx, ly - table_h * z);
            let _ = cr.stroke();
        }
    }

    // Table surface — thin slab filling the tile
    iso_left_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        table_h + slab,
        table_h,
        0.14 * 0.40,
        0.55 * 0.40,
        0.22 * 0.40,
    );
    iso_right_face(
        cr,
        sx,
        sy,
        z,
        wr,
        hr,
        table_h + slab,
        table_h,
        0.14 * 0.65,
        0.55 * 0.65,
        0.22 * 0.65,
    );
    iso_diamond(cr, sx, sy, z, wr, hr, table_h + slab, 0.14, 0.58, 0.24);

    // White border on outer edges only
    let tt = sy - (table_h + slab) * z;
    let bw = hw * 0.94;
    let bh = hh * 0.94;
    cr.set_source_rgb(0.95, 0.95, 0.95);
    cr.set_line_width(1.0 * z);
    if is_left {
        // Border on back, left, front — NOT on the right edge (shared with other half)
        cr.move_to(sx + bw, tt); // right (shared edge start)
        cr.line_to(sx, tt - bh); // back
        cr.line_to(sx - bw, tt); // left
        cr.line_to(sx, tt + bh); // front
        cr.line_to(sx + bw, tt); // back to shared edge
        let _ = cr.stroke();
    } else {
        // Border on back, right, front — NOT on the left edge (shared with other half)
        cr.move_to(sx - bw, tt); // left (shared edge start)
        cr.line_to(sx, tt - bh); // back
        cr.line_to(sx + bw, tt); // right
        cr.line_to(sx, tt + bh); // front
        cr.line_to(sx - bw, tt); // back to shared edge
        let _ = cr.stroke();
    }

    // Net on the shared edge (between left and right halves)
    // The shared edge is on the RIGHT side of Left half, LEFT side of Right half
    let net_h = 5.0 * z;
    if is_left {
        // Net post at right edge (back corner of shared edge)
        cr.set_source_rgba(0.70, 0.70, 0.70, 0.85);
        cr.set_line_width(1.5 * z);
        cr.move_to(sx + bw, tt);
        cr.line_to(sx + bw, tt - net_h);
        let _ = cr.stroke();
        // Net extends toward front
        cr.set_source_rgba(0.80, 0.80, 0.80, 0.5);
        cr.set_line_width(0.6 * z);
        cr.move_to(sx + bw, tt - net_h);
        cr.line_to(sx, tt + bh - net_h);
        let _ = cr.stroke();
    } else {
        // Net post at left edge (front corner of shared edge)
        cr.set_source_rgba(0.70, 0.70, 0.70, 0.85);
        cr.set_line_width(1.5 * z);
        cr.move_to(sx - bw, tt);
        cr.line_to(sx - bw, tt - net_h);
        let _ = cr.stroke();
        // Net extends toward back
        cr.set_source_rgba(0.80, 0.80, 0.80, 0.5);
        cr.set_line_width(0.6 * z);
        cr.move_to(sx - bw, tt - net_h);
        cr.line_to(sx, tt - bh - net_h);
        let _ = cr.stroke();
    }
}

// ─── Kitchen counter ───

fn draw_kitchen_counter(cr: &gtk4::cairo::Context, sx: f64, sy: f64, z: f64, gx: u16, gy: u16) {
    let wr = 0.96;
    let hr = 0.45; // narrow depth like real counter

    // Cabinet body
    iso_solid(cr, sx, sy, z, wr, hr, 24.0, 0.28, 0.28, 0.32);
    // Cabinet doors
    left_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.05, 0.92, 0.04, 0.46, 0.22, 0.22, 0.26,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.05, 0.92, 0.54, 0.96, 0.22, 0.22, 0.26,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.38, 0.46, 0.58, 0.58, 0.64,
    );
    left_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.44, 0.50, 0.86, 0.94, 0.58, 0.58, 0.64,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.05, 0.92, 0.04, 0.46, 0.25, 0.25, 0.30,
    );
    right_face_rect(
        cr, sx, sy, z, wr, hr, 24.0, 0.05, 0.92, 0.54, 0.96, 0.25, 0.25, 0.30,
    );

    // Countertop — same width as cabinet, slightly overhanging depth
    iso_diamond(cr, sx, sy, z, 0.98, 0.50, 25.0, 0.60, 0.56, 0.50);

    // Appliance on counter based on position hash
    let surface = sy - 25.0 * z;
    let variant = ((gx as u32) * 7 + (gy as u32) * 13) % 5;
    match variant {
        0 => {
            // Microwave
            iso_solid(cr, sx, surface, z, 0.42, 0.38, 10.0, 0.86, 0.86, 0.84);
            left_face_rect(
                cr, sx, surface, z, 0.42, 0.38, 10.0, 0.08, 0.85, 0.08, 0.70, 0.10, 0.10, 0.14,
            );
            right_face_rect(
                cr, sx, surface, z, 0.42, 0.38, 10.0, 0.20, 0.70, 0.78, 0.86, 0.68, 0.68, 0.70,
            );
        }
        1 => {
            // Toaster
            iso_solid(cr, sx, surface, z, 0.22, 0.22, 7.0, 0.40, 0.40, 0.42);
        }
        2 => {
            // Blender
            iso_solid(cr, sx, surface, z, 0.12, 0.12, 14.0, 0.75, 0.75, 0.78);
        }
        3 => {
            // Cutting board
            iso_diamond(cr, sx, surface, z, 0.32, 0.22, 1.0, 0.70, 0.52, 0.30);
        }
        _ => {}
    }
}
