use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

    let agents: Vec<_> = registry.agents().collect();
    let agent = if app.selected_index < agents.len() {
        Some(agents[app.selected_index])
    } else {
        None
    };

    let lines = if let Some(agent) = agent {
        let color = agent_color(agent.color_index);
        let s_color = state_color(&agent.state);

        let kind_label = match &agent.kind {
            crate::agent::AgentKind::OpenCrabs { endpoint } => {
                format!("A2A ({})", endpoint)
            }
            crate::agent::AgentKind::External { endpoint } => {
                format!("HTTP ({})", endpoint)
            }
            crate::agent::AgentKind::Local => "Local".to_string(),
        };

        let kind_color = match &agent.kind {
            crate::agent::AgentKind::OpenCrabs { .. } => Color::Rgb(200, 130, 50),
            crate::agent::AgentKind::External { .. } => Color::Rgb(80, 180, 220),
            crate::agent::AgentKind::Local => Color::Rgb(120, 120, 140),
        };

        let mut lines = vec![
            // Agent name header
            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(
                    &agent.name,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(
                    format!(" {} ", agent.state.label()),
                    Style::default()
                        .fg(Color::Rgb(20, 20, 30))
                        .bg(s_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌", Style::default().fg(Color::Rgb(60, 60, 80))),
            ]),
            Line::raw(""),
            // Info section
            Line::from(vec![
                Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                Span::styled("ID      ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled(
                    format!("{:.8}", agent.id),
                    Style::default().fg(Color::Rgb(180, 180, 190)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                Span::styled("Kind    ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled(kind_label, Style::default().fg(kind_color)),
            ]),
            Line::from(vec![
                Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                Span::styled("Pos     ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled(
                    format!("({}, {})", agent.position.x, agent.position.y),
                    Style::default().fg(Color::Rgb(180, 180, 190)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                Span::styled("Tasks   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled(
                    agent.task_count.to_string(),
                    Style::default().fg(if agent.task_count > 0 {
                        Color::Cyan
                    } else {
                        Color::Rgb(180, 180, 190)
                    }),
                ),
            ]),
            Line::raw(""),
        ];

        // Speech section
        if let Some(ref speech) = agent.speech {
            lines.push(Line::from(vec![
                Span::styled("  💬 ", Style::default()),
                Span::styled(
                    format!("\"{}\"", speech),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
            lines.push(Line::raw(""));
        }

        // Inbox section
        if agent.inbox.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                Span::styled("Inbox   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled("empty", Style::default().fg(Color::Rgb(80, 80, 100))),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  Inbox ({})", agent.inbox.len()),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌", Style::default().fg(Color::Rgb(60, 60, 80))),
            ]));
            for msg in agent.inbox.iter().rev().take(10) {
                let sender_name = registry
                    .get(&msg.from)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| format!("{:.8}", msg.from));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled(
                        format!("{} ", sender_name),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(&msg.text, Style::default().fg(Color::Rgb(200, 200, 210))),
                ]));
            }
        }

        lines
    } else {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No agent selected",
                Style::default().fg(Color::Rgb(100, 100, 120)),
            )]),
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  Use j/k to navigate, Enter to view",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    };

    let block = Block::default()
        .title(" Agent Detail ")
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
