use crate::gui::types::Camera;

/// Tile dimensions in screen pixels (2:1 isometric ratio).
pub const TILE_W: f64 = 64.0;
pub const TILE_H: f64 = 32.0;
/// Height extrusion for walls and raised objects.
pub const WALL_HEIGHT: f64 = 20.0;

/// Convert grid coordinates to screen position (center of diamond).
pub fn grid_to_screen(gx: f64, gy: f64, cam: &Camera) -> (f64, f64) {
    let (rx, ry) = apply_rotation(gx, gy, cam.rotation);
    let sx = (rx - ry) * (TILE_W / 2.0) * cam.zoom + cam.offset_x;
    let sy = (rx + ry) * (TILE_H / 2.0) * cam.zoom + cam.offset_y;
    (sx, sy)
}

/// Convert screen position back to fractional grid coordinates.
pub fn screen_to_grid(sx: f64, sy: f64, cam: &Camera) -> (f64, f64) {
    let lx = (sx - cam.offset_x) / cam.zoom;
    let ly = (sy - cam.offset_y) / cam.zoom;

    let rx = ly / TILE_H + lx / TILE_W;
    let ry = ly / TILE_H - lx / TILE_W;

    unapply_rotation(rx, ry, cam.rotation)
}

/// Apply rotation (0-3) to grid coordinates.
fn apply_rotation(gx: f64, gy: f64, rotation: u8) -> (f64, f64) {
    match rotation % 4 {
        0 => (gx, gy),
        1 => (gy, -gx),
        2 => (-gx, -gy),
        3 => (-gy, gx),
        _ => unreachable!(),
    }
}

/// Reverse rotation to get back to grid coordinates.
fn unapply_rotation(rx: f64, ry: f64, rotation: u8) -> (f64, f64) {
    match rotation % 4 {
        0 => (rx, ry),
        1 => (-ry, rx),
        2 => (-rx, -ry),
        3 => (ry, -rx),
        _ => unreachable!(),
    }
}

