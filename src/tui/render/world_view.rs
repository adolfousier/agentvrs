use crate::avatar::{agent_color, sprite::agent_sprite, sprite::tile_sprite};
use crate::tui::app::App;
use crate::world::Tile;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let grid = app.grid.read().unwrap();
    let registry = app.registry.read().unwrap();

    let mut lines: Vec<Line> = Vec::new();

    for y in 0..grid.height {
        let mut spans: Vec<Span> = Vec::new();

        for x in 0..grid.width {
            let pos = crate::world::Position::new(x, y);
            if let Some(cell) = grid.get(pos) {
                if let Some(ref agent_id) = cell.occupant {
                    if let Some(agent) = registry.get(agent_id) {
                        let sprite = agent_sprite(&agent.state);
                        let color = agent_color(agent.color_index);
                        spans.push(Span::styled(sprite, Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw(tile_sprite(false)));
                    }
                } else {
                    let is_wall = matches!(cell.tile, Tile::Wall);
                    let sprite = tile_sprite(is_wall);
                    let style = if is_wall {
                        Style::default().fg(Color::DarkGray)
                    } else {
                        Style::default()
                    };
                    spans.push(Span::styled(sprite, style));
                }
            }
        }

        lines.push(Line::from(spans));
    }

    // Overlay speech bubbles
    for agent in registry.agents() {
        if let Some(ref speech) = agent.speech {
            let bubble_y = agent.position.y.saturating_sub(1) as usize;
            let bubble_x = agent.position.x as usize * 2;
            if bubble_y < lines.len() {
                let text = format!(" {} ", truncate_speech(speech, 20));
                let bubble = Span::styled(text, Style::default().fg(Color::Black).bg(Color::White));
                if lines.len() > bubble_y {
                    let _ = bubble_x;
                    lines[bubble_y] =
                        Line::from(vec![Span::raw(" ".repeat(bubble_x.min(60))), bubble]);
                }
            }
        }
    }

    let block = Block::default()
        .title(" agentverse ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn truncate_speech(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
