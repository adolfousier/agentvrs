use crate::avatar::agent_color;
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(log) = app.message_log.read() else {
        return;
    };
    let Ok(registry) = app.registry.read() else {
        return;
    };

    let messages = log.recent(50);

    let lines: Vec<Line> = if messages.is_empty() {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No messages yet",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    } else {
        messages
            .iter()
            .map(|msg| {
                let from_agent = registry.get(&msg.from);
                let to_agent = registry.get(&msg.to);

                let from_name = from_agent.map(|a| a.name.as_str()).unwrap_or("?");
                let to_name = to_agent.map(|a| a.name.as_str()).unwrap_or("?");

                let from_color = from_agent
                    .map(|a| agent_color(a.color_index))
                    .unwrap_or(Color::Gray);
                let to_color = to_agent
                    .map(|a| agent_color(a.color_index))
                    .unwrap_or(Color::Gray);

                Line::from(vec![
                    Span::styled(
                        format!(" {} ", msg.timestamp.format("%H:%M:%S")),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(
                        from_name,
                        Style::default().fg(from_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" → ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(to_name, Style::default().fg(to_color)),
                    Span::styled("  ", Style::default()),
                    Span::styled(&msg.text, Style::default().fg(Color::Rgb(200, 200, 210))),
                ])
            })
            .collect()
    };

    let title = format!(" Messages ({}) ", messages.len());
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::vertical(1));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
