use crate::agent::{AgentMessage, AgentState};
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
pub struct AgentListEntry;

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
pub struct AgentLabel;

#[derive(Component)]
pub struct MessageInputText;

#[derive(Component)]
pub struct StatusBarRoot;

#[derive(Component)]
pub struct MessageInputBox;

#[derive(Component)]
pub struct SidebarSeparator;

#[derive(Component)]
pub struct SpeechBubble;

#[derive(Component)]
pub struct DetailPanel;

#[derive(Component)]
pub struct SidebarResizeHandle;

/// Resource: tracks text being typed in the message input.
#[derive(Resource, Default)]
pub struct MessageInputState {
    pub text: String,
    pub active: bool,
}

/// Resource: sidebar layout state with drag-resize support.
#[derive(Resource)]
pub struct SidebarState {
    pub width: f32,
    pub detail_height: f32,
    pub dragging_width: bool,
    pub dragging_detail: bool,
    pub drag_start: f32,
    pub drag_start_value: f32,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            width: 280.0,
            detail_height: 180.0,
            dragging_width: false,
            dragging_detail: false,
            drag_start: 0.0,
            drag_start_value: 0.0,
        }
    }
}

// ── Startup: build the full UI layout ───────────────────────────────────────

pub fn setup_ui(mut commands: Commands, sidebar_state: Res<SidebarState>) {
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
            ZIndex(100),
            StatusBarRoot,
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("tick: 0"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarTick,
            ));
            bar.spawn((
                Text::new("agents: 0"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarAgents,
            ));
            bar.spawn((
                Text::new("zoom: 100%"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusBarZoom,
            ));
            bar.spawn((
                Text::new("r:rotate  scroll:zoom  drag:pan  click:select  h:sidebar  m:mission  esc:deselect"),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
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
                width: Val::Px(sidebar_state.width),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgb(0.13, 0.13, 0.15)),
            ZIndex(100),
            SidebarRoot,
        ))
        .with_children(|sidebar| {
            // Resize handle (left edge — drag to resize width)
            sidebar.spawn((
                Node {
                    width: Val::Px(4.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                SidebarResizeHandle,
                Interaction::default(),
            ));

            // Main sidebar column
            sidebar
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::clip(),
                    ..default()
                })
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
                            padding: UiRect::new(
                                Val::Px(14.0),
                                Val::Px(14.0),
                                Val::Px(14.0),
                                Val::Px(8.0),
                            ),
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

                    // ── Separator (drag to resize detail panel) ──────────────
                    sidebar
                        .spawn((
                            Node {
                                height: Val::Px(8.0),
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            SidebarSeparator,
                            Interaction::default(),
                        ))
                        .with_children(|sep| {
                            // Thin visible line inside the hit area
                            sep.spawn((
                                Node {
                                    height: Val::Px(1.0),
                                    width: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.25, 0.25, 0.28)),
                            ));
                        });

                    // ── Detail panel ────────────────────────────────────────
                    sidebar
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(14.0)),
                                row_gap: Val::Px(8.0),
                                height: Val::Px(sidebar_state.detail_height),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            DetailPanel,
                        ))
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

                            // ── Message input ──────────────────────────────
                            detail.spawn((
                                Text::new("Send message:"),
                                TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            ));
                            detail
                                .spawn((
                                    Node {
                                        padding: UiRect::all(Val::Px(6.0)),
                                        min_height: Val::Px(24.0),
                                        width: Val::Percent(100.0),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.08, 0.08, 0.10)),
                                    MessageInputBox,
                                ))
                                .with_children(|input_box| {
                                    input_box.spawn((
                                        Text::new("Type and press Enter..."),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.35, 0.35, 0.35)),
                                        MessageInputText,
                                    ));
                                });
                        });
                });
        });
}

// ── Per-frame: update sidebar content ───────────────────────────────────────

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update_sidebar(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    selected: Res<SelectedAgent>,
    theme: Res<super::runner::ThemeState>,
    list_q: Query<Entity, With<AgentListContainer>>,
    mut title_q: Query<(&mut Text, &mut TextColor), (With<DetailTitle>, Without<DetailInfo>)>,
    mut info_q: Query<(&mut Text, &mut TextColor), (With<DetailInfo>, Without<DetailTitle>)>,
    entry_q: Query<Entity, With<AgentListEntry>>,
) {
    let registry = bridge.registry.read().unwrap();

    // ── Rebuild agent list ──────────────────────────────────────────────
    // Remove old entries
    for entity in entry_q.iter() {
        commands.entity(entity).despawn();
    }

    // Add current agents
    if let Ok(list_entity) = list_q.single() {
        for agent in registry.agents() {
            let is_selected = selected.agent_id == Some(agent.id);
            let dot_color = state_color(&agent.state);
            let name_color = if is_selected {
                Color::srgb(0.4, 0.9, 1.0)
            } else if theme.is_dark {
                Color::srgb(0.85, 0.85, 0.85)
            } else {
                Color::srgb(0.15, 0.15, 0.15)
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
                    AgentListEntry,
                ))
                .with_children(|row| {
                    // Status dot
                    row.spawn((
                        Node {
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(dot_color),
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
                    let state_text_color = if theme.is_dark {
                        Color::srgb(0.5, 0.5, 0.5)
                    } else {
                        Color::srgb(0.4, 0.4, 0.4)
                    };
                    row.spawn((
                        Text::new(format!("({})", agent.state.label())),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(state_text_color),
                    ));
                })
                .id();

            commands.entity(list_entity).add_child(row);
        }
    }

    // ── Update detail panel ─────────────────────────────────────────────
    let title_color = if theme.is_dark {
        Color::srgb(0.85, 0.85, 0.85)
    } else {
        Color::srgb(0.15, 0.15, 0.15)
    };
    let info_color = if theme.is_dark {
        Color::srgb(0.65, 0.65, 0.65)
    } else {
        Color::srgb(0.3, 0.3, 0.3)
    };

    if let Some(agent_id) = selected.agent_id {
        if let Some(agent) = registry.get(&agent_id) {
            for (mut text, mut color) in title_q.iter_mut() {
                **text = agent.name.clone();
                color.0 = title_color;
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
            for (mut text, mut color) in info_q.iter_mut() {
                **text = info_text.clone();
                color.0 = info_color;
            }
        }
    } else {
        for (mut text, mut color) in title_q.iter_mut() {
            **text = "No agent selected".to_string();
            color.0 = title_color;
        }
        for (mut text, mut color) in info_q.iter_mut() {
            **text = String::new();
            color.0 = info_color;
        }
    }
}

// ── Per-frame: update status bar ────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn update_status_bar(
    bridge: Res<WorldBridge>,
    cam_state: Res<super::camera::CameraState>,
    theme: Res<super::runner::ThemeState>,
    mut tick_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<StatusBarTick>,
            Without<StatusBarAgents>,
            Without<StatusBarZoom>,
        ),
    >,
    mut agents_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<StatusBarAgents>,
            Without<StatusBarTick>,
            Without<StatusBarZoom>,
        ),
    >,
    mut zoom_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<StatusBarZoom>,
            Without<StatusBarTick>,
            Without<StatusBarAgents>,
        ),
    >,
) {
    let count = bridge.registry.read().unwrap().count();
    let text_color = if theme.is_dark {
        Color::srgb(0.7, 0.7, 0.7)
    } else {
        Color::srgb(0.25, 0.25, 0.25)
    };

    for (mut text, mut color) in tick_q.iter_mut() {
        **text = format!("agents: {}", count);
        color.0 = text_color;
    }
    for (mut text, mut color) in agents_q.iter_mut() {
        let (w, h) = bridge.grid.read().unwrap().bounds();
        **text = format!("world: {}x{}", w, h);
        color.0 = text_color;
    }
    for (mut text, mut color) in zoom_q.iter_mut() {
        **text = format!("zoom: {:.0}%", (12.0 / cam_state.zoom) * 100.0);
        color.0 = text_color;
    }
}

// ── Per-frame: floating name labels above agents ────────────────────────────

#[allow(clippy::too_many_arguments)]
pub fn update_agent_labels(
    mut commands: Commands,
    bridge: Res<WorldBridge>,
    theme: Res<super::runner::ThemeState>,
    mc_state: Res<super::mission_control::MissionControlState>,
    agent_q: Query<(&GlobalTransform, &AgentMarker)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    label_q: Query<Entity, With<AgentLabel>>,
    bubble_q: Query<Entity, With<SpeechBubble>>,
    selected: Res<SelectedAgent>,
) {
    // Remove all existing labels and speech bubbles (recreated each frame)
    for entity in label_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in bubble_q.iter() {
        commands.entity(entity).despawn();
    }

    // Don't render labels when Mission Control is open
    if mc_state.open {
        return;
    }

    let Ok((camera, cam_gt)) = camera_q.single() else {
        return;
    };

    let registry = bridge.registry.read().unwrap();

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

        let text_color = if is_selected {
            Color::srgb(0.4, 0.9, 1.0)
        } else if theme.is_dark {
            Color::srgb(0.95, 0.95, 0.95)
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        };

        let dot_color = state_color(&agent.state);

        // Name label
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(viewport_pos.x - 50.0),
                    top: Val::Px(viewport_pos.y - 20.0),
                    padding: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(4.0), Val::Px(4.0)),
                    column_gap: Val::Px(6.0),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(if theme.is_dark {
                    Color::srgba(0.1, 0.1, 0.12, 0.80)
                } else {
                    Color::srgba(0.9, 0.9, 0.92, 0.80)
                }),
                AgentLabel,
            ))
            .with_children(|label| {
                // Colored status dot
                label.spawn((
                    Node {
                        width: Val::Px(7.0),
                        height: Val::Px(7.0),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(dot_color),
                ));
                // Agent name
                label.spawn((
                    Text::new(display_name),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(text_color),
                ));
            });

        // Speech bubble (shown above name label when agent has speech)
        if let Some(speech) = &agent.speech {
            let bubble_text = if speech.len() > 40 {
                format!("{}...", &speech[..37])
            } else {
                speech.clone()
            };
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(viewport_pos.x - 60.0),
                        top: Val::Px(viewport_pos.y - 50.0),
                        padding: UiRect::new(
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(5.0),
                            Val::Px(5.0),
                        ),
                        max_width: Val::Px(220.0),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(if theme.is_dark {
                        Color::srgba(0.15, 0.35, 0.55, 0.92)
                    } else {
                        Color::srgba(0.20, 0.50, 0.80, 0.92)
                    }),
                    SpeechBubble,
                ))
                .with_children(|bubble| {
                    bubble.spawn((
                        Text::new(bubble_text),
                        TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });
        }
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

// ── Message input handling ───────────────────────────────────────────────────

pub fn handle_message_input(
    mut char_evr: MessageReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut input_state: ResMut<MessageInputState>,
    selected: Res<SelectedAgent>,
    bridge: Res<WorldBridge>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<MessageInputText>>,
) {
    // Only active when an agent is selected
    if selected.agent_id.is_none() {
        input_state.active = false;
        input_state.text.clear();
        return;
    }
    input_state.active = true;

    // Handle character input
    for event in char_evr.read() {
        if event.state != bevy::input::ButtonState::Pressed {
            continue;
        }
        match &event.logical_key {
            bevy::input::keyboard::Key::Character(s) => {
                for ch in s.chars() {
                    if !ch.is_control() {
                        input_state.text.push(ch);
                    }
                }
            }
            bevy::input::keyboard::Key::Space => {
                input_state.text.push(' ');
            }
            bevy::input::keyboard::Key::Backspace => {
                input_state.text.pop();
            }
            _ => {}
        }
    }

    // Enter — send message (set speech bubble + push to inbox)
    if keys.just_pressed(KeyCode::Enter) && input_state.active && !input_state.text.is_empty() {
        if let Some(agent_id) = selected.agent_id {
            let mut registry = bridge.registry.write().unwrap();
            if let Some(agent) = registry.get_mut(&agent_id) {
                // Show as speech bubble in the 3D world
                agent.say(&input_state.text);
                // Also deliver to the agent's inbox so external agents can retrieve it
                let system_id = crate::agent::AgentId::default();
                let msg = AgentMessage::new(system_id, agent_id, &input_state.text);
                // Persist to database
                if let Ok(db) = bridge.db.lock() {
                    let _ = db.save_message(&msg);
                }
                agent.inbox.push_back(msg);
                agent.set_state(AgentState::Messaging);
                agent.anim.activity_ticks = 0;
            }
        }
        input_state.text.clear();
    }

    // Update displayed text
    for (mut text, mut color) in text_q.iter_mut() {
        if input_state.text.is_empty() {
            **text = "Type and press Enter...".to_string();
            *color = TextColor(Color::srgb(0.35, 0.35, 0.35));
        } else {
            **text = input_state.text.clone();
            *color = TextColor(Color::srgb(0.9, 0.9, 0.9));
        }
    }
}

// ── Theme-aware UI color updates ─────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn update_ui_theme(
    theme: Res<super::runner::ThemeState>,
    mut sidebar_q: Query<
        &mut BackgroundColor,
        (
            With<SidebarRoot>,
            Without<StatusBarRoot>,
            Without<SidebarSeparator>,
            Without<MessageInputBox>,
            Without<AgentLabel>,
        ),
    >,
    mut statusbar_q: Query<
        &mut BackgroundColor,
        (
            With<StatusBarRoot>,
            Without<SidebarRoot>,
            Without<SidebarSeparator>,
            Without<MessageInputBox>,
            Without<AgentLabel>,
        ),
    >,
    mut separator_q: Query<
        &mut BackgroundColor,
        (
            With<SidebarSeparator>,
            Without<SidebarRoot>,
            Without<StatusBarRoot>,
            Without<MessageInputBox>,
            Without<AgentLabel>,
        ),
    >,
    mut input_q: Query<
        &mut BackgroundColor,
        (
            With<MessageInputBox>,
            Without<SidebarRoot>,
            Without<StatusBarRoot>,
            Without<SidebarSeparator>,
            Without<AgentLabel>,
        ),
    >,
    mut label_q: Query<
        &mut BackgroundColor,
        (
            With<AgentLabel>,
            Without<SidebarRoot>,
            Without<StatusBarRoot>,
            Without<SidebarSeparator>,
            Without<MessageInputBox>,
        ),
    >,
) {
    if !theme.is_changed() {
        return;
    }

    let is_dark = theme.is_dark;

    let sidebar_bg = if is_dark {
        Color::srgb(0.13, 0.13, 0.15)
    } else {
        Color::srgb(0.92, 0.92, 0.94)
    };
    let bar_bg = if is_dark {
        Color::srgb(0.08, 0.08, 0.10)
    } else {
        Color::srgb(0.82, 0.82, 0.85)
    };
    let sep_bg = if is_dark {
        Color::srgb(0.25, 0.25, 0.28)
    } else {
        Color::srgb(0.75, 0.75, 0.78)
    };
    let input_bg = if is_dark {
        Color::srgb(0.08, 0.08, 0.10)
    } else {
        Color::srgb(0.85, 0.85, 0.88)
    };
    let label_bg = if is_dark {
        Color::srgba(0.1, 0.1, 0.12, 0.80)
    } else {
        Color::srgba(0.9, 0.9, 0.92, 0.80)
    };

    for mut bg in sidebar_q.iter_mut() {
        bg.0 = sidebar_bg;
    }
    for mut bg in statusbar_q.iter_mut() {
        bg.0 = bar_bg;
    }
    for mut bg in separator_q.iter_mut() {
        bg.0 = sep_bg;
    }
    for mut bg in input_q.iter_mut() {
        bg.0 = input_bg;
    }
    for mut bg in label_q.iter_mut() {
        bg.0 = label_bg;
    }
}

// ── Sidebar resize via drag handles ─────────────────────────────────────────

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn sidebar_resize(
    mut commands: Commands,
    mut sidebar_state: ResMut<SidebarState>,
    handle_q: Query<&Interaction, (With<SidebarResizeHandle>, Changed<Interaction>)>,
    sep_q: Query<&Interaction, (With<SidebarSeparator>, Changed<Interaction>)>,
    mut handle_bg_q: Query<
        (&Interaction, &mut BackgroundColor),
        (With<SidebarResizeHandle>, Without<SidebarSeparator>),
    >,
    mut sep_bg_q: Query<
        (&Interaction, &mut BackgroundColor),
        (With<SidebarSeparator>, Without<SidebarResizeHandle>),
    >,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    windows: Query<(Entity, &Window)>,
    mut sidebar_q: Query<&mut Node, (With<SidebarRoot>, Without<DetailPanel>)>,
    mut detail_q: Query<&mut Node, (With<DetailPanel>, Without<SidebarRoot>)>,
) {
    // Track whether any handle is hovered/active for cursor changes
    let mut wants_ew_cursor = false;
    let mut wants_ns_cursor = false;

    // Visual hover feedback on resize handle (invisible by default)
    for (interaction, mut bg) in handle_bg_q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered => {
                wants_ew_cursor = true;
                Color::srgba(0.4, 0.7, 1.0, 0.35)
            }
            Interaction::Pressed => {
                wants_ew_cursor = true;
                Color::srgba(0.4, 0.7, 1.0, 0.5)
            }
            Interaction::None => Color::NONE,
        };
    }
    // Visual hover feedback on separator (transparent by default, highlight on hover)
    for (interaction, mut bg) in sep_bg_q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered => {
                wants_ns_cursor = true;
                Color::srgba(0.4, 0.7, 1.0, 0.3)
            }
            Interaction::Pressed => {
                wants_ns_cursor = true;
                Color::srgba(0.4, 0.7, 1.0, 0.5)
            }
            Interaction::None => Color::NONE,
        };
    }

    // Update cursor icon based on hover/drag state
    if sidebar_state.dragging_width {
        wants_ew_cursor = true;
    }
    if sidebar_state.dragging_detail {
        wants_ns_cursor = true;
    }

    for (entity, _) in windows.iter() {
        if wants_ew_cursor {
            commands
                .entity(entity)
                .insert(bevy::window::CursorIcon::from(
                    bevy::window::SystemCursorIcon::ColResize,
                ));
        } else if wants_ns_cursor {
            commands
                .entity(entity)
                .insert(bevy::window::CursorIcon::from(
                    bevy::window::SystemCursorIcon::RowResize,
                ));
        } else {
            commands
                .entity(entity)
                .insert(bevy::window::CursorIcon::from(
                    bevy::window::SystemCursorIcon::Default,
                ));
        }
    }

    // Start width drag
    for interaction in handle_q.iter() {
        if *interaction == Interaction::Pressed
            && let Some((_, window)) = windows.iter().next()
            && let Some(cursor) = window.cursor_position()
        {
            sidebar_state.dragging_width = true;
            sidebar_state.drag_start = cursor.x;
            sidebar_state.drag_start_value = sidebar_state.width;
        }
    }

    // Start detail height drag
    for interaction in sep_q.iter() {
        if *interaction == Interaction::Pressed
            && let Some((_, window)) = windows.iter().next()
            && let Some(cursor) = window.cursor_position()
        {
            sidebar_state.dragging_detail = true;
            sidebar_state.drag_start = cursor.y;
            sidebar_state.drag_start_value = sidebar_state.detail_height;
        }
    }

    // While dragging width
    if sidebar_state.dragging_width {
        if mouse_btn.pressed(MouseButton::Left) {
            if let Some((_, window)) = windows.iter().next()
                && let Some(cursor) = window.cursor_position()
            {
                let delta = sidebar_state.drag_start - cursor.x;
                let new_width = (sidebar_state.drag_start_value + delta).clamp(200.0, 600.0);
                sidebar_state.width = new_width;
                for mut node in sidebar_q.iter_mut() {
                    node.width = Val::Px(new_width);
                }
            }
        } else {
            sidebar_state.dragging_width = false;
            save_sidebar_config(sidebar_state.width, sidebar_state.detail_height);
        }
    }

    // While dragging detail panel height
    if sidebar_state.dragging_detail {
        if mouse_btn.pressed(MouseButton::Left) {
            if let Some((_, window)) = windows.iter().next()
                && let Some(cursor) = window.cursor_position()
            {
                // Dragging separator up = larger detail, down = smaller
                let delta = sidebar_state.drag_start - cursor.y;
                let new_height = (sidebar_state.drag_start_value + delta).clamp(100.0, 500.0);
                sidebar_state.detail_height = new_height;
                for mut node in detail_q.iter_mut() {
                    node.height = Val::Px(new_height);
                }
            }
        } else {
            sidebar_state.dragging_detail = false;
            save_sidebar_config(sidebar_state.width, sidebar_state.detail_height);
        }
    }
}

/// Persist sidebar dimensions to config file (fire-and-forget).
fn save_sidebar_config(width: f32, _detail_height: f32) {
    std::thread::spawn(move || {
        if let Ok(mut config) = crate::config::AppConfig::load() {
            config.gui.sidebar_width = width as i32;
            let _ = config.save();
        }
    });
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
