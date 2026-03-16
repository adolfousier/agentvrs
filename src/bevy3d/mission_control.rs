use crate::bevy3d::bridge::WorldBridge;
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

/// Resource: mission control panel open/closed state.
#[derive(Resource, Default)]
pub struct MissionControlState {
    pub open: bool,
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
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(16.0),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.06, 0.08, 0.95)),
            Visibility::Hidden,
            MissionControlRoot,
        ))
        .with_children(|root| {
            // ── Header row ───────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("Mission Control"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.95, 0.95)),
                ));
                header.spawn((
                    Text::new("press M to close"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.4, 0.4, 0.4)),
                ));
            });

            // ── Content: two columns ─────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                column_gap: Val::Px(16.0),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|content| {
                // Left column: Agent cards
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(40.0),
                        row_gap: Val::Px(8.0),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|left| {
                        left.spawn((
                            Text::new("Agents"),
                            TextFont {
                                font_size: 15.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.8, 1.0)),
                        ));
                        left.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.0),
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
                        row_gap: Val::Px(12.0),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|right| {
                        // Activity section
                        right.spawn((
                            Text::new("Recent Activity"),
                            TextFont {
                                font_size: 15.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.8, 1.0)),
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(3.0),
                                height: Val::Percent(50.0),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            McActivityFeed,
                        ));

                        // Tasks section
                        right.spawn((
                            Text::new("Tasks"),
                            TextFont {
                                font_size: 15.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.8, 1.0)),
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(3.0),
                                flex_grow: 1.0,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            McTaskList,
                        ));
                    });
            });
        });
}

// ── Per-frame update: populate agent cards, activity feed, tasks ──────────────

#[allow(clippy::too_many_arguments)]
pub fn update_mission_control(
    mut commands: Commands,
    mc_state: Res<MissionControlState>,
    bridge: Res<WorldBridge>,
    card_q: Query<Entity, With<McAgentCard>>,
    feed_q: Query<Entity, With<McActivityFeed>>,
    task_q: Query<Entity, With<McTaskList>>,
    child_q: Query<Entity, With<McChild>>,
) {
    if !mc_state.open {
        return;
    }

    // Remove previous frame's children
    for entity in child_q.iter() {
        commands.entity(entity).despawn();
    }

    let registry = bridge.registry.read().unwrap();

    // ── Agent cards ──────────────────────────────────────────────
    if let Ok(card_parent) = card_q.single() {
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
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::new(
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        column_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.12, 0.12, 0.15, 0.8)),
                    McChild,
                ))
                .with_children(|card| {
                    // Status dot
                    card.spawn((
                        Node {
                            width: Val::Px(10.0),
                            height: Val::Px(10.0),
                            border_radius: BorderRadius::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(dot_color),
                    ));
                    // Agent info column
                    card.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.0),
                        ..default()
                    })
                    .with_children(|info| {
                        info.spawn((
                            Text::new(&agent.name),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                        info.spawn((
                            Text::new(format!(
                                "{} · {} · ({},{})",
                                agent.state.label(),
                                kind_label,
                                agent.position.x,
                                agent.position.y
                            )),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        ));
                    });
                    // Inbox count badge
                    if !agent.inbox.is_empty() {
                        card.spawn((
                            Node {
                                padding: UiRect::new(
                                    Val::Px(6.0),
                                    Val::Px(6.0),
                                    Val::Px(2.0),
                                    Val::Px(2.0),
                                ),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.5, 0.8)),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                Text::new(format!("{}", agent.inbox.len())),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                            ));
                        });
                    }
                })
                .id();
            commands.entity(card_parent).add_child(card);
        }
    }

    // ── Activity feed (from DB) ──────────────────────────────────
    if let Ok(feed_parent) = feed_q.single()
        && let Ok(db) = bridge.db.lock()
    {
        let mut all_activity: Vec<(String, crate::api::observability::ActivityEntry)> = Vec::new();
        for agent in registry.agents() {
            if let Ok(entries) = db.load_activity(&agent.id, 10) {
                for entry in entries {
                    all_activity.push((agent.name.clone(), entry));
                }
            }
        }
        all_activity.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        all_activity.truncate(20);

        for (agent_name, entry) in &all_activity {
            let time_ago = (chrono::Utc::now() - entry.timestamp).num_seconds();
            let time_str = if time_ago < 60 {
                format!("{}s ago", time_ago)
            } else if time_ago < 3600 {
                format!("{}m ago", time_ago / 60)
            } else {
                format!("{}h ago", time_ago / 3600)
            };

            let row = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(&time_str),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.4, 0.4, 0.4)),
                        Node {
                            min_width: Val::Px(50.0),
                            ..default()
                        },
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.8, 1.0)),
                        Node {
                            min_width: Val::Px(80.0),
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
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.55, 0.55, 0.55)),
                    ));
                })
                .id();
            commands.entity(feed_parent).add_child(row);
        }

        if all_activity.is_empty() {
            let empty = commands
                .spawn((
                    Text::new("No activity recorded yet"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.35, 0.35, 0.35)),
                    McChild,
                ))
                .id();
            commands.entity(feed_parent).add_child(empty);
        }
    }

    // ── Task list (from DB) ──────────────────────────────────────
    if let Ok(task_parent) = task_q.single() {
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

        for (agent_name, task) in &all_tasks {
            let task_state_color = match task.state.as_str() {
                "completed" => Color::srgb(0.2, 0.8, 0.2),
                "failed" => Color::srgb(1.0, 0.3, 0.3),
                "submitted" | "running" => Color::srgb(0.3, 0.7, 1.0),
                _ => Color::srgb(0.5, 0.5, 0.5),
            };

            let row = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: Val::Px(6.0),
                            height: Val::Px(6.0),
                            border_radius: BorderRadius::all(Val::Px(3.0)),
                            ..default()
                        },
                        BackgroundColor(task_state_color),
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.8, 1.0)),
                        Node {
                            min_width: Val::Px(80.0),
                            ..default()
                        },
                    ));
                    let summary = task.response_summary.as_deref().unwrap_or(&task.task_id);
                    let summary = if summary.len() > 50 {
                        format!("{}...", &summary[..47])
                    } else {
                        summary.to_string()
                    };
                    row.spawn((
                        Text::new(format!("{} — {}", task.state, summary)),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.55, 0.55, 0.55)),
                    ));
                })
                .id();
            commands.entity(task_parent).add_child(row);
        }

        if all_tasks.is_empty() {
            let empty = commands
                .spawn((
                    Text::new("No tasks submitted yet"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.35, 0.35, 0.35)),
                    McChild,
                ))
                .id();
            commands.entity(task_parent).add_child(empty);
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn state_color(state: &crate::agent::AgentState) -> Color {
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
