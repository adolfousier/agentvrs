use crate::bevy3d::bridge::WorldBridge;
use crate::bevy3d::runner::ThemeState;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

// ── Scroll system (Bevy 0.18 requires manual scroll wiring) ─────────────────

const LINE_HEIGHT: f32 = 20.0;

/// Marker for nodes that should be scrollable in MC.
#[derive(Component)]
pub struct McScrollable;

/// Reads mouse wheel and applies scroll to any hovered McScrollable node.
pub fn ui_scroll_system(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mc_state: Res<MissionControlState>,
    mut scroll_q: Query<
        (&Interaction, &mut ScrollPosition, &Node, &ComputedNode),
        With<McScrollable>,
    >,
) {
    if !mc_state.open {
        for _ in mouse_wheel_reader.read() {}
        return;
    }

    let mut total_delta = 0.0_f32;
    for ev in mouse_wheel_reader.read() {
        match ev.unit {
            MouseScrollUnit::Line => total_delta -= ev.y * LINE_HEIGHT,
            MouseScrollUnit::Pixel => total_delta -= ev.y,
        }
    }
    if total_delta == 0.0 {
        return;
    }

    for (interaction, mut scroll_pos, _node, computed) in scroll_q.iter_mut() {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            let max_y =
                (computed.content_size().y - computed.size().y) * computed.inverse_scale_factor();
            scroll_pos.y = (scroll_pos.y + total_delta).clamp(0.0, max_y.max(0.0));
            return;
        }
    }
}

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
    /// Human-readable duration from submitted_at to last_updated (e.g. "2m 30s").
    pub duration: String,
    /// Optional full scope/description of the task.
    pub scope: String,
}

/// Marker for the task detail popup overlay.
#[derive(Component)]
pub struct McTaskPopup;

/// Marker for a clickable message row — stores message data for popup.
#[derive(Component, Clone)]
pub struct McMessageRowButton {
    pub agent_name: String,
    pub from_name: String,
    pub text: String,
    pub timestamp: String,
}

/// Marker for the message detail popup overlay.
#[derive(Component)]
pub struct McMessagePopup;

/// Marker for the inbox list container in the right panel.
#[derive(Component)]
pub struct McInboxList;

/// Marker for the "Inbox" heading.
#[derive(Component)]
pub(crate) struct McInboxHeading;

/// Resource: mission control panel open/closed state.
#[derive(Resource)]
pub struct MissionControlState {
    pub open: bool,
    /// Currently selected agent in Mission Control (filters right panel).
    pub selected_agent: Option<crate::agent::AgentId>,
    /// Whether a task detail popup is currently shown.
    pub popup_open: bool,
    /// Whether a message detail popup is currently shown.
    pub message_popup_open: bool,
    /// UI zoom scale (1.0 = default, 0.5–3.0 range).
    pub zoom: f32,
    /// Show all activity entries (not just the recent subset).
    pub show_all_activity: bool,
    /// Show all task entries (not just the recent subset).
    pub show_all_tasks: bool,
    /// Show all inbox messages (not just the recent subset).
    pub show_all_inbox: bool,
}

impl Default for MissionControlState {
    fn default() -> Self {
        Self {
            open: false,
            selected_agent: None,
            popup_open: false,
            message_popup_open: false,
            zoom: 1.0,
            show_all_activity: false,
            show_all_tasks: false,
            show_all_inbox: false,
        }
    }
}

/// Marker component on clickable agent cards, stores the agent ID.
#[derive(Component)]
pub struct McCardButton(pub crate::agent::AgentId);

/// Marker for "See All" / "Show Less" button on activity section.
#[derive(Component)]
pub struct McSeeAllActivity;

/// Marker for "See All" / "Show Less" button on task section.
#[derive(Component)]
pub struct McSeeAllTasks;

/// Marker for "See All" / "Show Less" button on inbox section.
#[derive(Component)]
pub struct McSeeAllInbox;

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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_mission_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut mc_state: ResMut<MissionControlState>,
    mut mc_root_q: Query<
        &mut Visibility,
        (
            With<MissionControlRoot>,
            Without<super::overlay::StatusBarRoot>,
            Without<super::overlay::SidebarRoot>,
        ),
    >,
    mut statusbar_q: Query<
        &mut Visibility,
        (
            With<super::overlay::StatusBarRoot>,
            Without<MissionControlRoot>,
            Without<super::overlay::SidebarRoot>,
        ),
    >,
    mut sidebar_q: Query<
        &mut Visibility,
        (
            With<super::overlay::SidebarRoot>,
            Without<MissionControlRoot>,
            Without<super::overlay::StatusBarRoot>,
        ),
    >,
    popup_q: Query<Entity, With<McTaskPopup>>,
    msg_popup_q: Query<Entity, With<McMessagePopup>>,
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
        if mc_state.message_popup_open {
            for entity in msg_popup_q.iter() {
                commands.entity(entity).despawn();
            }
            mc_state.message_popup_open = false;
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
        // Hide sidebar and status bar when MC is open
        let gui_vis = if mc_state.open {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        for mut vis in statusbar_q.iter_mut() {
            *vis = gui_vis;
        }
        for mut vis in sidebar_q.iter_mut() {
            *vis = gui_vis;
        }
    }
}

// ── Zoom with Ctrl +/- keys ──────────────────────────────────────────────────

pub fn handle_mc_zoom(mut mc_state: ResMut<MissionControlState>, keys: Res<ButtonInput<KeyCode>>) {
    if !mc_state.open || mc_state.popup_open {
        return;
    }

    let ctrl = keys.pressed(KeyCode::SuperLeft)
        || keys.pressed(KeyCode::SuperRight)
        || keys.pressed(KeyCode::ControlLeft)
        || keys.pressed(KeyCode::ControlRight);
    if !ctrl {
        return;
    }

    let mut delta = 0.0_f32;

    // Ctrl+= / Ctrl++ zooms in, Ctrl+- zooms out
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::NumpadAdd) {
        delta += 0.1;
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        delta -= 0.1;
    }

    if delta != 0.0 {
        mc_state.zoom = (mc_state.zoom + delta).clamp(0.5, 3.0);
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
                height: Val::Percent(100.0),
                column_gap: Val::Px(20.0),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|content| {
                // Left column: Agent cards (scrollable)
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(45.0),
                        height: Val::Percent(100.0),
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
                        // Agent cards container — wrapping row, scrollable
                        left.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(10.0),
                                column_gap: Val::Px(10.0),
                                height: Val::Percent(100.0),
                                align_content: AlignContent::FlexStart,
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            ScrollPosition::default(),
                            Interaction::default(),
                            McScrollable,
                            McAgentCard,
                        ));
                    });

                // Right column: Activity feed + Tasks
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(55.0),
                        height: Val::Percent(100.0),
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
                            McActivityHeading,
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(30.0),
                                overflow: Overflow::scroll_y(),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            ScrollPosition::default(),
                            Interaction::default(),
                            McScrollable,
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
                                height: Val::Percent(30.0),
                                overflow: Overflow::scroll_y(),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            ScrollPosition::default(),
                            Interaction::default(),
                            McScrollable,
                            BackgroundColor(Color::srgb(0.11, 0.12, 0.14)),
                            BorderColor::all(Color::srgb(0.18, 0.19, 0.23)),
                            McTaskList,
                        ));

                        // Inbox section
                        right.spawn((
                            Text::new("Inbox"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.58, 0.68, 0.90)),
                            McHeading,
                            McInboxHeading,
                        ));
                        right.spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(30.0),
                                overflow: Overflow::scroll_y(),
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            ScrollPosition::default(),
                            Interaction::default(),
                            McScrollable,
                            BackgroundColor(Color::srgb(0.11, 0.12, 0.14)),
                            BorderColor::all(Color::srgb(0.18, 0.19, 0.23)),
                            McInboxList,
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
    inbox_q: Query<Entity, With<McInboxList>>,
    child_q: Query<Entity, With<McChild>>,
    mut title_q: Query<&mut TextColor, With<McTitle>>,
    mut hint_q: Query<(Entity, &mut TextColor), (With<McHint>, Without<McTitle>)>,
    mut heading_q: Query<&mut TextColor, (With<McHeading>, Without<McTitle>, Without<McHint>)>,
) {
    if !mc_state.open {
        return;
    }

    let t = mc_theme(theme.is_dark);
    let z = mc_state.zoom;
    let font = |size: f32| TextFont {
        font_size: size * z,
        ..default()
    };
    let px = |v: f32| Val::Px(v * z);

    // Update root background for theme
    for mut bg in root_q.iter_mut() {
        bg.0 = t.bg;
    }

    // Update static text colors for theme
    for mut tc in title_q.iter_mut() {
        tc.0 = t.title;
    }
    for (entity, mut tc) in hint_q.iter_mut() {
        tc.0 = t.text_muted;
        let zoom_pct = (z * 100.0) as u32;
        commands.entity(entity).insert(Text::new(format!(
            "M:close  Ctrl+/-:zoom ({zoom_pct}%)  scroll:navigate  click:select"
        )));
    }
    for mut tc in heading_q.iter_mut() {
        tc.0 = t.heading;
    }

    // Remove previous frame's children
    for entity in child_q.iter() {
        commands.entity(entity).despawn();
    }

    let Ok(registry) = bridge.registry.read() else {
        return;
    };

    // ── Agent cards (GitHub-style repo cards) ─────────────────────
    if let Ok(card_parent) = card_q.single() {
        if registry.agents().next().is_none() {
            let empty = commands
                .spawn((
                    Text::new("No agents connected"),
                    font(12.0),
                    TextColor(t.text_muted),
                    McChild,
                ))
                .id();
            commands.entity(card_parent).add_child(empty);
        }

        // Load per-agent tasks and activity from DB
        let mut agent_tasks: std::collections::HashMap<
            crate::agent::AgentId,
            Vec<crate::api::observability::TaskRecord>,
        > = std::collections::HashMap::new();
        let mut agent_activity: std::collections::HashMap<
            crate::agent::AgentId,
            Vec<crate::api::observability::ActivityEntry>,
        > = std::collections::HashMap::new();
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
                        padding: UiRect::all(px(14.0)),
                        row_gap: px(6.0),
                        width: px(280.0),
                        max_height: Val::Percent(90.0),
                        border: UiRect::all(px(if is_selected { 2.0 } else { 1.0 })),
                        border_radius: BorderRadius::all(px(8.0)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    ScrollPosition::default(),
                    BackgroundColor(t.card_bg),
                    BorderColor::all(card_border_color),
                    Interaction::default(),
                    McScrollable,
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
                            column_gap: px(8.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|name_row| {
                            name_row.spawn((
                                Node {
                                    width: px(10.0),
                                    height: px(10.0),
                                    border_radius: BorderRadius::all(px(5.0)),
                                    ..default()
                                },
                                BackgroundColor(dot_color),
                            ));
                            name_row.spawn((
                                Text::new(&agent.name),
                                font(14.0),
                                TextColor(t.text_primary),
                            ));
                        });
                        // Kind badge
                        top.spawn((
                            Node {
                                padding: UiRect::new(px(8.0), px(8.0), px(2.0), px(2.0)),
                                border: UiRect::all(px(1.0)),
                                border_radius: BorderRadius::all(px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderColor::all(t.card_border),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                Text::new(kind_label),
                                font(10.0),
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
                            font(12.0),
                            TextColor(t.text_secondary),
                        ));
                        row.spawn((
                            Text::new(format!("({},{})", agent.position.x, agent.position.y)),
                            font(10.0),
                            TextColor(t.text_muted),
                        ));
                        if !agent.inbox.is_empty() {
                            row.spawn((
                                Node {
                                    padding: UiRect::new(px(7.0), px(7.0), px(2.0), px(2.0)),
                                    border_radius: BorderRadius::all(px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(t.badge_bg),
                            ))
                            .with_children(|badge| {
                                badge.spawn((
                                    Text::new(format!("{} msg", agent.inbox.len())),
                                    font(10.0),
                                    TextColor(t.badge_text),
                                ));
                            });
                        }
                    });

                    // ── Recent tasks section ──
                    if let Some(tasks) = tasks
                        && !tasks.is_empty()
                    {
                        // Separator
                        card.spawn((
                            Node {
                                height: px(1.0),
                                ..default()
                            },
                            BackgroundColor(t.separator),
                        ));
                        card.spawn((Text::new("Tasks"), font(10.0), TextColor(t.heading)));
                        for task in tasks.iter().rev().take(3) {
                            let task_dot = match task.state.as_str() {
                                "completed" => Color::srgb(0.2, 0.8, 0.2),
                                "failed" => Color::srgb(1.0, 0.3, 0.3),
                                _ => Color::srgb(0.3, 0.7, 1.0),
                            };
                            card.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: px(6.0),
                                align_items: AlignItems::FlexStart,
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    Node {
                                        width: px(6.0),
                                        height: px(6.0),
                                        border_radius: BorderRadius::all(px(3.0)),
                                        margin: UiRect::top(px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(task_dot),
                                ));
                                // State badge
                                row.spawn((
                                    Node {
                                        padding: UiRect::new(px(5.0), px(5.0), px(1.0), px(1.0)),
                                        border: UiRect::all(px(1.0)),
                                        border_radius: BorderRadius::all(px(8.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor::all(task_dot),
                                ))
                                .with_children(|b| {
                                    b.spawn((
                                        Text::new(&task.state),
                                        font(9.0),
                                        TextColor(task_dot),
                                    ));
                                });
                                // Summary — full text, wrapping
                                let summary =
                                    task.response_summary.as_deref().unwrap_or(&task.task_id);
                                row.spawn((
                                    Text::new(summary),
                                    font(10.0),
                                    TextColor(t.text_secondary),
                                    Node {
                                        flex_shrink: 1.0,
                                        ..default()
                                    },
                                ));
                            });
                        }
                    }

                    // ── Recent activity section ──
                    if let Some(entries) = activity
                        && !entries.is_empty()
                    {
                        card.spawn((
                            Node {
                                height: px(1.0),
                                ..default()
                            },
                            BackgroundColor(t.separator),
                        ));
                        card.spawn((Text::new("Activity"), font(10.0), TextColor(t.heading)));
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
                                column_gap: px(6.0),
                                align_items: AlignItems::FlexStart,
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    Text::new(&ago),
                                    font(9.0),
                                    TextColor(t.text_muted),
                                    Node {
                                        min_width: px(28.0),
                                        ..default()
                                    },
                                ));
                                row.spawn((
                                    Text::new(&entry.detail),
                                    font(10.0),
                                    TextColor(t.text_secondary),
                                    Node {
                                        flex_shrink: 1.0,
                                        ..default()
                                    },
                                ));
                            });
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
            let db_limit = if mc_state.show_all_activity { 500 } else { 50 };
            for agent in &agents_to_show {
                if let Ok(entries) = db.load_activity(&agent.id, db_limit) {
                    for entry in entries {
                        all_activity.push((agent.name.clone(), entry));
                    }
                }
            }
        }
        all_activity.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        let total_activity = all_activity.len();
        if !mc_state.show_all_activity {
            let max_items = if mc_state.selected_agent.is_some() {
                20
            } else {
                10
            };
            all_activity.truncate(max_items);
        }

        if all_activity.is_empty() {
            let empty = commands
                .spawn((
                    Node {
                        padding: UiRect::all(px(16.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("No activity recorded yet"),
                        font(12.0),
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
                        column_gap: px(12.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::new(px(14.0), px(14.0), px(6.0), px(6.0)),
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(&time_str),
                        font(11.0),
                        TextColor(t.text_muted),
                        Node {
                            min_width: px(55.0),
                            ..default()
                        },
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        font(11.0),
                        TextColor(t.link),
                        Node {
                            min_width: px(90.0),
                            ..default()
                        },
                    ));
                    let detail = entry.detail.clone();
                    row.spawn((Text::new(detail), font(11.0), TextColor(t.text_secondary)));
                })
                .id();
            commands.entity(feed_parent).add_child(row);
        }

        // "See All" / "Show Less" button if there are more items
        if total_activity > all_activity.len() || mc_state.show_all_activity {
            let label = if mc_state.show_all_activity {
                format!("Show Less (showing {})", all_activity.len())
            } else {
                format!("See All ({} total)", total_activity)
            };
            let btn = commands
                .spawn((
                    Node {
                        padding: UiRect::new(px(14.0), px(14.0), px(8.0), px(8.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    Interaction::default(),
                    McSeeAllActivity,
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((Text::new(label), font(11.0), TextColor(t.link)));
                })
                .id();
            commands.entity(feed_parent).add_child(btn);
        }
    }

    // ── Inbox (from DB) — message list rows ─────────────────────
    if let Ok(inbox_parent) = inbox_q.single() {
        commands.entity(inbox_parent).insert((
            BackgroundColor(t.section_bg),
            BorderColor::all(t.section_border),
        ));

        let mut all_messages: Vec<(String, crate::agent::AgentMessage)> = Vec::new();
        if let Ok(db) = bridge.db.lock() {
            let agents_to_show: Vec<_> = match mc_state.selected_agent {
                Some(sel) => registry.agents().filter(|a| a.id == sel).collect(),
                None => registry.agents().collect(),
            };
            let db_limit = if mc_state.show_all_inbox { 500 } else { 50 };
            for agent in &agents_to_show {
                if let Ok(msgs) = db.load_messages_for(&agent.id, db_limit) {
                    for msg in msgs {
                        all_messages.push((agent.name.clone(), msg));
                    }
                }
            }
        }
        all_messages.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        let total_messages = all_messages.len();
        if !mc_state.show_all_inbox {
            let max_items = if mc_state.selected_agent.is_some() {
                20
            } else {
                10
            };
            all_messages.truncate(max_items);
        }

        if all_messages.is_empty() {
            let empty = commands
                .spawn((
                    Node {
                        padding: UiRect::all(px(16.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("No messages yet"),
                        font(12.0),
                        TextColor(t.text_muted),
                    ));
                })
                .id();
            commands.entity(inbox_parent).add_child(empty);
        }

        // Resolve sender names from registry
        let resolve_name = |agent_id: &crate::agent::AgentId| -> String {
            registry
                .get(agent_id)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| format!("{:.8}", agent_id))
        };

        for (i, (agent_name, msg)) in all_messages.iter().enumerate() {
            let time_ago = (chrono::Utc::now() - msg.timestamp).num_seconds();
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

            let from_name = resolve_name(&msg.from);

            let row = commands
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(12.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::new(px(14.0), px(14.0), px(6.0), px(6.0)),
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    Interaction::default(),
                    McMessageRowButton {
                        agent_name: agent_name.clone(),
                        from_name: from_name.clone(),
                        text: msg.text.clone(),
                        timestamp: msg.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(&time_str),
                        font(11.0),
                        TextColor(t.text_muted),
                        Node {
                            min_width: px(55.0),
                            ..default()
                        },
                    ));
                    // Envelope icon
                    row.spawn((
                        Node {
                            width: px(8.0),
                            height: px(8.0),
                            border_radius: BorderRadius::all(px(4.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.7, 1.0)),
                    ));
                    row.spawn((
                        Text::new(format!("{} → {}", from_name, agent_name)),
                        font(11.0),
                        TextColor(t.link),
                        Node {
                            min_width: px(130.0),
                            ..default()
                        },
                    ));
                    // Message preview (truncated)
                    let preview = if msg.text.len() > 60 {
                        format!("{}…", &msg.text[..60])
                    } else {
                        msg.text.clone()
                    };
                    row.spawn((
                        Text::new(preview),
                        font(11.0),
                        TextColor(t.text_secondary),
                        Node {
                            flex_shrink: 1.0,
                            ..default()
                        },
                    ));
                })
                .id();
            commands.entity(inbox_parent).add_child(row);
        }

        // "See All" / "Show Less" button
        if total_messages > all_messages.len() || mc_state.show_all_inbox {
            let label = if mc_state.show_all_inbox {
                format!("Show Less (showing {})", all_messages.len())
            } else {
                format!("See All ({} total)", total_messages)
            };
            let btn = commands
                .spawn((
                    Node {
                        padding: UiRect::new(px(14.0), px(14.0), px(8.0), px(8.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    Interaction::default(),
                    McSeeAllInbox,
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((Text::new(label), font(11.0), TextColor(t.link)));
                })
                .id();
            commands.entity(inbox_parent).add_child(btn);
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
            let db_limit = if mc_state.show_all_tasks { 500 } else { 50 };
            for agent in &agents_to_show {
                if let Ok(tasks) = db.load_tasks(&agent.id, db_limit) {
                    for task in tasks {
                        all_tasks.push((agent.name.clone(), task));
                    }
                }
            }
        }
        all_tasks.sort_by(|a, b| b.1.last_updated.cmp(&a.1.last_updated));
        let total_tasks = all_tasks.len();
        if !mc_state.show_all_tasks {
            let max_items = if mc_state.selected_agent.is_some() {
                20
            } else {
                10
            };
            all_tasks.truncate(max_items);
        }

        if all_tasks.is_empty() {
            let empty = commands
                .spawn((
                    Node {
                        padding: UiRect::all(px(16.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("No tasks submitted yet"),
                        font(12.0),
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
                        column_gap: px(12.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::new(px(14.0), px(14.0), px(6.0), px(6.0)),
                        ..default()
                    },
                    BackgroundColor(row_bg),
                    Interaction::default(),
                    McTaskRowButton {
                        agent_name: agent_name.clone(),
                        task_id: task.task_id.clone(),
                        state: task.state.clone(),
                        summary: task.response_summary.clone().unwrap_or_default(),
                        submitted_at: task
                            .submitted_at
                            .format("%Y-%m-%d %H:%M:%S UTC")
                            .to_string(),
                        last_updated: task
                            .last_updated
                            .format("%Y-%m-%d %H:%M:%S UTC")
                            .to_string(),
                        duration: format_duration(task.last_updated - task.submitted_at),
                        scope: task.scope.clone().unwrap_or_default(),
                    },
                    McChild,
                ))
                .with_children(|row| {
                    // State dot
                    row.spawn((
                        Node {
                            width: px(8.0),
                            height: px(8.0),
                            border_radius: BorderRadius::all(px(4.0)),
                            ..default()
                        },
                        BackgroundColor(task_state_color),
                    ));
                    row.spawn((
                        Text::new(agent_name),
                        font(11.0),
                        TextColor(t.link),
                        Node {
                            min_width: px(90.0),
                            ..default()
                        },
                    ));
                    // State badge
                    row.spawn((
                        Node {
                            padding: UiRect::new(px(7.0), px(7.0), px(1.0), px(1.0)),
                            border: UiRect::all(px(1.0)),
                            border_radius: BorderRadius::all(px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(task_state_color),
                    ))
                    .with_children(|badge| {
                        badge.spawn((
                            Text::new(&task.state),
                            font(10.0),
                            TextColor(task_state_color),
                        ));
                    });
                    let summary = task
                        .response_summary
                        .as_deref()
                        .unwrap_or(&task.task_id)
                        .to_string();
                    row.spawn((
                        Text::new(summary),
                        font(11.0),
                        TextColor(t.text_secondary),
                        Node {
                            flex_shrink: 1.0,
                            ..default()
                        },
                    ));
                    // Duration for completed/failed tasks
                    if task.state == "completed" || task.state == "failed" {
                        let dur = format_duration(task.last_updated - task.submitted_at);
                        row.spawn((
                            Text::new(dur),
                            font(10.0),
                            TextColor(t.text_muted),
                            Node {
                                min_width: px(50.0),
                                ..default()
                            },
                        ));
                    }
                })
                .id();
            commands.entity(task_parent).add_child(row);
        }

        // "See All" / "Show Less" button if there are more items
        if total_tasks > all_tasks.len() || mc_state.show_all_tasks {
            let label = if mc_state.show_all_tasks {
                format!("Show Less (showing {})", all_tasks.len())
            } else {
                format!("See All ({} total)", total_tasks)
            };
            let btn = commands
                .spawn((
                    Node {
                        padding: UiRect::new(px(14.0), px(14.0), px(8.0), px(8.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    Interaction::default(),
                    McSeeAllTasks,
                    McChild,
                ))
                .with_children(|row| {
                    row.spawn((Text::new(label), font(11.0), TextColor(t.link)));
                })
                .id();
            commands.entity(task_parent).add_child(btn);
        }
    }
}

// ── Card click handler ───────────────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn handle_card_clicks(
    mut mc_state: ResMut<MissionControlState>,
    card_q: Query<(&Interaction, &McCardButton), Changed<Interaction>>,
    bridge: Res<WorldBridge>,
    mut activity_heading_q: Query<
        &mut Text,
        (
            With<McActivityHeading>,
            Without<McTaskHeading>,
            Without<McInboxHeading>,
        ),
    >,
    mut task_heading_q: Query<
        &mut Text,
        (
            With<McTaskHeading>,
            Without<McActivityHeading>,
            Without<McInboxHeading>,
        ),
    >,
    mut inbox_heading_q: Query<
        &mut Text,
        (
            With<McInboxHeading>,
            Without<McActivityHeading>,
            Without<McTaskHeading>,
        ),
    >,
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
                bridge
                    .registry
                    .read()
                    .ok()
                    .and_then(|reg| reg.get(&id).map(|a| a.name.clone()))
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
            for mut text in inbox_heading_q.iter_mut() {
                **text = match &agent_name {
                    Some(name) => format!("Inbox — {}", name),
                    None => "Inbox".to_string(),
                };
            }
        }
    }
}

// ── See All toggle handler ───────────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn handle_see_all_clicks(
    mut mc_state: ResMut<MissionControlState>,
    activity_btn_q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<McSeeAllActivity>,
            Without<McSeeAllTasks>,
            Without<McSeeAllInbox>,
        ),
    >,
    task_btn_q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<McSeeAllTasks>,
            Without<McSeeAllActivity>,
            Without<McSeeAllInbox>,
        ),
    >,
    inbox_btn_q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<McSeeAllInbox>,
            Without<McSeeAllActivity>,
            Without<McSeeAllTasks>,
        ),
    >,
) {
    if !mc_state.open {
        return;
    }
    for interaction in activity_btn_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.show_all_activity = !mc_state.show_all_activity;
        }
    }
    for interaction in task_btn_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.show_all_tasks = !mc_state.show_all_tasks;
        }
    }
    for interaction in inbox_btn_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.show_all_inbox = !mc_state.show_all_inbox;
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
            spawn_task_popup(&mut commands, &btn, theme.is_dark, mc_state.zoom);
        }
        return;
    }

    for (interaction, btn) in task_row_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.popup_open = true;
            spawn_task_popup(&mut commands, btn, theme.is_dark, mc_state.zoom);
            return;
        }
    }
}

// ── Message popup handler ─────────────────────────────────────────────────

pub fn handle_message_popup(
    mut commands: Commands,
    mut mc_state: ResMut<MissionControlState>,
    theme: Res<ThemeState>,
    msg_row_q: Query<(&Interaction, &McMessageRowButton), Changed<Interaction>>,
    popup_q: Query<Entity, With<McMessagePopup>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Dismiss popup on Escape
    if mc_state.message_popup_open && keys.just_pressed(KeyCode::Escape) {
        for entity in popup_q.iter() {
            commands.entity(entity).despawn();
        }
        mc_state.message_popup_open = false;
        return;
    }

    // Open/replace popup on message row click
    if mc_state.message_popup_open {
        let mut clicked = None;
        for (interaction, btn) in msg_row_q.iter() {
            if *interaction == Interaction::Pressed {
                clicked = Some(btn.clone());
            }
        }
        if let Some(btn) = clicked {
            for entity in popup_q.iter() {
                commands.entity(entity).despawn();
            }
            spawn_message_popup(&mut commands, &btn, theme.is_dark, mc_state.zoom);
        }
        return;
    }

    for (interaction, btn) in msg_row_q.iter() {
        if *interaction == Interaction::Pressed {
            mc_state.message_popup_open = true;
            spawn_message_popup(&mut commands, btn, theme.is_dark, mc_state.zoom);
            return;
        }
    }
}

/// Spawn a label + value field pair inside a dialog.
macro_rules! field_node {
    ($parent:expr, $theme:expr, $label:expr, $value:expr, $zoom:expr) => {
        $parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0 * $zoom),
                ..default()
            })
            .with_children(|field| {
                field.spawn((
                    Text::new($label),
                    TextFont {
                        font_size: 11.0 * $zoom,
                        ..default()
                    },
                    TextColor($theme.text_muted),
                ));
                field.spawn((
                    Text::new($value),
                    TextFont {
                        font_size: 13.0 * $zoom,
                        ..default()
                    },
                    TextColor($theme.text_primary),
                ));
            });
    };
}

fn spawn_message_popup(
    commands: &mut Commands,
    btn: &McMessageRowButton,
    is_dark: bool,
    zoom: f32,
) {
    let t = mc_theme(is_dark);
    let font = |size: f32| TextFont {
        font_size: size * zoom,
        ..default()
    };
    let px = |v: f32| Val::Px(v * zoom);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(0.0),
                left: px(0.0),
                right: px(0.0),
                bottom: px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(100),
            McMessagePopup,
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(px(24.0)),
                        row_gap: px(14.0),
                        width: px(480.0),
                        max_height: Val::Percent(70.0),
                        overflow: Overflow::scroll_y(),
                        border: UiRect::all(px(1.0)),
                        border_radius: BorderRadius::all(px(12.0)),
                        ..default()
                    },
                    ScrollPosition::default(),
                    Interaction::default(),
                    McScrollable,
                    BackgroundColor(t.card_bg),
                    BorderColor::all(t.card_border),
                ))
                .with_children(|dialog| {
                    // Header
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|header| {
                            header.spawn((
                                Text::new("Message Detail"),
                                font(18.0),
                                TextColor(t.title),
                            ));
                            header.spawn((
                                Text::new("press Esc to close"),
                                font(11.0),
                                TextColor(t.text_muted),
                            ));
                        });

                    // Separator
                    dialog.spawn((
                        Node {
                            height: px(1.0),
                            ..default()
                        },
                        BackgroundColor(t.separator),
                    ));

                    // From
                    field_node!(dialog, &t, "From", &btn.from_name, zoom);

                    // To
                    field_node!(dialog, &t, "To", &btn.agent_name, zoom);

                    // Timestamp
                    field_node!(dialog, &t, "Timestamp", &btn.timestamp, zoom);

                    // Separator
                    dialog.spawn((
                        Node {
                            height: px(1.0),
                            ..default()
                        },
                        BackgroundColor(t.separator),
                    ));

                    // Message body
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: px(4.0),
                            ..default()
                        })
                        .with_children(|field| {
                            field.spawn((
                                Text::new("Message"),
                                font(11.0),
                                TextColor(t.text_muted),
                            ));
                            field.spawn((
                                Text::new(&btn.text),
                                font(13.0),
                                TextColor(t.text_primary),
                            ));
                        });
                });
        });
}

fn spawn_task_popup(commands: &mut Commands, btn: &McTaskRowButton, is_dark: bool, zoom: f32) {
    let t = mc_theme(is_dark);
    let font = |size: f32| TextFont {
        font_size: size * zoom,
        ..default()
    };
    let px = |v: f32| Val::Px(v * zoom);
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
                top: px(0.0),
                left: px(0.0),
                right: px(0.0),
                bottom: px(0.0),
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
                        padding: UiRect::all(px(24.0)),
                        row_gap: px(14.0),
                        width: px(480.0),
                        max_height: Val::Percent(70.0),
                        overflow: Overflow::scroll_y(),
                        border: UiRect::all(px(1.0)),
                        border_radius: BorderRadius::all(px(12.0)),
                        ..default()
                    },
                    ScrollPosition::default(),
                    Interaction::default(),
                    McScrollable,
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
                                font(18.0),
                                TextColor(t.title),
                            ));
                            header.spawn((
                                Text::new("press Esc to close"),
                                font(11.0),
                                TextColor(t.text_muted),
                            ));
                        });

                    // Separator
                    dialog.spawn((
                        Node {
                            height: px(1.0),
                            ..default()
                        },
                        BackgroundColor(t.separator),
                    ));

                    // Task ID
                    field_node!(dialog, &t, "Task ID", &btn.task_id, zoom);

                    // Agent
                    field_node!(dialog, &t, "Agent", &btn.agent_name, zoom);

                    // State with colored badge
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: px(4.0),
                            ..default()
                        })
                        .with_children(|field| {
                            field.spawn((Text::new("State"), font(11.0), TextColor(t.text_muted)));
                            field
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: px(8.0),
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|row| {
                                    row.spawn((
                                        Node {
                                            width: px(10.0),
                                            height: px(10.0),
                                            border_radius: BorderRadius::all(px(5.0)),
                                            ..default()
                                        },
                                        BackgroundColor(task_color),
                                    ));
                                    row.spawn((
                                        Node {
                                            padding: UiRect::new(
                                                px(8.0),
                                                px(8.0),
                                                px(2.0),
                                                px(2.0),
                                            ),
                                            border: UiRect::all(px(1.0)),
                                            border_radius: BorderRadius::all(px(10.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::NONE),
                                        BorderColor::all(task_color),
                                    ))
                                    .with_children(|badge| {
                                        badge.spawn((
                                            Text::new(&btn.state),
                                            font(12.0),
                                            TextColor(task_color),
                                        ));
                                    });
                                });
                        });

                    // Summary
                    if !btn.summary.is_empty() {
                        field_node!(dialog, &t, "Summary", &btn.summary, zoom);
                    }

                    // Scope
                    if !btn.scope.is_empty() {
                        field_node!(dialog, &t, "Scope", &btn.scope, zoom);
                    }

                    // Timestamps
                    field_node!(dialog, &t, "Submitted", &btn.submitted_at, zoom);
                    field_node!(dialog, &t, "Last Updated", &btn.last_updated, zoom);

                    // Duration (for completed/failed tasks)
                    if btn.state == "completed" || btn.state == "failed" {
                        field_node!(dialog, &t, "Duration", &btn.duration, zoom);
                    }
                });
        });
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Format a chrono::TimeDelta as a human-readable duration string.
fn format_duration(d: chrono::TimeDelta) -> String {
    let total_secs = d.num_seconds().max(0);
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

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
