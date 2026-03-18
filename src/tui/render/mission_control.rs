use crate::agent::AgentId;
use crate::api::observability::{ActivityEntry, TaskRecord};
use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let buf = frame.buffer_mut();

    // Dark background for MC overlay
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(x, y)) {
                cell.set_char(' ');
                cell.set_bg(Color::Rgb(18, 18, 24));
            }
        }
    }

    // Layout: left = agent cards, right = activity + tasks (stacked)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(area);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(cols[1]);

    draw_agent_cards(frame, app, cols[0]);
    draw_activity_feed(frame, app, right_rows[0]);
    draw_task_list(frame, app, right_rows[1]);
}

fn draw_agent_cards(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

    let items: Vec<ListItem> = registry
        .agents()
        .map(|agent| {
            let s_color = state_color(&agent.state);
            let a_color = agent_color(agent.color_index);

            let kind_label = match &agent.kind {
                crate::agent::AgentKind::OpenCrabs { .. } => "A2A",
                crate::agent::AgentKind::External { .. } => "HTTP",
                crate::agent::AgentKind::Local => "Local",
            };

            let kind_color = match &agent.kind {
                crate::agent::AgentKind::OpenCrabs { .. } => Color::Rgb(200, 130, 50),
                crate::agent::AgentKind::External { .. } => Color::Rgb(80, 180, 220),
                crate::agent::AgentKind::Local => Color::Rgb(100, 100, 120),
            };

            let inbox_count = agent.inbox.len();
            let inbox_span = if inbox_count > 0 {
                Span::styled(
                    format!(" [{}msg]", inbox_count),
                    Style::default()
                        .fg(Color::Rgb(255, 100, 100))
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled("  ● ", Style::default().fg(s_color)),
                    Span::styled(
                        &agent.name,
                        Style::default().fg(a_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        format!(" {} ", agent.state.label()),
                        Style::default()
                            .fg(Color::Rgb(20, 20, 30))
                            .bg(s_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    inbox_span,
                ]),
                Line::from(vec![
                    Span::styled("    ", Style::default()),
                    Span::styled(
                        format!("{:.8}", agent.id),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(kind_label, Style::default().fg(kind_color)),
                    Span::styled(
                        format!("  tasks:{}", agent.task_count),
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                ]),
                Line::from(vec![Span::styled(
                    "  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                    Style::default().fg(Color::Rgb(40, 40, 55)),
                )]),
            ];

            ListItem::new(lines)
        })
        .collect();

    let agent_count = registry.agents().count();
    let title = format!(" Agents ({}) ", agent_count);

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 70)))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::vertical(1));

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_activity_feed(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(observer) = app.observer.read() else {
        return;
    };
    let Ok(registry) = app.registry.read() else {
        return;
    };

    // Collect all activities across all agents, sorted by time (most recent first)
    let agent_ids: Vec<AgentId> = registry.agents().map(|a| a.id).collect();
    let mut all_entries: Vec<(AgentId, &ActivityEntry)> = Vec::new();
    for id in &agent_ids {
        for entry in observer.get_activity(id, 20) {
            all_entries.push((*id, entry));
        }
    }
    all_entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
    all_entries.truncate(30);

    let lines: Vec<Line> = if all_entries.is_empty() {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No activity yet",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    } else {
        all_entries
            .iter()
            .map(|(agent_id, entry)| {
                let agent_name = registry
                    .get(agent_id)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| format!("{:.8}", agent_id));

                let a_color = registry
                    .get(agent_id)
                    .map(|a| agent_color(a.color_index))
                    .unwrap_or(Color::Gray);

                Line::from(vec![
                    Span::styled(
                        format!(" {} ", entry.timestamp.format("%H:%M:%S")),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(
                        agent_name,
                        Style::default()
                            .fg(a_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        &entry.detail,
                        Style::default().fg(Color::Rgb(180, 180, 200)),
                    ),
                ])
            })
            .collect()
    };

    let title = format!(" Activity ({}) ", all_entries.len());
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 70)))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::vertical(1));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_task_list(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(observer) = app.observer.read() else {
        return;
    };
    let Ok(registry) = app.registry.read() else {
        return;
    };

    // Collect all tasks across all agents
    let agent_ids: Vec<AgentId> = registry.agents().map(|a| a.id).collect();
    let mut all_tasks: Vec<(AgentId, &TaskRecord)> = Vec::new();
    for id in &agent_ids {
        for task in observer.get_tasks(id, 50) {
            all_tasks.push((*id, task));
        }
    }
    all_tasks.sort_by(|a, b| b.1.last_updated.cmp(&a.1.last_updated));

    let lines: Vec<Line> = if all_tasks.is_empty() {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No tasks",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    } else {
        all_tasks
            .iter()
            .map(|(agent_id, task)| {
                let state_bg = match task.state.as_str() {
                    "submitted" => Color::Yellow,
                    "running" => Color::Rgb(80, 180, 220),
                    "completed" => Color::Green,
                    "failed" => Color::Red,
                    _ => Color::Gray,
                };

                let agent_name = registry
                    .get(agent_id)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| format!("{:.8}", agent_id));

                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        format!(" {:>9} ", task.state),
                        Style::default()
                            .fg(Color::Rgb(20, 20, 30))
                            .bg(state_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        agent_name,
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        task.response_summary.as_deref().unwrap_or("—"),
                        Style::default().fg(Color::Rgb(180, 180, 200)),
                    ),
                ])
            })
            .collect()
    };

    let title = format!(" Tasks ({}) ", all_tasks.len());
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 70)))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::vertical(1));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
