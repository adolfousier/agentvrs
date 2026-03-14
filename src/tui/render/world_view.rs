use crate::avatar::agents::agent_sprite;
use crate::avatar::floors::tile_sprite;
use crate::avatar::sprite::{SpriteFrame, TILE_H, TILE_W};
use crate::tui::app::App;
use crate::world::Position;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let grid = app.grid.read().unwrap();
    let registry = app.registry.read().unwrap();
    let buf = frame.buffer_mut();

    // Fill entire area with dark background (covers any leftover cells)
    let dark = Color::Rgb(20, 20, 25);
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let pos = ratatui::layout::Position::new(x, y);
            if let Some(cell) = buf.cell_mut(pos) {
                cell.set_char(' ');
                cell.set_bg(dark);
                cell.set_fg(dark);
            }
        }
    }

    let tiles_x = area.width / TILE_W;
    let tiles_y = area.height / TILE_H;

    let cam = app.camera;
    let start_x = cam.x.saturating_sub(tiles_x / 2);
    let start_y = cam.y.saturating_sub(tiles_y / 2);

    // Pass 1: tiles — render at area origin, no centering
    for gy in start_y..(start_y + tiles_y).min(grid.height) {
        for gx in start_x..(start_x + tiles_x).min(grid.width) {
            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                let sx = area.x + (gx - start_x) * TILE_W;
                let sy = area.y + (gy - start_y) * TILE_H;
                let sprite = tile_sprite(&cell.tile, gx, gy);
                render_sprite(buf, sx, sy, &sprite, area);
            }
        }
    }

    // Pass 2: agents
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx >= start_x && gx < start_x + tiles_x && gy >= start_y && gy < start_y + tiles_y {
            let sx = area.x + (gx - start_x) * TILE_W;
            let sy = area.y + (gy - start_y) * TILE_H;
            let sprite = agent_sprite(
                &agent.state,
                &agent.anim.facing,
                agent.anim.frame,
                agent.color_index,
            );
            render_sprite(buf, sx, sy, &sprite, area);
        }
    }

    // Pass 3: speech bubbles
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let gx = agent.position.x;
            let gy = agent.position.y;
            if gx >= start_x && gx < start_x + tiles_x && gy >= start_y && gy < start_y + tiles_y
            {
                let sx = area.x + (gx - start_x) * TILE_W;
                let sy = area.y + (gy - start_y) * TILE_H;
                render_speech(buf, sx, sy.saturating_sub(1), speech, area);
            }
        }
    }
}

fn render_sprite(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, sprite: &SpriteFrame, clip: Rect) {
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
