use crate::agent::AgentId;
use crate::api::observability::{ActivityEntry, TaskRecord};
use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};

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

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[1]);

    draw_agent_cards(frame, app, cols[0]);
    draw_activity_feed(frame, app, right_rows[0]);
    draw_task_list(frame, app, right_rows[1]);
}

fn draw_agent_cards(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

    // Collect agent info while registry is locked
    struct AgentInfo {
        id: AgentId,
        name: String,
        state_label: String,
        state_color: Color,
        agent_color: Color,
        kind_label: &'static str,
        kind_color: Color,
        position: (u16, u16),
        inbox_count: usize,
    }

    let agents: Vec<AgentInfo> = registry
        .agents()
        .map(|agent| {
            let kind_label = match &agent.kind {
                crate::agent::AgentKind::OpenCrabs { .. } => "opencrabs",
                crate::agent::AgentKind::External { .. } => "external",
                crate::agent::AgentKind::Local => "local",
            };
            let kind_color = match &agent.kind {
                crate::agent::AgentKind::OpenCrabs { .. } => Color::Rgb(200, 130, 50),
                crate::agent::AgentKind::External { .. } => Color::Rgb(80, 180, 220),
                crate::agent::AgentKind::Local => Color::Rgb(100, 100, 120),
            };
            AgentInfo {
                id: agent.id,
                name: agent.name.clone(),
                state_label: agent.state.label().to_string(),
                state_color: state_color(&agent.state),
                agent_color: agent_color(agent.color_index),
                kind_label,
                kind_color,
                position: (agent.position.x, agent.position.y),
                inbox_count: agent.inbox.len(),
            }
        })
        .collect();

    let agent_count = agents.len();
    drop(registry);

    // Load tasks and activity per agent from DB
    let mut agent_tasks: std::collections::HashMap<AgentId, Vec<TaskRecord>> =
        std::collections::HashMap::new();
    let mut agent_activity: std::collections::HashMap<AgentId, Vec<ActivityEntry>> =
        std::collections::HashMap::new();
    if let Ok(db) = app.db.lock() {
        for agent in &agents {
            if let Ok(tasks) = db.load_tasks(&agent.id, 3)
                && !tasks.is_empty()
            {
                agent_tasks.insert(agent.id, tasks);
            }
            if let Ok(entries) = db.load_activity(&agent.id, 3)
                && !entries.is_empty()
            {
                agent_activity.insert(agent.id, entries);
            }
        }
    }

    // Build card lines for each agent
    let mut all_lines: Vec<Line> = Vec::new();

    for agent in &agents {
        // ── Card top border ──
        all_lines.push(Line::from(vec![Span::styled(
            "  ╭──────────────────────────────────────╮",
            Style::default().fg(Color::Rgb(50, 50, 70)),
        )]));

        // Row 1: ● name   [kind]
        all_lines.push(Line::from(vec![
            Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
            Span::styled("● ", Style::default().fg(agent.state_color)),
            Span::styled(
                &agent.name,
                Style::default()
                    .fg(agent.agent_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" {} ", agent.state_label),
                Style::default()
                    .fg(Color::Rgb(20, 20, 30))
                    .bg(agent.state_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(agent.kind_label, Style::default().fg(agent.kind_color)),
            {
                if agent.inbox_count > 0 {
                    Span::styled(
                        format!("  {}msg", agent.inbox_count),
                        Style::default()
                            .fg(Color::Rgb(255, 100, 100))
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw("")
                }
            },
        ]));

        // Row 2: ID + position
        all_lines.push(Line::from(vec![
            Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
            Span::styled(
                format!("{:.8}", agent.id),
                Style::default().fg(Color::Rgb(70, 70, 90)),
            ),
            Span::styled(
                format!("  ({},{})", agent.position.0, agent.position.1),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            ),
        ]));

        // ── Tasks section (if any) ──
        if let Some(tasks) = agent_tasks.get(&agent.id) {
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
                Span::styled(
                    "╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                    Style::default().fg(Color::Rgb(40, 40, 55)),
                ),
            ]));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
                Span::styled(
                    "Tasks",
                    Style::default()
                        .fg(Color::Rgb(120, 120, 150))
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            for task in tasks.iter().take(3) {
                let dot_color = match task.state.as_str() {
                    "completed" => Color::Green,
                    "failed" => Color::Red,
                    "running" => Color::Rgb(80, 180, 220),
                    _ => Color::Yellow,
                };
                let summary = task
                    .response_summary
                    .as_deref()
                    .or(task.scope.as_deref())
                    .unwrap_or(&task.task_id[..task.task_id.len().min(20)]);
                all_lines.push(Line::from(vec![
                    Span::styled("  │  ", Style::default().fg(Color::Rgb(50, 50, 70))),
                    Span::styled("● ", Style::default().fg(dot_color)),
                    Span::styled(
                        format!("{:>9}", task.state),
                        Style::default().fg(dot_color),
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(summary, Style::default().fg(Color::Rgb(160, 160, 180))),
                ]));
            }
        }

        // ── Activity section (if any) ──
        if let Some(entries) = agent_activity.get(&agent.id) {
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
                Span::styled(
                    "╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                    Style::default().fg(Color::Rgb(40, 40, 55)),
                ),
            ]));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", Style::default().fg(Color::Rgb(50, 50, 70))),
                Span::styled(
                    "Activity",
                    Style::default()
                        .fg(Color::Rgb(120, 120, 150))
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            for entry in entries.iter().rev().take(3) {
                let secs = (chrono::Utc::now() - entry.timestamp).num_seconds();
                let ago = if secs < 60 {
                    format!("{}s", secs)
                } else if secs < 3600 {
                    format!("{}m", secs / 60)
                } else {
                    format!("{}h", secs / 3600)
                };
                // Truncate detail to fit card
                let detail = if entry.detail.len() > 30 {
                    format!("{}...", &entry.detail[..27])
                } else {
                    entry.detail.clone()
                };
                all_lines.push(Line::from(vec![
                    Span::styled("  │  ", Style::default().fg(Color::Rgb(50, 50, 70))),
                    Span::styled(
                        format!("{:>4} ", ago),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(detail, Style::default().fg(Color::Rgb(140, 140, 160))),
                ]));
            }
        }

        // ── Card bottom border ──
        all_lines.push(Line::from(vec![Span::styled(
            "  ╰──────────────────────────────────────╯",
            Style::default().fg(Color::Rgb(50, 50, 70)),
        )]));
        all_lines.push(Line::raw(""));
    }

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

    let paragraph = Paragraph::new(all_lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_activity_feed(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

    // Load activity from DB (persisted across sessions)
    let agent_ids: Vec<(AgentId, String, u8)> = registry
        .agents()
        .map(|a| (a.id, a.name.clone(), a.color_index))
        .collect();
    drop(registry);

    let mut all_entries: Vec<(String, u8, ActivityEntry)> = Vec::new();
    if let Ok(db) = app.db.lock() {
        for (id, name, color_idx) in &agent_ids {
            if let Ok(entries) = db.load_activity(id, 20) {
                for entry in entries {
                    all_entries.push((name.clone(), *color_idx, entry));
                }
            }
        }
    }
    all_entries.sort_by(|a, b| b.2.timestamp.cmp(&a.2.timestamp));
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
            .map(|(name, color_idx, entry)| {
                let a_color = agent_color(*color_idx);

                Line::from(vec![
                    Span::styled(
                        format!(" {} ", entry.timestamp.format("%H:%M:%S")),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(
                        name.as_str(),
                        Style::default().fg(a_color).add_modifier(Modifier::BOLD),
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
    let Ok(registry) = app.registry.read() else {
        return;
    };

    // Load tasks from DB (persisted across sessions)
    let agent_ids: Vec<(AgentId, String)> =
        registry.agents().map(|a| (a.id, a.name.clone())).collect();
    drop(registry);

    let mut all_tasks: Vec<(String, TaskRecord)> = Vec::new();
    if let Ok(db) = app.db.lock() {
        for (id, name) in &agent_ids {
            if let Ok(tasks) = db.load_tasks(id, 50) {
                for task in tasks {
                    all_tasks.push((name.clone(), task));
                }
            }
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
            .map(|(agent_name, task)| {
                let state_bg = match task.state.as_str() {
                    "submitted" => Color::Yellow,
                    "running" => Color::Rgb(80, 180, 220),
                    "completed" => Color::Green,
                    "failed" => Color::Red,
                    _ => Color::Gray,
                };

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
                    Span::styled(agent_name.as_str(), Style::default().fg(Color::Cyan)),
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        task.response_summary
                            .as_deref()
                            .or(task.scope.as_deref())
                            .unwrap_or(&task.task_id[..task.task_id.len().min(20)]),
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
