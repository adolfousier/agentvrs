use crate::tui::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };
    let agent_count = registry.count();

    let selected_name = if app.selected_index < registry.agents().count() {
        registry
            .agents()
            .nth(app.selected_index)
            .map(|a| a.name.clone())
    } else {
        None
    };
    drop(registry);

    let mode_label = match app.mode {
        AppMode::WorldView => "WORLD",
        AppMode::AgentDetail => "DETAIL",
        AppMode::MessageLog => "LOG",
        AppMode::CommandInput => "CMD",
        AppMode::MissionControl => "MC",
    };

    let mode_bg = match app.mode {
        AppMode::WorldView => Color::Rgb(40, 160, 200),
        AppMode::AgentDetail => Color::Rgb(80, 180, 80),
        AppMode::MessageLog => Color::Rgb(200, 160, 40),
        AppMode::CommandInput => Color::Rgb(200, 80, 80),
        AppMode::MissionControl => Color::Rgb(180, 80, 200),
    };

    let mut spans = vec![
        Span::styled(
            format!(" {} ", mode_label),
            Style::default()
                .fg(Color::Rgb(20, 20, 30))
                .bg(mode_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled("●", Style::default().fg(Color::Green)),
        Span::styled(
            format!(" {} agents ", agent_count),
            Style::default().fg(Color::Rgb(180, 180, 190)),
        ),
        Span::styled("│", Style::default().fg(Color::Rgb(60, 60, 80))),
        Span::styled(
            format!(" tick {} ", app.tick_count),
            Style::default().fg(Color::Rgb(120, 120, 140)),
        ),
    ];

    if let Some(name) = selected_name {
        spans.push(Span::styled(
            "│",
            Style::default().fg(Color::Rgb(60, 60, 80)),
        ));
        spans.push(Span::styled(
            format!(" {} ", name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    spans.push(Span::styled(
        "│",
        Style::default().fg(Color::Rgb(60, 60, 80)),
    ));

    if app.mode == AppMode::CommandInput {
        spans.push(Span::styled(
            format!(" :{}", app.command_input),
            Style::default().fg(Color::Yellow),
        ));
    } else if let Some(ref msg) = app.status_message {
        spans.push(Span::styled(
            format!(" {} ", msg),
            Style::default().fg(Color::Yellow),
        ));
    } else {
        spans.push(Span::styled(
            " q:quit  j/k:select  ↵:detail  ⇥:log  h:sidebar  m:MC  ::cmd",
            Style::default().fg(Color::Rgb(80, 80, 100)),
        ));
    }

    let line = Line::from(spans);
    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)));
    let paragraph = Paragraph::new(line).block(block);
    frame.render_widget(paragraph, area);
}
