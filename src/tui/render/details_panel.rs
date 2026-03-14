use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let registry = app.registry.read().unwrap();

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

        vec![
            Line::from(vec![Span::styled(
                &agent.name,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )]),
            Line::raw(""),
            Line::from(vec![
                Span::raw("  ID:     "),
                Span::styled(agent.id.to_string(), Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::raw("  State:  "),
                Span::styled(agent.state.label(), Style::default().fg(s_color)),
            ]),
            Line::from(vec![Span::raw("  Kind:   "), Span::raw(kind_label)]),
            Line::from(vec![
                Span::raw("  Pos:    "),
                Span::raw(format!("({}, {})", agent.position.x, agent.position.y)),
            ]),
            Line::from(vec![
                Span::raw("  Tasks:  "),
                Span::raw(agent.task_count.to_string()),
            ]),
            Line::raw(""),
            if let Some(ref speech) = agent.speech {
                Line::from(vec![
                    Span::raw("  Says: "),
                    Span::styled(
                        format!("\"{}\"", speech),
                        Style::default().fg(Color::Yellow),
                    ),
                ])
            } else {
                Line::raw("  (silent)")
            },
        ]
    } else {
        vec![Line::raw("  No agent selected. Use j/k and Enter.")]
    };

    let block = Block::default()
        .title(" agent detail ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
