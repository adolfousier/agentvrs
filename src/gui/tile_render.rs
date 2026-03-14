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

fn draw_floor(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &FloorKind, zoom: f64) {
    let (r, g, b) = match kind {
        FloorKind::Wood => (0.65, 0.45, 0.25),
        FloorKind::Tile => (0.80, 0.80, 0.84),
        FloorKind::Carpet => (0.28, 0.28, 0.50),
        FloorKind::Concrete => (0.52, 0.52, 0.54),
    };
    draw_floor_diamond(cr, sx, sy, zoom, r, g, b);
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

fn draw_wall(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &WallKind, zoom: f64) {
    let hw = TILE_W / 2.0 * zoom;
    let hh = TILE_H / 2.0 * zoom;
    let wh = WALL_HEIGHT * zoom;

    let (r, g, b) = match kind {
        WallKind::Solid => (0.50, 0.50, 0.55),
        WallKind::Window => (0.55, 0.65, 0.80),
    };

    // Left face
    cr.move_to(sx - hw, sy - wh);
    cr.line_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.6, g * 0.6, b * 0.6);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.4, g * 0.4, b * 0.4);
    cr.set_line_width(0.5);
    let _ = cr.stroke();

    // Right face
    cr.move_to(sx + hw, sy - wh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.75, g * 0.75, b * 0.75);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.5, g * 0.5, b * 0.5);
    let _ = cr.stroke();

    // Top face
    cr.move_to(sx, sy - hh - wh);
    cr.line_to(sx + hw, sy - wh);
    cr.line_to(sx, sy + hh - wh);
    cr.line_to(sx - hw, sy - wh);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.6, g * 0.6, b * 0.6);
    let _ = cr.stroke();

    // Window pane on right face
    if matches!(kind, WallKind::Window) {
        let inset = 6.0 * zoom;
        let pane_h = wh - inset * 2.0;
        cr.move_to(sx + hw - inset * 0.8, sy - wh + inset + pane_h * 0.1);
        cr.line_to(sx + hw - inset * 0.8, sy - inset * 0.5);
        cr.line_to(sx + inset * 0.5, sy + hh - inset * 0.5);
        cr.line_to(sx + inset * 0.5, sy + hh - wh + inset + pane_h * 0.1);
        cr.close_path();
        cr.set_source_rgba(0.65, 0.88, 1.0, 0.45);
        let _ = cr.fill();
    }
}

fn draw_door(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.55, 0.55, 0.55);
    // Threshold lines
    let hw = TILE_W / 2.0 * zoom * 0.6;
    let hh = TILE_H / 2.0 * zoom * 0.6;
    cr.move_to(sx - hw, sy);
    cr.line_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.close_path();
    cr.set_source_rgb(0.45, 0.45, 0.45);
    cr.set_line_width(1.5 * zoom);
    let _ = cr.stroke();
}

fn draw_rug(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    draw_floor_diamond(cr, sx, sy, zoom, 0.65, 0.18, 0.18);
    // Inner pattern
    let hw = TILE_W / 2.0 * zoom * 0.6;
    let hh = TILE_H / 2.0 * zoom * 0.6;
    cr.move_to(sx, sy - hh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx - hw, sy);
    cr.close_path();
    cr.set_source_rgb(0.75, 0.25, 0.12);
    let _ = cr.fill();
}

// --- Furniture with proper 3D shapes ---

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
    cr.set_source_rgb(r * 0.6, g * 0.6, b * 0.6);
    let _ = cr.fill();

    // Right face
    cr.move_to(sx + hw, sy - bh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - bh);
    cr.close_path();
    cr.set_source_rgb(r * 0.8, g * 0.8, b * 0.8);
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

fn draw_desk(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Table top
    iso_block(cr, sx, sy, zoom, 0.75, 0.75, 8.0, 0.55, 0.35, 0.17);
    // Monitor stand
    let desk_top = sy - 8.0 * zoom;
    cr.rectangle(sx - 1.0 * zoom, desk_top - 4.0 * zoom, 2.0 * zoom, 4.0 * zoom);
    cr.set_source_rgb(0.25, 0.25, 0.3);
    let _ = cr.fill();
    // Monitor body
    let mon_w = 16.0 * zoom;
    let mon_h = 11.0 * zoom;
    let mon_y = desk_top - 4.0 * zoom - mon_h;
    cr.rectangle(sx - mon_w / 2.0, mon_y, mon_w, mon_h);
    cr.set_source_rgb(0.15, 0.15, 0.2);
    let _ = cr.fill();
    // Screen
    let border = 1.5 * zoom;
    cr.rectangle(
        sx - mon_w / 2.0 + border,
        mon_y + border,
        mon_w - border * 2.0,
        mon_h - border * 2.0,
    );
    cr.set_source_rgb(0.2, 0.5, 0.85);
    let _ = cr.fill();
    // Code lines on screen
    cr.set_source_rgba(0.5, 0.9, 0.5, 0.7);
    cr.set_line_width(1.0 * zoom);
    for i in 0..3 {
        let ly = mon_y + border + 2.0 * zoom + (i as f64) * 2.5 * zoom;
        let lw = (mon_w - border * 4.0) * (0.8 - i as f64 * 0.15);
        cr.move_to(sx - mon_w / 2.0 + border * 2.0, ly);
        cr.line_to(sx - mon_w / 2.0 + border * 2.0 + lw, ly);
        let _ = cr.stroke();
    }
}

fn draw_vending(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    iso_block(cr, sx, sy, zoom, 0.55, 0.55, 22.0, 0.75, 0.15, 0.15);
    // Display panel
    let top = sy - 22.0 * zoom;
    cr.rectangle(sx - 5.0 * zoom, top + 2.0 * zoom, 12.0 * zoom, 5.0 * zoom);
    cr.set_source_rgb(1.0, 0.85, 0.3);
    let _ = cr.fill();
    // Product rows
    for i in 0..3 {
        let ry = top + 9.0 * zoom + i as f64 * 3.5 * zoom;
        for j in 0..3 {
            let rx = sx - 5.0 * zoom + j as f64 * 4.0 * zoom;
            cr.rectangle(rx, ry, 3.0 * zoom, 2.5 * zoom);
            cr.set_source_rgb(0.3, 0.6, 0.3);
            let _ = cr.fill();
        }
    }
}

fn draw_coffee(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Counter base
    iso_block(cr, sx, sy, zoom, 0.55, 0.55, 10.0, 0.45, 0.45, 0.48);
    // Machine body on counter
    let top = sy - 10.0 * zoom;
    iso_block(cr, sx, sy.min(top + 2.0 * zoom), zoom, 0.35, 0.35, 12.0, 0.35, 0.22, 0.12);
    // Cup
    cr.rectangle(sx + 4.0 * zoom, top - 3.0 * zoom, 3.0 * zoom, 3.0 * zoom);
    cr.set_source_rgb(0.9, 0.9, 0.9);
    let _ = cr.fill();
    // Steam wisps
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.5);
    cr.set_line_width(1.5 * zoom);
    cr.move_to(sx, top - 14.0 * zoom);
    cr.curve_to(
        sx - 3.0 * zoom, top - 18.0 * zoom,
        sx + 1.0 * zoom, top - 22.0 * zoom,
        sx + 3.0 * zoom, top - 26.0 * zoom,
    );
    let _ = cr.stroke();
}

fn draw_couch(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Seat
    iso_block(cr, sx, sy, zoom, 0.7, 0.7, 5.0, 0.65, 0.12, 0.12);
    // Back rest
    let hw = TILE_W / 2.0 * zoom * 0.7;
    let back_h = 8.0 * zoom;
    cr.move_to(sx - hw, sy - 5.0 * zoom - back_h);
    cr.line_to(sx - hw, sy - 5.0 * zoom);
    cr.line_to(sx - hw + 3.0 * zoom, sy - 5.0 * zoom);
    cr.line_to(sx - hw + 3.0 * zoom, sy - 5.0 * zoom - back_h);
    cr.close_path();
    cr.set_source_rgb(0.55, 0.08, 0.08);
    let _ = cr.fill();
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Pot
    iso_block(cr, sx, sy, zoom, 0.3, 0.3, 6.0, 0.55, 0.32, 0.15);
    // Trunk
    cr.set_source_rgb(0.4, 0.28, 0.12);
    cr.rectangle(sx - 1.5 * zoom, sy - 14.0 * zoom, 3.0 * zoom, 8.0 * zoom);
    let _ = cr.fill();
    // Foliage (layered circles)
    let leaf = (0.13, 0.55, 0.13);
    cr.arc(sx, sy - 20.0 * zoom, 8.0 * zoom, 0.0, TAU);
    cr.set_source_rgb(leaf.0, leaf.1, leaf.2);
    let _ = cr.fill();
    cr.arc(sx - 4.0 * zoom, sy - 17.0 * zoom, 5.0 * zoom, 0.0, TAU);
    cr.set_source_rgb(leaf.0 * 0.8, leaf.1 * 0.8, leaf.2 * 0.8);
    let _ = cr.fill();
    cr.arc(sx + 4.0 * zoom, sy - 18.0 * zoom, 5.0 * zoom, 0.0, TAU);
    cr.set_source_rgb(leaf.0 * 0.9, leaf.1 * 0.9, leaf.2 * 0.9);
    let _ = cr.fill();
}

fn draw_arcade(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    iso_block(cr, sx, sy, zoom, 0.5, 0.5, 20.0, 0.45, 0.12, 0.55);
    let top = sy - 20.0 * zoom;
    // Screen
    cr.rectangle(sx - 5.0 * zoom, top + 2.0 * zoom, 12.0 * zoom, 7.0 * zoom);
    cr.set_source_rgb(0.1, 0.1, 0.15);
    let _ = cr.fill();
    cr.rectangle(sx - 4.0 * zoom, top + 3.0 * zoom, 10.0 * zoom, 5.0 * zoom);
    cr.set_source_rgb(0.15, 0.85, 0.3);
    let _ = cr.fill();
    // Control panel
    cr.rectangle(sx - 4.0 * zoom, top + 11.0 * zoom, 10.0 * zoom, 3.0 * zoom);
    cr.set_source_rgb(0.3, 0.3, 0.35);
    let _ = cr.fill();
    // Buttons
    cr.arc(sx - 1.0 * zoom, top + 12.5 * zoom, 1.0 * zoom, 0.0, TAU);
    cr.set_source_rgb(1.0, 0.2, 0.2);
    let _ = cr.fill();
    cr.arc(sx + 2.0 * zoom, top + 12.5 * zoom, 1.0 * zoom, 0.0, TAU);
    cr.set_source_rgb(0.2, 0.2, 1.0);
    let _ = cr.fill();
}

fn draw_treadmill(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Base/belt
    iso_block(cr, sx, sy, zoom, 0.65, 0.4, 3.0, 0.3, 0.3, 0.32);
    // Handles
    cr.set_source_rgb(0.5, 0.5, 0.55);
    cr.set_line_width(2.0 * zoom);
    cr.move_to(sx - 6.0 * zoom, sy - 3.0 * zoom);
    cr.line_to(sx - 6.0 * zoom, sy - 16.0 * zoom);
    let _ = cr.stroke();
    cr.move_to(sx + 6.0 * zoom, sy - 3.0 * zoom);
    cr.line_to(sx + 6.0 * zoom, sy - 16.0 * zoom);
    let _ = cr.stroke();
    // Top bar
    cr.move_to(sx - 6.0 * zoom, sy - 16.0 * zoom);
    cr.line_to(sx + 6.0 * zoom, sy - 16.0 * zoom);
    let _ = cr.stroke();
    // Display
    cr.rectangle(sx - 3.0 * zoom, sy - 15.0 * zoom, 6.0 * zoom, 3.0 * zoom);
    cr.set_source_rgb(0.2, 0.8, 0.3);
    let _ = cr.fill();
}

fn draw_whiteboard(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    // Stand
    cr.set_source_rgb(0.5, 0.5, 0.55);
    cr.set_line_width(2.0 * zoom);
    cr.move_to(sx - 8.0 * zoom, sy);
    cr.line_to(sx - 8.0 * zoom, sy - 18.0 * zoom);
    let _ = cr.stroke();
    cr.move_to(sx + 8.0 * zoom, sy);
    cr.line_to(sx + 8.0 * zoom, sy - 18.0 * zoom);
    let _ = cr.stroke();
    // Board
    cr.rectangle(sx - 10.0 * zoom, sy - 18.0 * zoom, 20.0 * zoom, 12.0 * zoom);
    cr.set_source_rgb(0.95, 0.95, 0.97);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(0.6, 0.6, 0.65);
    cr.set_line_width(1.5 * zoom);
    let _ = cr.stroke();
    // Scribbles
    cr.set_source_rgb(0.2, 0.4, 0.8);
    cr.set_line_width(1.0 * zoom);
    cr.move_to(sx - 7.0 * zoom, sy - 15.0 * zoom);
    cr.line_to(sx + 5.0 * zoom, sy - 14.0 * zoom);
    let _ = cr.stroke();
    cr.move_to(sx - 6.0 * zoom, sy - 12.0 * zoom);
    cr.line_to(sx + 3.0 * zoom, sy - 11.0 * zoom);
    let _ = cr.stroke();
}
