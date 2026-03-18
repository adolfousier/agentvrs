use crate::avatar::agents::agent_sprite;
use crate::avatar::floors::tile_sprite;
use crate::avatar::palette::agent_color;
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
                cell.set_bg(Color::Rgb(20, 20, 28));
                cell.set_fg(Color::Rgb(20, 20, 28));
            }
        }
    }

    // Calculate scale factor to fill the screen as much as possible
    let world_w = grid.width * TILE_W;
    let world_h = grid.height * TILE_H;
    let scale_x = area.width / world_w.max(1);
    let scale_y = area.height / world_h.max(1);
    let scale = scale_x.min(scale_y).max(1);

    let scaled_w = world_w * scale;
    let scaled_h = world_h * scale;
    let tw = TILE_W * scale; // scaled tile width
    let th = TILE_H * scale; // scaled tile height

    // Center the world in the available area
    let ox = area.x + (area.width.saturating_sub(scaled_w)) / 2;
    let oy = area.y + (area.height.saturating_sub(scaled_h)) / 2;

    // Pass 1: tiles (scaled)
    for gy in 0..grid.height {
        for gx in 0..grid.width {
            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                let sx = ox + gx * tw;
                let sy = oy + gy * th;
                let sprite = tile_sprite(&cell.tile, gx, gy);
                render_sprite_scaled(buf, sx, sy, &sprite, scale, area);
            }
        }
    }

    // Pass 2: agents (scaled, centered on their grid position)
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < grid.width && gy < grid.height {
            let sx = ox + gx * tw;
            let sy = oy + gy * th;
            // Center the 8x6 sprite within the scaled tile
            let ax = sx + (tw / 2).saturating_sub(4);
            let ay = sy + (th / 2).saturating_sub(3);
            let sprite = agent_sprite(
                &agent.state,
                &agent.anim.facing,
                agent.anim.frame,
                agent.color_index,
            );
            render_big_sprite(buf, ax, ay, &sprite, area);
        }
    }

    // Pass 3: agent name labels (below sprite, centered in tile)
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < grid.width && gy < grid.height {
            let sx = ox + gx * tw;
            let sy = oy + gy * th;
            let label_y = sy + (th / 2) + 4;
            let name: String = agent.name.chars().take(10).collect();
            let name_len = name.len() as u16;
            let label_x = (sx + tw / 2).saturating_sub(name_len / 2);
            let color = agent_color(agent.color_index);
            render_label(buf, label_x, label_y, &name, color, area);
        }
    }

    // Pass 4: speech bubbles (above agent)
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let gx = agent.position.x;
            let gy = agent.position.y;
            if gx < grid.width && gy < grid.height {
                let sx = ox + gx * tw;
                let sy = oy + gy * th;
                let bubble_y = (sy + th / 2).saturating_sub(6);
                render_speech(buf, sx, bubble_y, speech, area);
            }
        }
    }
}

fn render_sprite_scaled(
    buf: &mut ratatui::buffer::Buffer,
    x: u16,
    y: u16,
    sprite: &SpriteFrame,
    scale: u16,
    clip: Rect,
) {
    for (row_idx, row) in sprite.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            // Repeat each cell scale×scale times
            for sy in 0..scale {
                for sx in 0..scale {
                    let bx = x + col_idx as u16 * scale + sx;
                    let by = y + row_idx as u16 * scale + sy;
                    if bx >= clip.x
                        && bx < clip.x + clip.width
                        && by >= clip.y
                        && by < clip.y + clip.height
                    {
                        let pos = ratatui::layout::Position::new(bx, by);
                        if let Some(buf_cell) = buf.cell_mut(pos) {
                            if let Some(bg) = cell.bg {
                                buf_cell.set_bg(bg);
                            }
                            if cell.ch != ' ' {
                                buf_cell.set_char(cell.ch);
                                buf_cell.set_fg(cell.fg);
                            } else if let Some(bg) = cell.bg {
                                buf_cell.set_char(' ');
                                buf_cell.set_bg(bg);
                            }
                        }
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
    let content = if text.chars().count() > 18 {
        let truncated: String = text.chars().take(15).collect();
        format!("{}...", truncated)
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
