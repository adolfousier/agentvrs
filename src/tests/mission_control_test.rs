use crate::agent::{Agent, AgentKind, AgentRegistry, AgentState};
use crate::api::observability::{ActivityEntry, ActivityKind, TaskRecord};
use crate::db::Database;
use crate::world::Position;
use chrono::Utc;

#[cfg(feature = "bevy3d")]
mod bevy_tests {
    use super::*;
    use crate::bevy3d::bridge::WorldBridge;
    use crate::bevy3d::mission_control::*;
    use bevy::prelude::*;
    use std::sync::{Arc, Mutex, RwLock};

    fn test_bridge() -> WorldBridge {
        let grid = Arc::new(RwLock::new(crate::world::build_office_world(10, 8)));
        let registry = Arc::new(RwLock::new(AgentRegistry::new()));
        let db = Arc::new(Mutex::new(Database::open_in_memory().unwrap()));
        WorldBridge { grid, registry, db }
    }

    /// Count entities matching a query filter via world_mut().
    fn count_with<F: bevy::ecs::query::QueryFilter>(app: &mut App) -> usize {
        let world = app.world_mut();
        let mut query = world.query_filtered::<(), F>();
        query.iter(world).count()
    }

    // ── MissionControlState ──────────────────────────────────────

    #[test]
    fn test_state_defaults_to_closed() {
        let state = MissionControlState::default();
        assert!(!state.open);
    }

    #[test]
    fn test_state_toggle() {
        let mut state = MissionControlState::default();
        state.open = !state.open;
        assert!(state.open);
        state.open = !state.open;
        assert!(!state.open);
    }

    // ── state_color ──────────────────────────────────────────────

    #[test]
    fn test_state_color_all_states() {
        let states = [
            AgentState::Working,
            AgentState::Thinking,
            AgentState::Eating,
            AgentState::Playing,
            AgentState::Exercising,
            AgentState::Messaging,
            AgentState::Error,
            AgentState::Walking,
            AgentState::Offline,
            AgentState::Idle,
        ];
        for s in &states {
            let _color = state_color(s); // should not panic
        }
    }

    #[test]
    fn test_state_color_working_is_green() {
        let color = state_color(&AgentState::Working);
        assert_eq!(color, Color::srgb(0.2, 0.8, 0.2));
    }

    #[test]
    fn test_state_color_error_is_red() {
        let color = state_color(&AgentState::Error);
        assert_eq!(color, Color::srgb(1.0, 0.0, 0.0));
    }

    // ── Setup system spawns correct hierarchy ────────────────────

    #[test]
    fn test_setup_spawns_root_hidden() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_mission_control);
        app.update();

        let world = app.world_mut();
        let mut query = world.query_filtered::<&Visibility, With<MissionControlRoot>>();
        let roots: Vec<_> = query.iter(world).collect();
        assert_eq!(roots.len(), 1);
        assert_eq!(*roots[0], Visibility::Hidden);
    }

    #[test]
    fn test_setup_spawns_agent_card_container() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_mission_control);
        app.update();

        assert_eq!(count_with::<With<McAgentCard>>(&mut app), 1);
    }

    #[test]
    fn test_setup_spawns_activity_feed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_mission_control);
        app.update();

        assert_eq!(count_with::<With<McActivityFeed>>(&mut app), 1);
    }

    #[test]
    fn test_setup_spawns_task_list() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_mission_control);
        app.update();

        assert_eq!(count_with::<With<McTaskList>>(&mut app), 1);
    }

    // ── Update system skips when closed ──────────────────────────

    #[test]
    fn test_update_skips_when_closed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        {
            let mut reg = bridge.registry.write().unwrap();
            let agent = Agent::new("test-agent", AgentKind::Local, Position::new(2, 2));
            reg.register(agent);
        }

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: false });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update(); // startup
        app.update(); // first update

        assert_eq!(count_with::<With<McChild>>(&mut app), 0);
    }

    // ── Update system populates when open ────────────────────────

    #[test]
    fn test_update_creates_agent_cards_when_open() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        {
            let mut reg = bridge.registry.write().unwrap();
            let mut grid = bridge.grid.write().unwrap();
            for name in &["alpha", "beta", "gamma"] {
                if let Some(pos) = grid.find_empty_floor() {
                    let agent = Agent::new(*name, AgentKind::Local, pos);
                    let id = agent.id;
                    grid.place_agent(pos, id);
                    reg.register(agent);
                }
            }
        }

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: true });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update(); // startup
        app.update(); // first update

        let child_count = count_with::<With<McChild>>(&mut app);
        assert!(
            child_count >= 3,
            "expected at least 3 McChild entities for 3 agents, got {}",
            child_count
        );
    }

    #[test]
    fn test_update_shows_activity_from_db() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        let agent_id;
        {
            let mut reg = bridge.registry.write().unwrap();
            let mut grid = bridge.grid.write().unwrap();
            let pos = grid.find_empty_floor().unwrap();
            let agent = Agent::new("active-bot", AgentKind::Local, pos);
            agent_id = agent.id;
            grid.place_agent(pos, agent_id);
            reg.register(agent);
        }
        {
            let db = bridge.db.lock().unwrap();
            for i in 0..5 {
                let entry = ActivityEntry {
                    timestamp: Utc::now(),
                    kind: ActivityKind::StateChange,
                    detail: format!("action-{}", i),
                };
                db.save_activity(agent_id, &entry).unwrap();
            }
        }

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: true });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update();
        app.update();

        let child_count = count_with::<With<McChild>>(&mut app);
        assert!(
            child_count >= 6,
            "expected at least 6 McChild entities (1 card + 5 activity), got {}",
            child_count
        );
    }

    #[test]
    fn test_update_shows_tasks_from_db() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        let agent_id;
        {
            let mut reg = bridge.registry.write().unwrap();
            let mut grid = bridge.grid.write().unwrap();
            let pos = grid.find_empty_floor().unwrap();
            let agent = Agent::new("task-bot", AgentKind::Local, pos);
            agent_id = agent.id;
            grid.place_agent(pos, agent_id);
            reg.register(agent);
        }
        {
            let db = bridge.db.lock().unwrap();
            for i in 0..3 {
                let task = TaskRecord {
                    task_id: format!("task-{}", i),
                    submitted_at: Utc::now(),
                    state: "completed".to_string(),
                    last_updated: Utc::now(),
                    response_summary: Some(format!("done-{}", i)),
                };
                db.save_task(agent_id, &task).unwrap();
            }
        }

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: true });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update();
        app.update();

        let child_count = count_with::<With<McChild>>(&mut app);
        assert!(
            child_count >= 4,
            "expected at least 4 McChild entities (1 card + 3 tasks), got {}",
            child_count
        );
    }

    #[test]
    fn test_update_cleans_up_previous_children() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        {
            let mut reg = bridge.registry.write().unwrap();
            let mut grid = bridge.grid.write().unwrap();
            let pos = grid.find_empty_floor().unwrap();
            let agent = Agent::new("cleanup-bot", AgentKind::Local, pos);
            let id = agent.id;
            grid.place_agent(pos, id);
            reg.register(agent);
        }

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: true });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update(); // startup
        app.update(); // first update

        let count_after_first = count_with::<With<McChild>>(&mut app);

        app.update(); // second update — should despawn old, spawn new

        let count_after_second = count_with::<With<McChild>>(&mut app);
        assert_eq!(count_after_first, count_after_second);
    }

    #[test]
    fn test_empty_world_shows_no_cards() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let bridge = test_bridge();

        app.insert_resource(bridge);
        app.insert_resource(MissionControlState { open: true });
        app.add_systems(Startup, setup_mission_control);
        app.add_systems(Update, update_mission_control);
        app.update();
        app.update();

        // "No activity recorded yet" + "No tasks submitted yet" = 2
        assert_eq!(count_with::<With<McChild>>(&mut app), 2);
    }
}
