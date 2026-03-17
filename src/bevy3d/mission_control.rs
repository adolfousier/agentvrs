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

/// Resource: mission control panel open/closed state.
#[derive(Resource, Default)]
pub struct MissionControlState {
    pub open: bool,
}

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
            badge_bg: Color::srgb(0.15, 0.30, 0.55),
            badge_text: Color::srgb(0.55, 0.75, 1.0),
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
            badge_bg: Color::srgb(0.87, 0.92, 1.0),
            badge_text: Color::srgb(0.08, 0.38, 0.78),
            separator: Color::srgb(0.85, 0.86, 0.89),
        }
    }
}

// ── Toggle with M key ────────────────────────────────────────────────────────

pub fn toggle_mission_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut mc_state: ResMut<MissionControlState>,
    mut mc_root_q: Query<&mut Visibility, With<MissionControlRoot>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        mc_state.open = !mc_state.open;
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
                overflow: Overflow::clip(),
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
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|content| {
                // Left column: Agent cards (grid layout)
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(45.0),
                        row_gap: Val::Px(12.0),
                        overflow: Overflow::clip(),
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
                                overflow: Overflow::clip(),
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
                        overflow: Overflow::clip(),
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
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(0.0),
                                height: Val::Percent(50.0),
                                overflow: Overflow::clip(),
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
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(0.0),
                                flex_grow: 1.0,
                                overflow: Overflow::clip(),
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

        for agent in registry.agents() {
            let dot_color = state_color(&agent.state);
            let kind_label = match &agent.kind {
                crate::agent::AgentKind::Local => "local",
                crate::agent::AgentKind::External { .. } => "external",
                crate::agent::AgentKind::OpenCrabs { .. } => "opencrabs",
            };

            let card = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(14.0)),
                        row_gap: Val::Px(8.0),
                        width: Val::Px(220.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(t.card_bg),
                    BorderColor::all(t.card_border),
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
                        // Name with status dot
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
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(t.text_primary),
                            ));
                        });
                        // Kind badge
                        top.spawn((
                            Node {
                                padding: UiRect::new(
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                    Val::Px(2.0),
                                    Val::Px(2.0),
                                ),
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
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(t.text_secondary),
                            ));
                        });
                    });

                    // State label
                    card.spawn((
                        Text::new(agent.state.label()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(t.text_secondary),
                    ));

                    // Separator
                    card.spawn((
                        Node {
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(t.separator),
                    ));

                    // Bottom row: position + inbox
                    card.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|bottom| {
                        bottom.spawn((
                            Text::new(format!("({}, {})", agent.position.x, agent.position.y)),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(t.text_muted),
                        ));
                        if !agent.inbox.is_empty() {
                            bottom
                                .spawn((
                                    Node {
                                        padding: UiRect::new(
                                            Val::Px(7.0),
                                            Val::Px(7.0),
                                            Val::Px(2.0),
                                            Val::Px(2.0),
                                        ),
                                        border_radius: BorderRadius::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BackgroundColor(t.badge_bg),
                                ))
                                .with_children(|badge| {
                                    badge.spawn((
                                        Text::new(format!("{} msg", agent.inbox.len())),
                                        TextFont {
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(t.badge_text),
                                    ));
                                });
                        }
                    });
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
            for agent in registry.agents() {
                if let Ok(entries) = db.load_activity(&agent.id, 10) {
                    for entry in entries {
                        all_activity.push((agent.name.clone(), entry));
                    }
                }
            }
        }
        all_activity.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        all_activity.truncate(20);

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
                    let detail = if entry.detail.len() > 60 {
                        format!("{}...", &entry.detail[..57])
                    } else {
                        entry.detail.clone()
                    };
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
            for agent in registry.agents() {
                if let Ok(tasks) = db.load_tasks(&agent.id, 10) {
                    for task in tasks {
                        all_tasks.push((agent.name.clone(), task));
                    }
                }
            }
        }
        all_tasks.sort_by(|a, b| b.1.last_updated.cmp(&a.1.last_updated));
        all_tasks.truncate(20);

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
                    let summary = task.response_summary.as_deref().unwrap_or(&task.task_id);
                    let summary = if summary.len() > 50 {
                        format!("{}...", &summary[..47])
                    } else {
                        summary.to_string()
                    };
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
