use crate::gui::iso::{TILE_H, TILE_W, WALL_HEIGHT};
use crate::world::{FloorKind, Tile, WallKind};

pub fn draw_tile(cr: &gtk4::cairo::Context, sx: f64, sy: f64, tile: &Tile, zoom: f64) {
    match tile {
        Tile::Floor(kind) => draw_floor(cr, sx, sy, kind, zoom),
        Tile::Wall(kind) => draw_wall(cr, sx, sy, kind, zoom),
        Tile::DoorOpen => draw_floor_color(cr, sx, sy, zoom, 0.45, 0.45, 0.45),
        Tile::Rug => draw_floor_color(cr, sx, sy, zoom, 0.6, 0.15, 0.15),
        Tile::Desk => {
            draw_floor_color(cr, sx, sy, zoom, 0.55, 0.35, 0.17);
            draw_block(cr, sx, sy, zoom, 10.0, 0.35, 0.22, 0.12);
        }
        Tile::VendingMachine => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 14.0, 0.7, 0.15, 0.15);
        }
        Tile::CoffeeMachine => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 10.0, 0.4, 0.26, 0.13);
        }
        Tile::Couch => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 6.0, 0.6, 0.12, 0.12);
        }
        Tile::Plant => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_plant(cr, sx, sy, zoom);
        }
        Tile::PinballMachine => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 12.0, 0.6, 0.2, 0.6);
        }
        Tile::GymTreadmill => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 8.0, 0.3, 0.3, 0.35);
        }
        Tile::Whiteboard => {
            draw_floor_color(cr, sx, sy, zoom, 0.4, 0.4, 0.42);
            draw_block(cr, sx, sy, zoom, 16.0, 0.85, 0.85, 0.9);
        }
    }
}

fn draw_floor(cr: &gtk4::cairo::Context, sx: f64, sy: f64, kind: &FloorKind, zoom: f64) {
    let (r, g, b) = match kind {
        FloorKind::Wood => (0.65, 0.45, 0.25),
        FloorKind::Tile => (0.78, 0.78, 0.82),
        FloorKind::Carpet => (0.25, 0.25, 0.45),
        FloorKind::Concrete => (0.50, 0.50, 0.52),
    };
    draw_floor_color(cr, sx, sy, zoom, r, g, b);
}

fn draw_floor_color(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64, r: f64, g: f64, b: f64) {
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
        WallKind::Solid => (0.45, 0.45, 0.50),
        WallKind::Window => (0.55, 0.65, 0.80),
    };

    // Top face
    cr.move_to(sx, sy - hh - wh);
    cr.line_to(sx + hw, sy - wh);
    cr.line_to(sx, sy + hh - wh);
    cr.line_to(sx - hw, sy - wh);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.6, g * 0.6, b * 0.6);
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

    // Left face
    cr.move_to(sx - hw, sy - wh);
    cr.line_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - wh);
    cr.close_path();
    cr.set_source_rgb(r * 0.6, g * 0.6, b * 0.6);
    let _ = cr.fill_preserve();
    cr.set_source_rgb(r * 0.4, g * 0.4, b * 0.4);
    let _ = cr.stroke();

    // Window glass
    if matches!(kind, WallKind::Window) {
        let inset = 4.0 * zoom;
        cr.move_to(sx + hw - inset, sy - wh + inset);
        cr.line_to(sx + hw - inset, sy - inset);
        cr.line_to(sx + inset, sy + hh - inset);
        cr.line_to(sx + inset, sy + hh - wh + inset);
        cr.close_path();
        cr.set_source_rgba(0.6, 0.85, 1.0, 0.4);
        let _ = cr.fill();
    }
}

fn draw_block(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    zoom: f64,
    height: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let scale = 0.5;
    let hw = TILE_W / 2.0 * zoom * scale;
    let hh = TILE_H / 2.0 * zoom * scale;
    let bh = height * zoom;

    // Top
    cr.move_to(sx, sy - hh - bh);
    cr.line_to(sx + hw, sy - bh);
    cr.line_to(sx, sy + hh - bh);
    cr.line_to(sx - hw, sy - bh);
    cr.close_path();
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();

    // Right face
    cr.move_to(sx + hw, sy - bh);
    cr.line_to(sx + hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - bh);
    cr.close_path();
    cr.set_source_rgb(r * 0.7, g * 0.7, b * 0.7);
    let _ = cr.fill();

    // Left face
    cr.move_to(sx - hw, sy - bh);
    cr.line_to(sx - hw, sy);
    cr.line_to(sx, sy + hh);
    cr.line_to(sx, sy + hh - bh);
    cr.close_path();
    cr.set_source_rgb(r * 0.55, g * 0.55, b * 0.55);
    let _ = cr.fill();
}

fn draw_plant(cr: &gtk4::cairo::Context, sx: f64, sy: f64, zoom: f64) {
    let bh = 6.0 * zoom;
    // Pot
    draw_block(cr, sx, sy, zoom, bh / zoom, 0.55, 0.35, 0.17);
    // Foliage (circle above pot)
    let radius = 8.0 * zoom;
    cr.arc(sx, sy - bh - radius, radius, 0.0, std::f64::consts::TAU);
    cr.set_source_rgb(0.15, 0.55, 0.15);
    let _ = cr.fill();
}
