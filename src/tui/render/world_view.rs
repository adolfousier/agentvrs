use crate::avatar::agents::agent_sprite;
use crate::avatar::floors::tile_sprite;
use crate::avatar::sprite::{BigSpriteFrame, SpriteFrame, TILE_H, TILE_W};
use crate::tui::app::App;
use crate::world::Position;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(grid) = app.grid.read() else { return };
    let Ok(registry) = app.registry.read() else {
        return;
    };
    let buf = frame.buffer_mut();

    // Fill entire area with dark bg
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(x, y)) {
                cell.set_char(' ');
                cell.set_bg(Color::Rgb(25, 25, 30));
                cell.set_fg(Color::Rgb(25, 25, 30));
            }
        }
    }

    // Show the entire world centered in the terminal area
    let tiles_x = (area.width / TILE_W).min(grid.width);
    let tiles_y = (area.height / TILE_H).min(grid.height);

    // Center the grid in the available area
    let grid_pixel_w = tiles_x * TILE_W;
    let grid_pixel_h = tiles_y * TILE_H;
    let ox = area.x + (area.width.saturating_sub(grid_pixel_w)) / 2;
    let oy = area.y + (area.height.saturating_sub(grid_pixel_h)) / 2;

    // Pass 1: tiles
    for gy in 0..tiles_y {
        for gx in 0..tiles_x {
            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                let sx = ox + gx * TILE_W;
                let sy = oy + gy * TILE_H;
                let sprite = tile_sprite(&cell.tile, gx, gy);
                render_sprite(buf, sx, sy, &sprite, area);
            }
        }
    }

    // Pass 2: agents (rendered at 8x6, centered on their grid position)
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < tiles_x && gy < tiles_y {
            let sx = ox + gx * TILE_W;
            let sy = oy + gy * TILE_H;
            let ax = sx.saturating_sub(2);
            let ay = sy.saturating_sub(2);
            let sprite = agent_sprite(
                &agent.state,
                &agent.anim.facing,
                agent.anim.frame,
                agent.color_index,
            );
            render_big_sprite(buf, ax, ay, &sprite, area);
        }
    }

    // Pass 3: speech bubbles
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let gx = agent.position.x;
            let gy = agent.position.y;
            if gx < tiles_x && gy < tiles_y {
                let sx = ox + gx * TILE_W;
                let sy = oy + gy * TILE_H;
                render_speech(buf, sx, sy.saturating_sub(3), speech, area);
            }
        }
    }
}

fn render_sprite(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    sprite: &SpriteFrame,
    clip: Rect,
) {
    for (row_idx, row) in sprite.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let bx = x + col_idx as u16;
            let by = y + row_idx as u16;
            if bx >= clip.x && bx < clip.x + clip.width && by >= clip.y && by < clip.y + clip.height
            {
                let pos = ratatui::layout::Position::new(bx, by);
                if let Some(buf_cell) = buf.cell_mut(pos) {
                    if let Some(bg) = cell.bg {
                        buf_cell.set_bg(bg);
                    }
                    if cell.ch != ' ' {
                        buf_cell.set_char(cell.ch);
                        buf_cell.set_fg(cell.fg);
                    }
                }
            }
        }
    }
}

fn render_big_sprite(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    sprite: &BigSpriteFrame,
    clip: Rect,
) {
    for (row_idx, row) in sprite.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let bx = x + col_idx as u16;
            let by = y + row_idx as u16;
            if bx >= clip.x && bx < clip.x + clip.width && by >= clip.y && by < clip.y + clip.height
            {
                let pos = ratatui::layout::Position::new(bx, by);
                if let Some(buf_cell) = buf.cell_mut(pos) {
                    if let Some(bg) = cell.bg {
                        buf_cell.set_bg(bg);
                    }
                    if cell.ch != ' ' {
                        buf_cell.set_char(cell.ch);
                        buf_cell.set_fg(cell.fg);
                    }
                }
            }
        }
    }
}

fn render_speech(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, text: &str, clip: Rect) {
    let display = if text.len() > 16 {
        format!(" {}... ", &text[..13])
    } else {
        format!(" {} ", text)
    };

    for (i, ch) in display.chars().enumerate() {
        let bx = x + i as u16;
        if bx >= clip.x && bx < clip.x + clip.width && y >= clip.y && y < clip.y + clip.height {
            let pos = ratatui::layout::Position::new(bx, y);
            if let Some(buf_cell) = buf.cell_mut(pos) {
                buf_cell.set_char(ch);
                buf_cell.set_fg(Color::Black);
                buf_cell.set_bg(Color::White);
            }
        }
    }
}
