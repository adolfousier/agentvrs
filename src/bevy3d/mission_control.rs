use crate::bevy3d::bridge::WorldBridge;
use crate::bevy3d::runner::ThemeState;
use bevy::prelude::*;

// ── Marker components ────────────────────────────────────────────────────────

#[derive(Component)]
pub struct MissionControlRoot;

#[derive(Component)]
pub struct McAgentCard;

#[derive(Component)]
pub struct McActivityFeed;

#[derive(Component)]
pub struct McTaskList;

/// Temporary child marker for cleanup each frame.
#[derive(Component)]
pub(crate) struct McChild;

/// Marker for the "Mission Control" title text.
#[derive(Component)]
pub(crate) struct McTitle;

/// Marker for the "press M to close" hint text.
#[derive(Component)]
pub(crate) struct McHint;

/// Marker for section heading texts (Agents, Recent Activity, Tasks).
#[derive(Component)]
pub(crate) struct McHeading;

/// Marker for the "Recent Activity" heading (updated when agent is selected).
#[derive(Component)]
pub(crate) struct McActivityHeading;

/// Marker for the "Tasks" heading (updated when agent is selected).
#[derive(Component)]
pub(crate) struct McTaskHeading;

/// Marker for a clickable task row — stores agent name + task record data.
#[derive(Component, Clone)]
pub struct McTaskRowButton {
    pub agent_name: String,
    pub task_id: String,
    pub state: String,
    pub summary: String,
    pub submitted_at: String,
    pub last_updated: String,
}

/// Marker for the task detail popup overlay.
#[derive(Component)]
pub struct McTaskPopup;

/// Resource: mission control panel open/closed state.
#[derive(Resource, Default)]
pub struct MissionControlState {
    pub open: bool,
    /// Currently selected agent in Mission Control (filters right panel).
    pub selected_agent: Option<crate::agent::AgentId>,
    /// Whether a task detail popup is currently shown.
    pub popup_open: bool,
}

/// Marker component on clickable agent cards, stores the agent ID.
#[derive(Component)]
pub struct McCardButton(pub crate::agent::AgentId);

// ── Theme palette ────────────────────────────────────────────────────────────

struct McTheme {
    bg: Color,
    card_bg: Color,
    card_border: Color,
    section_bg: Color,
    section_border: Color,
    title: Color,
    heading: Color,
    text_primary: Color,
    text_secondary: Color,
    text_muted: Color,
    link: Color,
    badge_bg: Color,
    badge_text: Color,
    separator: Color,
}

fn mc_theme(is_dark: bool) -> McTheme {
    if is_dark {
        McTheme {
            bg: Color::srgb(0.09, 0.09, 0.11),
            card_bg: Color::srgb(0.13, 0.14, 0.17),
            card_border: Color::srgb(0.20, 0.21, 0.25),
            section_bg: Color::srgb(0.11, 0.12, 0.14),
            section_border: Color::srgb(0.18, 0.19, 0.23),
            title: Color::srgb(0.93, 0.93, 0.95),
            heading: Color::srgb(0.58, 0.68, 0.90),
            text_primary: Color::srgb(0.88, 0.88, 0.90),
            text_secondary: Color::srgb(0.56, 0.58, 0.63),
            text_muted: Color::srgb(0.38, 0.40, 0.45),
            link: Color::srgb(0.35, 0.60, 1.0),
            badge_bg: Color::srgb(0.20, 0.35, 0.65),
            badge_text: Color::srgb(0.85, 0.90, 1.0),
            separator: Color::srgb(0.18, 0.19, 0.23),
        }
    } else {
        McTheme {
            bg: Color::srgb(0.94, 0.95, 0.97),
            card_bg: Color::srgb(1.0, 1.0, 1.0),
            card_border: Color::srgb(0.82, 0.84, 0.87),
            section_bg: Color::srgb(0.97, 0.97, 0.98),
            section_border: Color::srgb(0.85, 0.86, 0.89),
            title: Color::srgb(0.12, 0.13, 0.16),
            heading: Color::srgb(0.22, 0.30, 0.55),
            text_primary: Color::srgb(0.15, 0.16, 0.20),
            text_secondary: Color::srgb(0.38, 0.40, 0.46),
            text_muted: Color::srgb(0.55, 0.57, 0.62),
            link: Color::srgb(0.08, 0.38, 0.78),
            badge_bg: Color::srgb(0.22, 0.44, 0.72),
            badge_text: Color::WHITE,
            separator: Color::srgb(0.85, 0.86, 0.89),
        }
    }
}

// ── Toggle with M key ────────────────────────────────────────────────────────

pub fn toggle_mission_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut mc_state: ResMut<MissionControlState>,
    mut mc_root_q: Query<&mut Visibility, With<MissionControlRoot>>,
    popup_q: Query<Entity, With<McTaskPopup>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        // If popup is open, M closes the popup first (not the whole MC)
        if mc_state.popup_open {
            for entity in popup_q.iter() {
                commands.entity(entity).despawn();
            }
            mc_state.popup_open = false;
            return;
        }
        mc_state.open = !mc_state.open;
        if !mc_state.open {
            mc_state.selected_agent = None;
        }
        for mut vis in mc_root_q.iter_mut() {
            *vis = if mc_state.open {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

// ── Setup: build the full-screen overlay UI ──────────────────────────────────

pub fn setup_mission_control(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(32.0)),
                row_gap: Val::Px(20.0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.09, 0.11)),
            Visibility::Hidden,
            MissionControlRoot,
        ))
        .with_children(|root| {
            // ── Header row ───────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(12.0)),
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("Mission Control"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.93, 0.93, 0.95)),
                    McTitle,
                ));
                header.spawn((
                    Text::new("press M to close"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.38, 0.40, 0.45)),
                    McHint,
                ));
            });

            // ── Content: two columns ─────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                column_gap: Val::Px(20.0),
                overflow: Overflow::scroll_y(),
                ..default()
            })
            .with_children(|content| {
                // Left column: Agent cards (grid layout)
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(45.0),
                        row_gap: Val::Px(12.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    })
                    .with_children(|left| {
                        // Section header
                        left.spawn((
                            Text::new("Agents"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.58, 0.68, 0.90)),
                            McHeading,
                        ));
                        // Agent cards container — wrapping row for grid
                        left.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(10.0),
                                column_gap: Val::Px(10.0),
                                flex_grow: 1.0,
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            McAgentCard,
                        ));
                    });

                // Right column: Activity feed + Tasks
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        row_gap: Val::Px(16.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    })
                    .with_children(|right| {
                        // Activity section
                        right.spawn((
                            Text::new("Recent Activity"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.58, 0.68, 0.90)),
                            McHeading,
                            McActivityHeading,
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(0.0),
                                height: Val::Percent(50.0),
                                overflow: Overflow::scroll_y(),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.11, 0.12, 0.14)),
                            BorderColor::all(Color::srgb(0.18, 0.19, 0.23)),
                            McActivityFeed,
                        ));

                        // Tasks section
                        right.spawn((
                            Text::new("Tasks"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.58, 0.68, 0.90)),
                            McHeading,
                            McTaskHeading,
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(0.0),
                                flex_grow: 1.0,
                                overflow: Overflow::scroll_y(),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.11, 0.12, 0.14)),
                            BorderColor::all(Color::srgb(0.18, 0.19, 0.23)),
                            McTaskList,
                        ));
                    });
            });
        });
}

// ── Per-frame update: populate agent cards, activity feed, tasks ──────────────

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update_mission_control(
    mut commands: Commands,
    mc_state: Res<MissionControlState>,
    bridge: Res<WorldBridge>,
    theme: Res<ThemeState>,
    mut root_q: Query<&mut BackgroundColor, With<MissionControlRoot>>,
    card_q: Query<Entity, With<McAgentCard>>,
    feed_q: Query<Entity, With<McActivityFeed>>,
    task_q: Query<Entity, With<McTaskList>>,
    child_q: Query<Entity, With<McChild>>,
    mut title_q: Query<&mut TextColor, With<McTitle>>,
    mut hint_q: Query<&mut TextColor, (With<McHint>, Without<McTitle>)>,
    mut heading_q: Query<&mut TextColor, (With<McHeading>, Without<McTitle>, Without<McHint>)>,
) {
    if !mc_state.open {
        return;
    }

    let t = mc_theme(theme.is_dark);

    // Update root background for theme
    for mut bg in root_q.iter_mut() {
        bg.0 = t.bg;
    }

    // Update static text colors for theme
    for mut tc in title_q.iter_mut() {
        tc.0 = t.title;
    }
    for mut tc in hint_q.iter_mut() {
        tc.0 = t.text_muted;
    }
    for mut tc in heading_q.iter_mut() {
        tc.0 = t.heading;
    }

    // Remove previous frame's children
    for entity in child_q.iter() {
        commands.entity(entity).despawn();
    }

    let Ok(registry) = bridge.registry.read() else { return };

    // ── Agent cards (GitHub-style repo cards) ─────────────────────
    if let Ok(card_parent) = card_q.single() {
        if registry.agents().next().is_none() {
            let empty = commands
                .spawn((
                    Text::new("No agents connected"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(t.text_muted),
                    McChild,
                ))
                .id();
            commands.entity(card_parent).add_child(empty);
        }

        // Load per-agent tasks and activity from DB
        let mut agent_tasks: std::collections::HashMap<crate::agent::AgentId, Vec<crate::api::observability::TaskRecord>> =
            std::collections::HashMap::new();
        let mut agent_activity: std::collections::HashMap<crate::agent::AgentId, Vec<crate::api::observability::ActivityEntry>> =
            std::collections::HashMap::new();
        if let Ok(db) = bridge.db.lock() {
            for agent in registry.agents() {
                if let Ok(tasks) = db.load_tasks(&agent.id, 3) {
                    agent_tasks.insert(agent.id, tasks);
                }
                if let Ok(entries) = db.load_activity(&agent.id, 3) {
                    agent_activity.insert(agent.id, entries);
                }
            }
        }

        for agent in registry.agents() {
            let dot_color = state_color(&agent.state);
            let kind_label = match &agent.kind {
                crate::agent::AgentKind::Local => "local",
                crate::agent::AgentKind::External { .. } => "external",
                crate::agent::AgentKind::OpenCrabs { .. } => "opencrabs",
            };

            let tasks = agent_tasks.get(&agent.id);
            let activity = agent_activity.get(&agent.id);
            let is_selected = mc_state.selected_agent == Some(agent.id);
            let card_border_color = if is_selected { t.link } else { t.card_border };

            let card = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(14.0)),
                        row_gap: Val::Px(6.0),
                        width: Val::Px(280.0),
                        border: UiRect::all(Val::Px(if is_selected { 2.0 } else { 1.0 })),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(t.card_bg),
                    BorderColor::all(card_border_color),
                    Interaction::default(),
                    McCardButton(agent.id),
                    McChild,
                ))
                .with_children(|card| {
                    // Top row: name + badge
                    card.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|top| {
                        top.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(8.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|name_row| {
                            name_row.spawn((
                                Node {
                                    width: Val::Px(10.0),
                                    height: Val::Px(10.0),
                                    border_radius: BorderRadius::all(Val::Px(5.0)),
                                    ..default()
                                },
                                BackgroundColor(dot_color),
                            ));
                            name_row.spawn((
                                Text::new(&agent.name),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(t.text_primary),
                            ));
                        });
                        // Kind badge
                        top.spawn((
                            Node {
                                padding: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(2.0), Val::Px(2.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderColor::all(t.card_border),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                Text::new(kind_label),
                                TextFont { font_size: 10.0, ..default() },
                                TextColor(t.text_secondary),
                            ));
                        });
                    });

                    // State + position row
                    card.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new(agent.state.label()),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(t.text_secondary),
                        ));
                        row.spawn((
                            Text::new(format!("({},{})", agent.position.x, agent.position.y)),
                            TextFont { font_size: 10.0, ..default() },
                            TextColor(t.text_muted),
                        ));
                        if !agent.inbox.is_empty() {
                            row.spawn((
                                Node {
                                    padding: UiRect::new(Val::Px(7.0), Val::Px(7.0), Val::Px(2.0), Val::Px(2.0)),
                                    border_radius: BorderRadius::all(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(t.badge_bg),
                            ))
                            .with_children(|badge| {
                                badge.spawn((
                                    Text::new(format!("{} msg", agent.inbox.len())),
                                    TextFont { font_size: 10.0, ..default() },
                                    TextColor(t.badge_text),
                                ));
                            });
                        }
                    });

                    // ── Recent tasks section ──
                    if let Some(tasks) = tasks {
                        if !tasks.is_empty() {
                            // Separator
                            card.spawn((
                                Node { height: Val::Px(1.0), ..default() },
                                BackgroundColor(t.separator),
                            ));
                            card.spawn((
                                Text::new("Tasks"),
                                TextFont { font_size: 10.0, ..default() },
                                TextColor(t.heading),
                            ));
                            for task in tasks.iter().rev().take(3) {
                                let task_dot = match task.state.as_str() {
                                    "completed" => Color::srgb(0.2, 0.8, 0.2),
                                    "failed" => Color::srgb(1.0, 0.3, 0.3),
                                    _ => Color::srgb(0.3, 0.7, 1.0),
                                };
                                card.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(6.0),
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Node {
                                            width: Val::Px(6.0),
                                            height: Val::Px(6.0),
                                            border_radius: BorderRadius::all(Val::Px(3.0)),
                                            margin: UiRect::top(Val::Px(4.0)),
                                            ..default()
                                        },
                                        BackgroundColor(task_dot),
                                    ));
                                    // State badge
                                    row.spawn((
                                        Node {
                                            padding: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(1.0), Val::Px(1.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(8.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::NONE),
                                        BorderColor::all(task_dot),
                                    ))
                                    .with_children(|b| {
                                        b.spawn((
                                            Text::new(&task.state),
                                            TextFont { font_size: 9.0, ..default() },
                                            TextColor(task_dot),
                                        ));
                                    });
                                    // Summary — full text, wrapping
                                    let summary = task.response_summary.as_deref()
                                        .unwrap_or(&task.task_id);
                                    row.spawn((
                                        Text::new(summary),
                                        TextFont { font_size: 10.0, ..default() },
                                        TextColor(t.text_secondary),
                                        Node {
                                            flex_shrink: 1.0,
                                            ..default()
                                        },
                                    ));
                                });
                            }
                        }
                    }

                    // ── Recent activity section ──
                    if let Some(entries) = activity {
                        if !entries.is_empty() {
                            card.spawn((
                                Node { height: Val::Px(1.0), ..default() },
                                BackgroundColor(t.separator),
                            ));
                            card.spawn((
                                Text::new("Activity"),
                                TextFont { font_size: 10.0, ..default() },
                                TextColor(t.heading),
                            ));
                            for entry in entries.iter().rev().take(3) {
                                let secs = (chrono::Utc::now() - entry.timestamp).num_seconds();
                                let ago = if secs < 60 {
                                    format!("{}s", secs)
                                } else if secs < 3600 {
                                    format!("{}m", secs / 60)
                                } else {
                                    format!("{}h", secs / 3600)
                                };
                                card.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(6.0),
                                    align_items: AlignItems::FlexStart,
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new(&ago),
                                        TextFont { font_size: 9.0, ..default() },
                                        TextColor(t.text_muted),
                                        Node { min_width: Val::Px(28.0), ..default() },
                                    ));
                                    row.spawn((
                                        Text::new(&entry.detail),
                                        TextFont { font_size: 10.0, ..default() },
                                        TextColor(t.text_secondary),
                                        Node { flex_shrink: 1.0, ..default() },
                                    ));
                                });
                            }
                        }
                    }
                })
                .id();
            commands.entity(card_parent).add_child(card);
        }
    }

    // ── Activity feed (from DB) — GitHub-style list rows ─────────
    if let Ok(feed_parent) = feed_q.single() {
        // Update section background + border for theme
        commands.entity(feed_parent).insert((
            BackgroundColor(t.section_bg),
            BorderColor::all(t.section_border),
        ));

        let mut all_activity: Vec<(String, crate::api::observability::ActivityEntry)> = Vec::new();
        if let Ok(db) = bridge.db.lock() {
            let agents_to_show: Vec<_> = match mc_state.selected_agent {
                Some(sel) => registry.agents().filter(|a| a.id == sel).collect(),
                None => registry.agents().collect(),
            };
            for agent in &agents_to_show {
                let limit = if mc_state.selected_agent.is_some() { 50 } else { 10 };
                if let Ok(entries) = db.load_activity(&agent.id, limit) {
                    for entry in entries {
                        all_activity.push((agent.name.clone(), entry));
                    }
                }
            }
        }
        all_activity.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        let max_items = if mc_state.selected_agent.is_some() { 50 } else { 20 };
        all_activity.truncate(max_items);

        if all_activity.is_empty() {
            let empty = commands
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(16.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("No activity recorded yet"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(t.text_muted),
                    ));
                })
                .id();
            commands.entity(feed_parent).add_child(empty);
        }

        for (i, (agent_name, entry)) in all_activity.iter().enumerate() {
            let time_ago = (chrono::Utc::now() - entry.timestamp).num_seconds();
            let time_str = if time_ago < 60 {
                format!("{}s ago", time_ago)
            } else if time_ago < 3600 {
                format!("{}m ago", time_ago / 60)
            } else {
                format!("{}h ago", time_ago / 3600)
            };

            let row_bg = if i % 2 == 0 {
                Color::NONE
            } else {
                t.separator.with_alpha(0.3)
            };

            let row = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::new(
                            Val::Px(14.0),
                            Val::Px(14.0),
                            Val::Px(6.0),
                            Val::Px(6.0),
                        ),
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(&time_str),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(t.text_muted),
                        Node {
                            min_width: Val::Px(55.0),
                            ..default()
                        },
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(t.link),
                        Node {
                            min_width: Val::Px(90.0),
                            ..default()
                        },
                    ));
                    let detail = entry.detail.clone();
                    row.spawn((
                        Text::new(detail),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(t.text_secondary),
                    ));
                })
                .id();
            commands.entity(feed_parent).add_child(row);
        }
    }

    // ── Task list (from DB) — GitHub-style list rows ─────────────
    if let Ok(task_parent) = task_q.single() {
        commands.entity(task_parent).insert((
            BackgroundColor(t.section_bg),
            BorderColor::all(t.section_border),
        ));

        let mut all_tasks: Vec<(String, crate::api::observability::TaskRecord)> = Vec::new();
        if let Ok(db) = bridge.db.lock() {
            let agents_to_show: Vec<_> = match mc_state.selected_agent {
                Some(sel) => registry.agents().filter(|a| a.id == sel).collect(),
                None => registry.agents().collect(),
            };
            for agent in &agents_to_show {
                let limit = if mc_state.selected_agent.is_some() { 50 } else { 10 };
                if let Ok(tasks) = db.load_tasks(&agent.id, limit) {
                    for task in tasks {
                        all_tasks.push((agent.name.clone(), task));
                    }
                }
            }
        }
        all_tasks.sort_by(|a, b| b.1.last_updated.cmp(&a.1.last_updated));
        let max_items = if mc_state.selected_agent.is_some() { 50 } else { 20 };
        all_tasks.truncate(max_items);

        if all_tasks.is_empty() {
            let empty = commands
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(16.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("No tasks submitted yet"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(t.text_muted),
                    ));
                })
                .id();
            commands.entity(task_parent).add_child(empty);
        }

        for (i, (agent_name, task)) in all_tasks.iter().enumerate() {
            let task_state_color = match task.state.as_str() {
                "completed" => Color::srgb(0.2, 0.8, 0.2),
                "failed" => Color::srgb(1.0, 0.3, 0.3),
                "submitted" | "running" => Color::srgb(0.3, 0.7, 1.0),
                _ => t.text_muted,
            };

            let row_bg = if i % 2 == 0 {
                Color::NONE
            } else {
                t.separator.with_alpha(0.3)
            };

            let row = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::new(
                            Val::Px(14.0),
                            Val::Px(14.0),
                            Val::Px(6.0),
                            Val::Px(6.0),
                        ),
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    Interaction::default(),
                    McTaskRowButton {
                        agent_name: agent_name.clone(),
                        task_id: task.task_id.clone(),
                        state: task.state.clone(),
                        summary: task.response_summary.clone().unwrap_or_default(),
                        submitted_at: task.submitted_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        last_updated: task.last_updated.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    McChild,
                ))
                .with_children(|row| {
                    // State dot
                    row.spawn((
                        Node {
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(task_state_color),
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(t.link),
                        Node {
                            min_width: Val::Px(90.0),
                            ..default()
                        },
                    ));
                    // State badge
                    row.spawn((
                        Node {
                            padding: UiRect::new(
                                Val::Px(7.0),
                                Val::Px(7.0),
                                Val::Px(1.0),
                                Val::Px(1.0),
                            ),
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(task_state_color),
                    ))
                    .with_children(|badge| {
                        badge.spawn((
                            Text::new(&task.state),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(task_state_color),
                        ));
                    });
                    let summary = task.response_summary.as_deref().unwrap_or(&task.task_id).to_string();
                    row.spawn((
                        Text::new(summary),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(t.text_secondary),
                    ));
                })
                .id();
            commands.entity(task_parent).add_child(row);
        }
    }
}

// ── Card click handler ───────────────────────────────────────────────────────

pub fn handle_card_clicks(
    mut mc_state: ResMut<MissionControlState>,
    card_q: Query<(&Interaction, &McCardButton), Changed<Interaction>>,
    bridge: Res<WorldBridge>,
    mut activity_heading_q: Query<&mut Text, With<McActivityHeading>>,
    mut task_heading_q: Query<&mut Text, (With<McTaskHeading>, Without<McActivityHeading>)>,
) {
    if !mc_state.open {
        return;
    }
    for (interaction, card_btn) in card_q.iter() {
        if *interaction == Interaction::Pressed {
            if mc_state.selected_agent == Some(card_btn.0) {
                mc_state.selected_agent = None;
            } else {
                mc_state.selected_agent = Some(card_btn.0);
            }

            // Update heading texts
            let agent_name = mc_state.selected_agent.and_then(|id| {
                bridge.registry.read().ok().and_then(|reg| {
                    reg.get(&id).map(|a| a.name.clone())
                })
            });
            for mut text in activity_heading_q.iter_mut() {
                **text = match &agent_name {
                    Some(name) => format!("Recent Activity — {}", name),
                    None => "Recent Activity".to_string(),
                };
            }
            for mut text in task_heading_q.iter_mut() {
                **text = match &agent_name {
                    Some(name) => format!("Tasks — {}", name),
                    None => "Tasks".to_string(),
                };
            }
        }
    }
}

// ── Task popup handler ───────────────────────────────────────────────────────

pub fn handle_task_popup(
    mut commands: Commands,
    mut mc_state: ResMut<MissionControlState>,
    theme: Res<ThemeState>,
    task_row_q: Query<(&Interaction, &McTaskRowButton), Changed<Interaction>>,
    popup_q: Query<Entity, With<McTaskPopup>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Dismiss popup on Escape
    if mc_state.popup_open && keys.just_pressed(KeyCode::Escape) {
        for entity in popup_q.iter() {
            commands.entity(entity).despawn();
        }
        mc_state.popup_open = false;
        return;
    }

    // Open popup on task row click
    if mc_state.popup_open {
        // If popup is open and user clicks a task row, close existing and open new
        let mut clicked = None;
        for (interaction, btn) in task_row_q.iter() {
            if *interaction == Interaction::Pressed {
                clicked = Some(btn.clone());
            }
        }
        if let Some(btn) = clicked {
            for entity in popup_q.iter() {
                commands.entity(entity).despawn();
            }
            spawn_task_popup(&mut commands, &btn, theme.is_dark);
        }
        return;
    }

    for (interaction, btn) in task_row_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.popup_open = true;
            spawn_task_popup(&mut commands, &btn, theme.is_dark);
            return;
        }
    }
}

/// Spawn a label + value field pair inside a dialog.
macro_rules! field_node {
    ($parent:expr, $theme:expr, $label:expr, $value:expr) => {
        $parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|field| {
                field.spawn((
                    Text::new($label),
                    TextFont { font_size: 11.0, ..default() },
                    TextColor($theme.text_muted),
                ));
                field.spawn((
                    Text::new($value),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor($theme.text_primary),
                ));
            });
    };
}

fn spawn_task_popup(commands: &mut Commands, btn: &McTaskRowButton, is_dark: bool) {
    let t = mc_theme(is_dark);
    let task_color = match btn.state.as_str() {
        "completed" => Color::srgb(0.2, 0.8, 0.2),
        "failed" => Color::srgb(1.0, 0.3, 0.3),
        "running" => Color::srgb(1.0, 0.85, 0.0),
        _ => Color::srgb(0.3, 0.7, 1.0),
    };

    commands
        .spawn((
            // Full-screen semi-transparent backdrop
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(100),
            McTaskPopup,
        ))
        .with_children(|backdrop| {
            // Dialog card
            backdrop
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(14.0),
                        width: Val::Px(480.0),
                        max_height: Val::Percent(70.0),
                        overflow: Overflow::scroll_y(),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(t.card_bg),
                    BorderColor::all(t.card_border),
                ))
                .with_children(|dialog| {
                    // Header row: title + close hint
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|header| {
                            header.spawn((
                                Text::new("Task Detail"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(t.title),
                            ));
                            header.spawn((
                                Text::new("press Esc to close"),
                                TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(t.text_muted),
                            ));
                        });

                    // Separator
                    dialog.spawn((
                        Node {
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(t.separator),
                    ));

                    // Task ID
                    field_node!(dialog, &t, "Task ID", &btn.task_id);

                    // Agent
                    field_node!(dialog, &t, "Agent", &btn.agent_name);

                    // State with colored badge
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|field| {
                            field.spawn((
                                Text::new("State"),
                                TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(t.text_muted),
                            ));
                            field
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(8.0),
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Node {
                                            width: Val::Px(10.0),
                                            height: Val::Px(10.0),
                                            border_radius: BorderRadius::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        BackgroundColor(task_color),
                                    ));
                                    row.spawn((
                                        Node {
                                            padding: UiRect::new(
                                                Val::Px(8.0),
                                                Val::Px(8.0),
                                                Val::Px(2.0),
                                                Val::Px(2.0),
                                            ),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::NONE),
                                        BorderColor::all(task_color),
                                    ))
                                    .with_children(|badge| {
                                        badge.spawn((
                                            Text::new(&btn.state),
                                            TextFont {
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(task_color),
                                        ));
                                    });
                                });
                        });

                    // Summary
                    if !btn.summary.is_empty() {
                        field_node!(dialog, &t, "Summary", &btn.summary);
                    }

                    // Timestamps
                    field_node!(dialog, &t, "Submitted", &btn.submitted_at);
                    field_node!(dialog, &t, "Last Updated", &btn.last_updated);
                });
        });
}

// ── Helpers ──────────────────────────────────────────────────────────────────

pub(crate) fn state_color(state: &crate::agent::AgentState) -> Color {
    use crate::agent::AgentState;
    match state {
        AgentState::Working => Color::srgb(0.2, 0.8, 0.2),
        AgentState::Thinking => Color::srgb(1.0, 0.9, 0.0),
        AgentState::Eating => Color::srgb(1.0, 0.6, 0.0),
        AgentState::Playing => Color::srgb(0.8, 0.2, 0.8),
        AgentState::Exercising => Color::srgb(0.0, 0.8, 0.8),
        AgentState::Messaging => Color::srgb(0.0, 0.8, 1.0),
        AgentState::Error => Color::srgb(1.0, 0.0, 0.0),
        AgentState::Walking => Color::srgb(0.9, 0.9, 0.9),
        AgentState::Offline => Color::srgb(0.3, 0.3, 0.3),
        _ => Color::srgb(0.5, 0.5, 0.5),
    }
}
