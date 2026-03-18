use crate::agent::AgentId;
use crate::api::observability::{ActivityEntry, TaskRecord};
use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::{App, McPanel};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

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

    // Detail popup overlay
    if app.mc_detail_open {
        draw_detail_popup(frame, app, area);
    }
}

/// Truncate a string to fit within `max` display characters (char-boundary safe)
fn trunc(s: &str, max: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max {
        s.to_string()
    } else if max > 3 {
        let truncated: String = s.chars().take(max - 3).collect();
        format!("{}...", truncated)
    } else {
        s.chars().take(max).collect()
    }
}

fn draw_agent_cards(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

    struct AgentInfo {
        id: AgentId,
        name: String,
        state_label: String,
        s_color: Color,
        a_color: Color,
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
                s_color: state_color(&agent.state),
                a_color: agent_color(agent.color_index),
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

    // Inner width = area.width - 2 (block borders) - 2 (padding)
    let inner_w = area.width.saturating_sub(4) as usize;
    // Card content width (inside card borders)
    let card_w = inner_w.saturating_sub(4); // "  │ " prefix = 4 chars
    let border_w = inner_w.saturating_sub(2); // "  ╭...╮" = 2 chars overhead

    let sep = Style::default().fg(Color::Rgb(40, 40, 55));

    let mut all_lines: Vec<Line> = Vec::new();

    let is_focused = app.mc_panel == McPanel::Agents;
    let selected = if agent_count > 0 {
        app.mc_selected.min(agent_count - 1)
    } else {
        0
    };
    // Track starting line of each card for scroll calculation
    let mut card_start_lines: Vec<usize> = Vec::new();

    for (idx, agent) in agents.iter().enumerate() {
        card_start_lines.push(all_lines.len());
        let is_selected = is_focused && idx == selected;

        let border_color = if is_selected {
            Color::Cyan
        } else {
            Color::Rgb(50, 50, 70)
        };
        let bd = Style::default().fg(border_color);

        // Top border
        let border_fill: String = "─".repeat(border_w.saturating_sub(2));
        all_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("╭{}╮", border_fill), bd),
        ]));

        // Row 1: ● name  state  kind  [Nmsg]
        let mut row1 = vec![
            Span::styled("  │ ", bd),
            Span::styled("● ", Style::default().fg(agent.s_color)),
            Span::styled(
                trunc(&agent.name, card_w / 3),
                Style::default()
                    .fg(agent.a_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" {} ", agent.state_label),
                Style::default()
                    .fg(Color::Rgb(20, 20, 30))
                    .bg(agent.s_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(agent.kind_label, Style::default().fg(agent.kind_color)),
        ];
        if agent.inbox_count > 0 {
            row1.push(Span::styled(
                format!(" {}msg", agent.inbox_count),
                Style::default()
                    .fg(Color::Rgb(255, 100, 100))
                    .add_modifier(Modifier::BOLD),
            ));
        }
        all_lines.push(Line::from(row1));

        // Row 2: ID + position
        all_lines.push(Line::from(vec![
            Span::styled("  │ ", bd),
            Span::styled(
                format!("{:.8}", agent.id),
                Style::default().fg(Color::Rgb(70, 70, 90)),
            ),
            Span::styled(
                format!("  ({},{})", agent.position.0, agent.position.1),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            ),
        ]));

        // Tasks section
        if let Some(tasks) = agent_tasks.get(&agent.id) {
            let sep_fill: String = "╌".repeat(border_w.saturating_sub(2));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", bd),
                Span::styled(sep_fill.clone(), sep),
            ]));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", bd),
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
                let max_summary = card_w.saturating_sub(15);
                all_lines.push(Line::from(vec![
                    Span::styled("  │  ", bd),
                    Span::styled("● ", Style::default().fg(dot_color)),
                    Span::styled(
                        format!("{:>9} ", task.state),
                        Style::default().fg(dot_color),
                    ),
                    Span::styled(
                        trunc(summary, max_summary),
                        Style::default().fg(Color::Rgb(160, 160, 180)),
                    ),
                ]));
            }
        }

        // Activity section
        if let Some(entries) = agent_activity.get(&agent.id) {
            let sep_fill: String = "╌".repeat(border_w.saturating_sub(2));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", bd),
                Span::styled(sep_fill.clone(), sep),
            ]));
            all_lines.push(Line::from(vec![
                Span::styled("  │ ", bd),
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
                let max_detail = card_w.saturating_sub(8);
                all_lines.push(Line::from(vec![
                    Span::styled("  │  ", bd),
                    Span::styled(
                        format!("{:>4} ", ago),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(
                        trunc(&entry.detail, max_detail),
                        Style::default().fg(Color::Rgb(140, 140, 160)),
                    ),
                ]));
            }
        }

        // Bottom border
        let border_fill: String = "─".repeat(border_w.saturating_sub(2));
        all_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("╰{}╯", border_fill), bd),
        ]));

        // Spacer between cards
        if idx + 1 < agents.len() {
            all_lines.push(Line::raw(""));
        }
    }

    let title = format!(" Agents ({}) ", agent_count);
    let panel_border = if is_focused {
        Color::Cyan
    } else {
        Color::Rgb(50, 50, 70)
    };
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(panel_border))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::new(0, 0, 1, 0));

    // Auto-scroll to keep selected card visible
    let visible_h = area.height.saturating_sub(4) as usize; // borders + top padding
    let scroll = if is_focused && !card_start_lines.is_empty() && selected < card_start_lines.len()
    {
        let card_line = card_start_lines[selected];
        if card_line >= visible_h {
            (card_line - visible_h / 3) as u16
        } else {
            0
        }
    } else {
        0
    };

    let paragraph = Paragraph::new(all_lines).block(block).scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn draw_activity_feed(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

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

    let inner_w = area.width.saturating_sub(4) as usize;

    let lines: Vec<Line> = if all_entries.is_empty() {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No activity yet",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    } else {
        let is_focused = app.mc_panel == McPanel::Activity;
        let selected = app.mc_selected.min(all_entries.len().saturating_sub(1));
        all_entries
            .iter()
            .enumerate()
            .map(|(idx, (name, color_idx, entry))| {
                let a_color = agent_color(*color_idx);
                let prefix_len = 10 + name.len() + 2;
                let max_detail = inner_w.saturating_sub(prefix_len);
                let is_sel = is_focused && idx == selected;

                let mut line = Line::from(vec![
                    Span::styled(
                        if is_sel { "▸" } else { " " },
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{} ", entry.timestamp.format("%H:%M:%S")),
                        Style::default().fg(Color::Rgb(80, 80, 100)),
                    ),
                    Span::styled(
                        name.as_str(),
                        Style::default().fg(a_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        trunc(&entry.detail, max_detail),
                        Style::default().fg(Color::Rgb(180, 180, 200)),
                    ),
                ]);
                if is_sel {
                    line = line.style(Style::default().bg(Color::Rgb(30, 30, 45)));
                }
                line
            })
            .collect()
    };

    let is_focused = app.mc_panel == McPanel::Activity;
    let title = format!(" Activity ({}) ", all_entries.len());
    let panel_border = if is_focused {
        Color::Yellow
    } else {
        Color::Rgb(50, 50, 70)
    };
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(panel_border))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::new(0, 0, 1, 0));

    // Auto-scroll to keep selected item visible
    let visible_h = area.height.saturating_sub(4) as usize; // borders + top padding
    let entry_count = all_entries.len();
    let scroll = if is_focused && entry_count > 0 {
        let sel = app.mc_selected.min(entry_count.saturating_sub(1));
        if sel >= visible_h {
            (sel - visible_h + 1) as u16
        } else {
            0
        }
    } else {
        0
    };

    let paragraph = Paragraph::new(lines).block(block).scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn draw_task_list(frame: &mut Frame, app: &App, area: Rect) {
    let Ok(registry) = app.registry.read() else {
        return;
    };

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

    let inner_w = area.width.saturating_sub(4) as usize;

    let lines: Vec<Line> = if all_tasks.is_empty() {
        vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "  No tasks",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )]),
        ]
    } else {
        let is_focused = app.mc_panel == McPanel::Tasks;
        let selected = app.mc_selected.min(all_tasks.len().saturating_sub(1));
        all_tasks
            .iter()
            .enumerate()
            .map(|(idx, (agent_name, task))| {
                let state_bg = match task.state.as_str() {
                    "submitted" => Color::Yellow,
                    "running" => Color::Rgb(80, 180, 220),
                    "completed" => Color::Green,
                    "failed" => Color::Red,
                    _ => Color::Gray,
                };
                let is_sel = is_focused && idx == selected;

                let prefix_len = 3 + 11 + 1 + agent_name.len() + 2;
                let max_summary = inner_w.saturating_sub(prefix_len);
                let summary = task
                    .response_summary
                    .as_deref()
                    .or(task.scope.as_deref())
                    .unwrap_or(&task.task_id[..task.task_id.len().min(20)]);

                let mut line = Line::from(vec![
                    Span::styled(
                        if is_sel { " ▸" } else { "  " },
                        Style::default().fg(Color::Cyan),
                    ),
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
                        trunc(summary, max_summary),
                        Style::default().fg(Color::Rgb(180, 180, 200)),
                    ),
                ]);
                if is_sel {
                    line = line.style(Style::default().bg(Color::Rgb(30, 30, 45)));
                }
                line
            })
            .collect()
    };

    let is_focused = app.mc_panel == McPanel::Tasks;
    let title = format!(" Tasks ({}) ", all_tasks.len());
    let panel_border = if is_focused {
        Color::Green
    } else {
        Color::Rgb(50, 50, 70)
    };
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(panel_border))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::new(0, 0, 1, 0));

    // Auto-scroll to keep selected task visible
    let visible_h = area.height.saturating_sub(4) as usize;
    let task_count = all_tasks.len();
    let scroll = if is_focused && task_count > 0 {
        let sel = app.mc_selected.min(task_count.saturating_sub(1));
        if sel >= visible_h {
            (sel - visible_h + 1) as u16
        } else {
            0
        }
    } else {
        0
    };

    let paragraph = Paragraph::new(lines).block(block).scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn draw_detail_popup(frame: &mut Frame, app: &App, area: Rect) {
    let pw = (area.width * 60 / 100).max(30);
    let ph = (area.height * 70 / 100).max(10);
    let px = area.x + (area.width.saturating_sub(pw)) / 2;
    let py = area.y + (area.height.saturating_sub(ph)) / 2;
    let popup = Rect::new(px, py, pw, ph);

    frame.render_widget(Clear, popup);

    let inner_w = pw.saturating_sub(4) as usize;
    let mut lines: Vec<Line> = Vec::new();

    match app.mc_panel {
        McPanel::Agents => {
            let Ok(registry) = app.registry.read() else {
                return;
            };
            let agents: Vec<_> = registry.agents().collect();
            if app.mc_selected >= agents.len() {
                lines.push(Line::from(Span::styled(
                    "  No agent selected",
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));
            } else {
                let agent = agents[app.mc_selected];
                let color = agent_color(agent.color_index);
                let s_color = state_color(&agent.state);

                let agent_name = agent.name.clone();
                let agent_id_str = format!("{:.8}", agent.id);
                let pos_str = format!("({}, {})", agent.position.x, agent.position.y);
                let state_str = format!(" {} ", agent.state.label());
                let task_count_str = agent.task_count.to_string();
                let task_count = agent.task_count;
                let inbox_empty = agent.inbox.is_empty();
                let inbox_len = agent.inbox.len();
                let inbox_lines: Vec<(String, String)> = agent
                    .inbox
                    .iter()
                    .rev()
                    .take(10)
                    .map(|msg| {
                        let sender = registry
                            .get(&msg.from)
                            .map(|a| a.name.clone())
                            .unwrap_or_else(|| format!("{:.8}", msg.from));
                        (sender, msg.text.clone())
                    })
                    .collect();
                drop(agents);
                drop(registry);

                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        agent_name,
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        state_str,
                        Style::default()
                            .fg(Color::Rgb(20, 20, 30))
                            .bg(s_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(Span::styled(
                    "  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                    Style::default().fg(Color::Rgb(60, 60, 80)),
                )));
                lines.push(Line::raw(""));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("ID      ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(agent_id_str, Style::default().fg(Color::Rgb(180, 180, 190))),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("Pos     ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(pos_str, Style::default().fg(Color::Rgb(180, 180, 190))),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("Tasks   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(
                        task_count_str,
                        Style::default().fg(if task_count > 0 {
                            Color::Cyan
                        } else {
                            Color::Rgb(180, 180, 190)
                        }),
                    ),
                ]));
                lines.push(Line::raw(""));

                if inbox_empty {
                    lines.push(Line::from(vec![
                        Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                        Span::styled("Inbox   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                        Span::styled("empty", Style::default().fg(Color::Rgb(80, 80, 100))),
                    ]));
                } else {
                    lines.push(Line::from(Span::styled(
                        format!("  Inbox ({})", inbox_len),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                    lines.push(Line::from(Span::styled(
                        "  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                        Style::default().fg(Color::Rgb(60, 60, 80)),
                    )));
                    for (sender_name, msg_text) in &inbox_lines {
                        let max_text = inner_w.saturating_sub(sender_name.len() + 8);
                        lines.push(Line::from(vec![
                            Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                            Span::styled(
                                format!("{} ", sender_name),
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                trunc(msg_text, max_text),
                                Style::default().fg(Color::Rgb(200, 200, 210)),
                            ),
                        ]));
                    }
                }
            }
        }
        McPanel::Tasks => {
            let Ok(registry) = app.registry.read() else {
                return;
            };
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

            if app.mc_selected >= all_tasks.len() {
                lines.push(Line::from(Span::styled(
                    "  No task selected",
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));
            } else {
                let agent_name = all_tasks[app.mc_selected].0.clone();
                let task_id = all_tasks[app.mc_selected].1.task_id.clone();
                let task_state = all_tasks[app.mc_selected].1.state.clone();
                let task_scope = all_tasks[app.mc_selected].1.scope.clone();
                let task_summary = all_tasks[app.mc_selected].1.response_summary.clone();
                let task_updated = all_tasks[app.mc_selected]
                    .1
                    .last_updated
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                let st_color = match task_state.as_str() {
                    "completed" => Color::Green,
                    "failed" => Color::Red,
                    "running" => Color::Rgb(80, 180, 220),
                    _ => Color::Yellow,
                };

                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        task_id,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(Span::styled(
                    "  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌",
                    Style::default().fg(Color::Rgb(60, 60, 80)),
                )));
                lines.push(Line::raw(""));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("Agent   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(agent_name, Style::default().fg(Color::Cyan)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("State   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(
                        format!(" {} ", task_state),
                        Style::default()
                            .fg(Color::Rgb(20, 20, 30))
                            .bg(st_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                if let Some(ref scope) = task_scope {
                    lines.push(Line::from(vec![
                        Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                        Span::styled("Scope   ", Style::default().fg(Color::Rgb(100, 100, 120))),
                        Span::styled(
                            trunc(scope, inner_w.saturating_sub(14)),
                            Style::default().fg(Color::Rgb(180, 180, 190)),
                        ),
                    ]));
                }
                if let Some(ref summary) = task_summary {
                    lines.push(Line::raw(""));
                    lines.push(Line::from(Span::styled(
                        "  Summary",
                        Style::default()
                            .fg(Color::Rgb(120, 120, 150))
                            .add_modifier(Modifier::BOLD),
                    )));
                    let max_line = inner_w.saturating_sub(4).max(1);
                    let chars: Vec<char> = summary.chars().collect();
                    for chunk in chars.chunks(max_line) {
                        let text: String = chunk.iter().collect();
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(text, Style::default().fg(Color::Rgb(180, 180, 200))),
                        ]));
                    }
                }
                lines.push(Line::raw(""));
                lines.push(Line::from(vec![
                    Span::styled("  ┃ ", Style::default().fg(Color::Rgb(60, 60, 80))),
                    Span::styled("Updated ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled(task_updated, Style::default().fg(Color::Rgb(140, 140, 160))),
                ]));
            }
        }
        McPanel::Activity => {
            lines.push(Line::from(Span::styled(
                "  Activity detail — press Esc to close",
                Style::default().fg(Color::Rgb(120, 120, 150)),
            )));
        }
    }

    let title = match app.mc_panel {
        McPanel::Agents => " Agent Detail ",
        McPanel::Tasks => " Task Detail ",
        McPanel::Activity => " Activity ",
    };

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::new(1, 1, 1, 1));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, popup);
}
