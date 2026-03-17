use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(log) = app.message_log.read() else { return };
    let Ok(registry) = app.registry.read() else { return };

    let lines: Vec<Line> = log
        .recent(50)
        .iter()
        .map(|msg| {
            let from_name = registry
                .get(&msg.from)
                .map(|a| a.name.as_str())
                .unwrap_or("?");
            let to_name = registry
                .get(&msg.to)
                .map(|a| a.name.as_str())
                .unwrap_or("?");

            Line::raw(format!(
                "[{}] {} -> {}: {}",
                msg.timestamp.format("%H:%M:%S"),
                from_name,
                to_name,
                msg.text
            ))
        })
        .collect();

    let block = Block::default()
        .title(" message log ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
