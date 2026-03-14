/// Tile dimensions in screen pixels (2:1 isometric ratio).
pub const TILE_W: f64 = 64.0;
pub const TILE_H: f64 = 32.0;
/// Height extrusion for walls.
pub const WALL_HEIGHT: f64 = 20.0;

/// Convert grid coordinates to screen position (center of diamond).
/// Rotates around the world center so rotation doesn't shift the view.
pub fn grid_to_screen(
    gx: f64,
    gy: f64,
    zoom: f64,
    rotation: u8,
    world_cx: f64,
    world_cy: f64,
) -> (f64, f64) {
    let cx = gx - world_cx;
    let cy = gy - world_cy;
    let (rx, ry) = apply_rotation(cx, cy, rotation);
    let sx = (rx - ry) * (TILE_W / 2.0) * zoom;
    let sy = (rx + ry) * (TILE_H / 2.0) * zoom;
    (sx, sy)
}

/// Convert screen position back to fractional grid coordinates.
pub fn screen_to_grid(
    sx: f64,
    sy: f64,
    zoom: f64,
    rotation: u8,
    world_cx: f64,
    world_cy: f64,
) -> (f64, f64) {
    let lx = sx / zoom;
    let ly = sy / zoom;
    let rx = ly / TILE_H + lx / TILE_W;
    let ry = ly / TILE_H - lx / TILE_W;
    let (gx, gy) = unapply_rotation(rx, ry, rotation);
    (gx + world_cx, gy + world_cy)
}

/// Return iteration order for painter's algorithm based on rotation.
/// Returns (y_range, x_range) as (start, end, step) for correct back-to-front.
pub fn draw_order(w: u16, h: u16, rotation: u8) -> Vec<(u16, u16)> {
    let mut coords = Vec::with_capacity(w as usize * h as usize);
    match rotation % 4 {
        0 => {
            for gy in 0..h {
                for gx in 0..w {
                    coords.push((gx, gy));
                }
            }
        }
        1 => {
            for gx in (0..w).rev() {
                for gy in 0..h {
                    coords.push((gx, gy));
                }
            }
        }
        2 => {
            for gy in (0..h).rev() {
                for gx in (0..w).rev() {
                    coords.push((gx, gy));
                }
            }
        }
        3 => {
            for gx in 0..w {
                for gy in (0..h).rev() {
                    coords.push((gx, gy));
                }
            }
        }
        _ => unreachable!(),
    }
    coords
}

fn apply_rotation(gx: f64, gy: f64, rotation: u8) -> (f64, f64) {
    match rotation % 4 {
        0 => (gx, gy),
        1 => (gy, -gx),
        2 => (-gx, -gy),
        3 => (-gy, gx),
        _ => unreachable!(),
    }
}

fn unapply_rotation(rx: f64, ry: f64, rotation: u8) -> (f64, f64) {
    match rotation % 4 {
        0 => (rx, ry),
        1 => (-ry, rx),
        2 => (-rx, -ry),
        3 => (ry, -rx),
        _ => unreachable!(),
    }
}
