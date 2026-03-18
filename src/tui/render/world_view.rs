use crate::avatar::agents::agent_sprite;
use crate::avatar::floors::tile_sprite;
use crate::avatar::palette::agent_color;
use crate::avatar::sprite::{BigSpriteFrame, SpriteFrame, StyledCell, TILE_H, TILE_W};
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

    // Compute scale factors so the world fills the available area
    // Base tile size is 4x3 chars, scale up to fill
    let base_w = grid.width * TILE_W;
    let base_h = grid.height * TILE_H;

    // Scale factor: how many times to repeat each cell (minimum 1)
    let sx = (area.width / base_w).max(1);
    let sy = (area.height / base_h).max(1);

    // Scaled tile dimensions
    let tw = TILE_W * sx;
    let th = TILE_H * sy;

    // Scaled world size
    let world_w = grid.width * tw;
    let world_h = grid.height * th;

    // Center in area
    let ox = area.x + (area.width.saturating_sub(world_w)) / 2;
    let oy = area.y + (area.height.saturating_sub(world_h)) / 2;

    // Pass 1: tiles
    for gy in 0..grid.height {
        for gx in 0..grid.width {
            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                let screen_x = ox + gx * tw;
                let screen_y = oy + gy * th;
                let sprite = tile_sprite(&cell.tile, gx, gy);
                render_sprite_scaled(buf, screen_x, screen_y, &sprite, sx, sy, area);
            }
        }
    }

    // Agent sprite scale: 8x6 base, scale by same factors
    let agent_w = 8 * sx;
    let agent_h = 6 * sy;

    // Pass 2: agents
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < grid.width && gy < grid.height {
            let screen_x = ox + gx * tw;
            let screen_y = oy + gy * th;
            // Center the 8*sx wide agent on the tw-wide tile
            let ax = (screen_x + tw / 2).saturating_sub(agent_w / 2);
            let ay = (screen_y + th / 2).saturating_sub(agent_h / 2);
            let sprite = agent_sprite(
                &agent.state,
                &agent.anim.facing,
                agent.anim.frame,
                agent.color_index,
            );
            render_big_sprite_scaled(buf, ax, ay, &sprite, sx, sy, area);
        }
    }

    // Pass 3: agent name labels (below sprite)
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < grid.width && gy < grid.height {
            let screen_x = ox + gx * tw;
            let screen_y = oy + gy * th;
            let label_y = screen_y + th + 1;
            let name = if agent.name.len() > 12 {
                &agent.name[..12]
            } else {
                &agent.name
            };
            let name_len = name.len() as u16;
            let label_x = (screen_x + tw / 2).saturating_sub(name_len / 2);
            let color = agent_color(agent.color_index);
            render_label(buf, label_x, label_y, name, color, area);
        }
    }

    // Pass 4: speech bubbles
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let gx = agent.position.x;
            let gy = agent.position.y;
            if gx < grid.width && gy < grid.height {
                let screen_x = ox + gx * tw;
                let screen_y = oy + gy * th;
                render_speech(buf, screen_x, screen_y.saturating_sub(4), speech, area);
            }
        }
    }
}

/// Render a 4x3 tile sprite scaled by (sx, sy) — each cell becomes sx wide, sy tall.
fn render_sprite_scaled(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    sprite: &SpriteFrame,
    sx: u16,
    sy: u16,
    clip: Rect,
) {
    for (row_idx, row) in sprite.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            paint_scaled_cell(buf, x + col_idx as u16 * sx, y + row_idx as u16 * sy, cell, sx, sy, clip);
        }
    }
}

/// Render an 8x6 agent sprite scaled by (sx, sy).
fn render_big_sprite_scaled(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    sprite: &BigSpriteFrame,
    sx: u16,
    sy: u16,
    clip: Rect,
) {
    for (row_idx, row) in sprite.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            paint_scaled_cell(buf, x + col_idx as u16 * sx, y + row_idx as u16 * sy, cell, sx, sy, clip);
        }
    }
}

/// Paint a single sprite cell scaled to sx*sy terminal cells.
fn paint_scaled_cell(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    cell: &StyledCell,
    sx: u16,
    sy: u16,
    clip: Rect,
) {
    for dy in 0..sy {
        for dx in 0..sx {
            let bx = x + dx;
            let by = y + dy;
            if bx >= clip.x && bx < clip.x + clip.width && by >= clip.y && by < clip.y + clip.height {
                let pos = ratatui::layout::Position::new(bx, by);
                if let Some(buf_cell) = buf.cell_mut(pos) {
                    if let Some(bg) = cell.bg {
                        buf_cell.set_bg(bg);
                    }
                    if cell.ch != ' ' {
                        // Only draw the character in the first column of the scale group
                        // to avoid overlapping characters; fill rest with bg
                        if dx == 0 && dy == 0 {
                            buf_cell.set_char(cell.ch);
                            buf_cell.set_fg(cell.fg);
                        } else if let Some(bg) = cell.bg {
                            buf_cell.set_char(' ');
                            buf_cell.set_bg(bg);
                        } else {
                            buf_cell.set_char(cell.ch);
                            buf_cell.set_fg(cell.fg);
                        }
                    }
                }
            }
        }
    }
}

fn render_label(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    text: &str,
    color: Color,
    clip: Rect,
) {
    for (i, ch) in text.chars().enumerate() {
        let bx = x + i as u16;
        if bx >= clip.x && bx < clip.x + clip.width && y >= clip.y && y < clip.y + clip.height {
            let pos = ratatui::layout::Position::new(bx, y);
            if let Some(buf_cell) = buf.cell_mut(pos) {
                buf_cell.set_char(ch);
                buf_cell.set_fg(color);
            }
        }
    }
}

fn render_speech(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, text: &str, clip: Rect) {
    let content = if text.len() > 18 {
        format!("{}...", &text[..15])
    } else {
        text.to_string()
    };
    let inner = format!(" {} ", content);
    let width = inner.len();
    let bubble_fg = Color::Rgb(60, 60, 70);
    let bubble_bg = Color::Rgb(245, 245, 250);
    let text_fg = Color::Rgb(30, 30, 40);

    // Row 0: top border  ╭───╮
    let top_y = y.saturating_sub(1);
    render_text_at(buf, x, top_y, "╭", bubble_fg, None, clip);
    for i in 0..width {
        render_text_at(buf, x + 1 + i as u16, top_y, "─", bubble_fg, None, clip);
    }
    render_text_at(buf, x + 1 + width as u16, top_y, "╮", bubble_fg, None, clip);

    // Row 1: content  │ text │
    render_text_at(buf, x, y, "│", bubble_fg, None, clip);
    for (i, ch) in inner.chars().enumerate() {
        let bx = x + 1 + i as u16;
        if bx >= clip.x && bx < clip.x + clip.width && y >= clip.y && y < clip.y + clip.height {
            let pos = ratatui::layout::Position::new(bx, y);
            if let Some(buf_cell) = buf.cell_mut(pos) {
                buf_cell.set_char(ch);
                buf_cell.set_fg(text_fg);
                buf_cell.set_bg(bubble_bg);
            }
        }
    }
    render_text_at(buf, x + 1 + width as u16, y, "│", bubble_fg, None, clip);

    // Row 2: bottom border with tail  ╰─▽─╯
    let bot_y = y + 1;
    render_text_at(buf, x, bot_y, "╰", bubble_fg, None, clip);
    let tail_pos = width / 2;
    for i in 0..width {
        if i == tail_pos {
            render_text_at(buf, x + 1 + i as u16, bot_y, "▽", bubble_fg, None, clip);
        } else {
            render_text_at(buf, x + 1 + i as u16, bot_y, "─", bubble_fg, None, clip);
        }
    }
    render_text_at(buf, x + 1 + width as u16, bot_y, "╯", bubble_fg, None, clip);
}

fn render_text_at(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    text: &str,
    fg: Color,
    bg: Option<Color>,
    clip: Rect,
) {
    if x >= clip.x && x < clip.x + clip.width && y >= clip.y && y < clip.y + clip.height {
        let pos = ratatui::layout::Position::new(x, y);
        if let Some(buf_cell) = buf.cell_mut(pos) {
            for ch in text.chars() {
                buf_cell.set_char(ch);
                buf_cell.set_fg(fg);
                if let Some(bg) = bg {
                    buf_cell.set_bg(bg);
                }
            }
        }
    }
}
