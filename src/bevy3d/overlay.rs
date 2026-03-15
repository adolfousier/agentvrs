use crate::agent::{AgentId, AgentState};
use crate::bevy3d::agents::AgentMarker;
use crate::bevy3d::bridge::WorldBridge;
use crate::bevy3d::camera::MainCamera;
use crate::bevy3d::interaction::SelectedAgent;
use bevy::prelude::*;

// ── Marker components for UI nodes ──────────────────────────────────────────

#[derive(Component)]
pub struct SidebarRoot;

#[derive(Component)]
pub struct AgentListContainer;

#[derive(Component)]
pub struct AgentListEntry(pub AgentId);

#[derive(Component)]
pub struct DetailTitle;

#[derive(Component)]
pub struct DetailInfo;

#[derive(Component)]
pub struct StatusBarTick;

#[derive(Component)]
pub struct StatusBarAgents;

#[derive(Component)]
pub struct StatusBarZoom;

#[derive(Component)]
pub struct AgentLabel {
    pub agent_id: AgentId,
}

// ── Startup: build the full UI layout ───────────────────────────────────────

pub fn setup_ui(mut commands: Commands) {
    // ── Status bar (bottom) ─────────────────────────────────────────────
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                height: Val::Px(28.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                align_items: AlignItems::Center,
                column_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.10)),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("tick: 0"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarTick,
            ));
            bar.spawn((
                Text::new("agents: 0"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarAgents,
            ));
            bar.spawn((
                Text::new("zoom: 100%"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarZoom,
            ));
            bar.spawn((
                Text::new("r:rotate  scroll:zoom  shift+drag:pan  click:select  h:sidebar"),
                TextFont { font_size: 11.0, ..default() },
                TextColor(Color::srgb(0.45, 0.45, 0.45)),
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
            ));
        });

    // ── Sidebar (right side) ────────────────────────────────────────────
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(28.0),
                width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.13, 0.13, 0.15)),
            SidebarRoot,
        ))
        .with_children(|sidebar| {
            // ── Header ──────────────────────────────────────────────
            sidebar.spawn((
                Text::new("Agents"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
            ));

            // ── Agent list (scrollable area) ────────────────────────
            sidebar
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(6.0)),
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    AgentListContainer,
                ))
                .with_children(|_| {});

            // ── Separator ───────────────────────────────────────────
            sidebar.spawn((
                Node {
                    height: Val::Px(1.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.25, 0.25, 0.28)),
            ));

            // ── Detail panel ────────────────────────────────────────
            sidebar
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(14.0)),
                    row_gap: Val::Px(8.0),
                    min_height: Val::Px(160.0),
                    ..default()
                })
                .with_children(|detail| {
                    detail.spawn((
                        Text::new("No agent selected"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.85, 0.85, 0.85)),
                        DetailTitle,
                    ));
                    detail.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.65, 0.65, 0.65)),
                        DetailInfo,
                    ));
                });
        });
}

// ── Per-frame: update sidebar content ───────────────────────────────────────

pub fn update_sidebar(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    selected: Res<SelectedAgent>,
    list_q: Query<(Entity, &Children), With<AgentListContainer>>,
    mut title_q: Query<&mut Text, (With<DetailTitle>, Without<DetailInfo>)>,
    mut info_q: Query<&mut Text, (With<DetailInfo>, Without<DetailTitle>)>,
    entry_q: Query<Entity, With<AgentListEntry>>,
) {
    let registry = bridge.registry.read().unwrap();

    // ── Rebuild agent list ──────────────────────────────────────────────
    // Remove old entries
    for entity in entry_q.iter() {
        commands.entity(entity).despawn();
    }

    // Add current agents
    if let Ok((list_entity, _)) = list_q.get_single() {
        for agent in registry.agents() {
            let is_selected = selected.agent_id == Some(agent.id);
            let dot_color = state_color(&agent.state);
            let name_color = if is_selected {
                Color::srgb(0.4, 0.9, 1.0)
            } else {
                Color::srgb(0.85, 0.85, 0.85)
            };

            let row_bg = if is_selected {
                Color::srgba(0.2, 0.5, 0.6, 0.25)
            } else {
                Color::NONE
            };

            let row = commands
                .spawn((
                    Node {
                        padding: UiRect::new(
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(5.0),
                            Val::Px(5.0),
                        ),
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    AgentListEntry(agent.id),
                ))
                .with_children(|row| {
                    // Status dot
                    row.spawn((
                        Node {
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(dot_color),
                        BorderRadius::all(Val::Px(4.0)),
                    ));
                    // Name
                    row.spawn((
                        Text::new(&agent.name),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(name_color),
                    ));
                    // State label
                    row.spawn((
                        Text::new(format!("({})", agent.state.label())),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                })
                .id();

            commands.entity(list_entity).add_child(row);
        }
    }

    // ── Update detail panel ─────────────────────────────────────────────
    if let Some(agent_id) = selected.agent_id {
        if let Some(agent) = registry.get(&agent_id) {
            for mut text in title_q.iter_mut() {
                **text = agent.name.clone();
            }
            let info_text = format!(
                "State: {}\nPosition: ({}, {})\nKind: {:?}\nTasks: {}{}",
                agent.state.label(),
                agent.position.x,
                agent.position.y,
                agent.kind,
                agent.task_count,
                agent
                    .speech
                    .as_ref()
                    .map(|s| format!("\n\nSays: \"{}\"", s))
                    .unwrap_or_default(),
            );
            for mut text in info_q.iter_mut() {
                **text = info_text.clone();
            }
        }
    } else {
        for mut text in title_q.iter_mut() {
            **text = "No agent selected".to_string();
        }
        for mut text in info_q.iter_mut() {
            **text = String::new();
        }
    }
}

// ── Per-frame: update status bar ────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn update_status_bar(
    bridge: Res<WorldBridge>,
    cam_state: Res<super::camera::CameraState>,
    mut tick_q: Query<&mut Text, (With<StatusBarTick>, Without<StatusBarAgents>, Without<StatusBarZoom>)>,
    mut agents_q: Query<&mut Text, (With<StatusBarAgents>, Without<StatusBarTick>, Without<StatusBarZoom>)>,
    mut zoom_q: Query<&mut Text, (With<StatusBarZoom>, Without<StatusBarTick>, Without<StatusBarAgents>)>,
) {
    let count = bridge.registry.read().unwrap().count();

    for mut text in tick_q.iter_mut() {
        // We don't have tick count easily accessible; show agent count instead
        **text = format!("agents: {}", count);
    }
    for mut text in agents_q.iter_mut() {
        let (w, h) = bridge.grid.read().unwrap().bounds();
        **text = format!("world: {}x{}", w, h);
    }
    for mut text in zoom_q.iter_mut() {
        **text = format!("zoom: {:.0}%", (12.0 / cam_state.zoom) * 100.0);
    }
}

// ── Per-frame: floating name labels above agents ────────────────────────────

pub fn update_agent_labels(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    agent_q: Query<(&GlobalTransform, &AgentMarker)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut label_q: Query<(Entity, &AgentLabel, &mut Node, &mut Text)>,
    selected: Res<SelectedAgent>,
) {
    let Ok((camera, cam_gt)) = camera_q.get_single() else {
        return;
    };

    let registry = bridge.registry.read().unwrap();

    let mut existing_labels: std::collections::HashMap<AgentId, Entity> =
        label_q.iter().map(|(e, l, _, _)| (l.agent_id, e)).collect();

    for (agent_gt, marker) in agent_q.iter() {
        let agent_pos = agent_gt.translation() + Vec3::Y * 0.65;

        let Ok(viewport_pos) = camera.world_to_viewport(cam_gt, agent_pos) else {
            continue;
        };

        let agent = match registry.get(&marker.agent_id) {
            Some(a) => a,
            None => continue,
        };

        let is_selected = selected.agent_id == Some(marker.agent_id);
        let display_name = if agent.name.len() > 14 {
            format!("{}...", &agent.name[..11])
        } else {
            agent.name.clone()
        };

        let dot = state_dot(&agent.state);
        let label_text = format!("{} {}", dot, display_name);

        let text_color = if is_selected {
            Color::srgb(0.4, 0.9, 1.0)
        } else {
            Color::srgb(0.95, 0.95, 0.95)
        };

        if let Some(label_entity) = existing_labels.remove(&marker.agent_id) {
            if let Ok((_, _, mut node, mut text)) = label_q.get_mut(label_entity) {
                node.left = Val::Px(viewport_pos.x - 50.0);
                node.top = Val::Px(viewport_pos.y - 20.0);
                **text = label_text;
            }
        } else {
            commands.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(viewport_pos.x - 50.0),
                    top: Val::Px(viewport_pos.y - 20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.80)),
                BorderRadius::all(Val::Px(8.0)),
                AgentLabel {
                    agent_id: marker.agent_id,
                },
            ));
        }
    }

    // Remove stale labels
    for (_, entity) in existing_labels {
        commands.entity(entity).despawn();
    }
}

// ── Toggle sidebar with H key ───────────────────────────────────────────────

pub fn toggle_sidebar(
    keys: Res<ButtonInput<KeyCode>>,
    mut sidebar_q: Query<&mut Visibility, With<SidebarRoot>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        for mut vis in sidebar_q.iter_mut() {
            *vis = match *vis {
                Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn state_color(state: &AgentState) -> Color {
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

fn state_dot(state: &AgentState) -> &'static str {
    match state {
        AgentState::Working => "\u{1F7E2}",
        AgentState::Thinking => "\u{1F7E1}",
        AgentState::Eating => "\u{1F7E0}",
        AgentState::Playing => "\u{1F7E3}",
        AgentState::Exercising => "\u{1F535}",
        AgentState::Messaging => "\u{1F535}",
        AgentState::Error => "\u{1F534}",
        AgentState::Walking => "\u{26AA}",
        _ => "\u{26AB}",
    }
}
