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

    // Pass 3: agent name labels (below sprite)
    for agent in registry.agents() {
        let gx = agent.position.x;
        let gy = agent.position.y;
        if gx < tiles_x && gy < tiles_y {
            let sx = ox + gx * TILE_W;
            let sy = oy + gy * TILE_H;
            let label_y = sy + TILE_H + 1; // one row below the sprite bottom
            let name = if agent.name.len() > 10 {
                &agent.name[..10]
            } else {
                &agent.name
            };
            let name_len = name.len() as u16;
            // Center the label on the tile
            let label_x = sx + TILE_W / 2 - name_len / 2;
            let color = agent_color(agent.color_index);
            render_label(buf, label_x, label_y, name, color, area);
        }
    }

    // Pass 4: speech bubbles
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let gx = agent.position.x;
            let gy = agent.position.y;
            if gx < tiles_x && gy < tiles_y {
                let sx = ox + gx * TILE_W;
                let sy = oy + gy * TILE_H;
                render_speech(buf, sx, sy.saturating_sub(4), speech, area);
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
