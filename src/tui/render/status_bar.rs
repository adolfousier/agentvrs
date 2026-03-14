use crate::tui::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let agent_count = {
        let registry = app.registry.read().unwrap();
        registry.count()
    };

    let mode_label = match app.mode {
        AppMode::WorldView => "WORLD",
        AppMode::AgentDetail => "DETAIL",
        AppMode::MessageLog => "LOG",
        AppMode::CommandInput => "CMD",
    };

    let mut spans = vec![
        Span::styled(
            format!(" {} ", mode_label),
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::styled(
            format!(" {}agt ", agent_count),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            format!("t:{} ", app.tick_count),
            Style::default().fg(Color::DarkGray),
        ),
    ];

    if app.mode == AppMode::CommandInput {
        spans.push(Span::styled(
            format!(":{}", app.command_input),
            Style::default().fg(Color::Yellow),
        ));
    } else if let Some(ref msg) = app.status_message {
        spans.push(Span::styled(
            msg.clone(),
            Style::default().fg(Color::Yellow),
        ));
    } else {
        spans.push(Span::styled(
            "q:quit hjkl:pan n/p:sel c:center f:fit tab:log :cmd",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Rgb(30, 30, 35)));
    frame.render_widget(paragraph, area);
}
