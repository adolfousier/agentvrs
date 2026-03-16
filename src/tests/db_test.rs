use crate::agent::{Agent, AgentId, AgentKind, AgentMessage, AgentState};
use crate::api::observability::{ActivityEntry, ActivityKind, HeartbeatInfo, TaskRecord};
use crate::db::Database;
use crate::world::Position;
use chrono::Utc;

fn test_db() -> Database {
    Database::open_in_memory().unwrap()
}

// ── Agent persistence ────────────────────────────────────────

#[test]
fn test_save_and_load_agent() {
    let db = test_db();
    let agent = Agent::new("test-bot", AgentKind::Local, Position::new(5, 3));
    db.save_agent(&agent).unwrap();

    let loaded = db.load_agents().unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id, agent.id);
    assert_eq!(loaded[0].name, "test-bot");
    assert_eq!(loaded[0].position, Position::new(5, 3));
}

#[test]
fn test_save_external_agent() {
    let db = test_db();
    let agent = Agent::new(
        "ext-bot",
        AgentKind::External {
            endpoint: "http://localhost:9090".to_string(),
        },
        Position::new(2, 4),
    );
    db.save_agent(&agent).unwrap();

    let loaded = db.load_agents().unwrap();
    assert_eq!(loaded.len(), 1);
    assert!(matches!(loaded[0].kind, AgentKind::External { .. }));
}

#[test]
fn test_save_multiple_agents() {
    let db = test_db();
    for i in 0..5 {
        let agent = Agent::new(
            format!("bot-{}", i),
            AgentKind::Local,
            Position::new(i as u16, 0),
        );
        db.save_agent(&agent).unwrap();
    }

    let loaded = db.load_agents().unwrap();
    assert_eq!(loaded.len(), 5);
}

#[test]
fn test_remove_agent() {
    let db = test_db();
    let agent = Agent::new("removable", AgentKind::Local, Position::new(1, 1));
    let id = agent.id;
    db.save_agent(&agent).unwrap();
    assert_eq!(db.load_agents().unwrap().len(), 1);

    db.remove_agent(&id).unwrap();
    assert_eq!(db.load_agents().unwrap().len(), 0);
}

#[test]
fn test_update_agent_position() {
    let db = test_db();
    let agent = Agent::new("mover", AgentKind::Local, Position::new(1, 1));
    let id = agent.id;
    db.save_agent(&agent).unwrap();

    db.update_agent_position(&id, Position::new(10, 12))
        .unwrap();
    let loaded = db.load_agents().unwrap();
    assert_eq!(loaded[0].position, Position::new(10, 12));
}

#[test]
fn test_save_agent_upsert() {
    let db = test_db();
    let mut agent = Agent::new("upsert", AgentKind::Local, Position::new(1, 1));
    db.save_agent(&agent).unwrap();

    // Update same agent (same ID)
    agent.position = Position::new(9, 9);
    db.save_agent(&agent).unwrap();

    let loaded = db.load_agents().unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].position, Position::new(9, 9));
}

// ── Message persistence ──────────────────────────────────────

#[test]
fn test_save_and_load_message() {
    let db = test_db();
    let from = AgentId::new();
    let to = AgentId::new();
    let msg = AgentMessage::new(from, to, "hello world");
    db.save_message(&msg).unwrap();

    let loaded = db.load_messages_for(&to, 50).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].from, from);
    assert_eq!(loaded[0].to, to);
    assert_eq!(loaded[0].text, "hello world");
}

#[test]
fn test_load_messages_limit() {
    let db = test_db();
    let from = AgentId::new();
    let to = AgentId::new();
    for i in 0..10 {
        let msg = AgentMessage::new(from, to, format!("msg-{}", i));
        db.save_message(&msg).unwrap();
    }

    let loaded = db.load_messages_for(&to, 3).unwrap();
    assert_eq!(loaded.len(), 3);
}

#[test]
fn test_clear_messages() {
    let db = test_db();
    let from = AgentId::new();
    let to = AgentId::new();
    for i in 0..5 {
        let msg = AgentMessage::new(from, to, format!("msg-{}", i));
        db.save_message(&msg).unwrap();
    }

    let cleared = db.clear_messages_for(&to).unwrap();
    assert_eq!(cleared, 5);

    let loaded = db.load_messages_for(&to, 50).unwrap();
    assert!(loaded.is_empty());
}

#[test]
fn test_messages_only_for_recipient() {
    let db = test_db();
    let a = AgentId::new();
    let b = AgentId::new();
    let c = AgentId::new();

    db.save_message(&AgentMessage::new(a, b, "for b")).unwrap();
    db.save_message(&AgentMessage::new(a, c, "for c")).unwrap();

    let b_inbox = db.load_messages_for(&b, 50).unwrap();
    assert_eq!(b_inbox.len(), 1);
    assert_eq!(b_inbox[0].text, "for b");

    let c_inbox = db.load_messages_for(&c, 50).unwrap();
    assert_eq!(c_inbox.len(), 1);
    assert_eq!(c_inbox[0].text, "for c");
}

// ── Activity persistence ─────────────────────────────────────

#[test]
fn test_save_and_load_activity() {
    let db = test_db();
    let id = AgentId::new();
    let entry = ActivityEntry {
        timestamp: Utc::now(),
        kind: ActivityKind::Spawned,
        detail: "Agent connected".to_string(),
    };
    db.save_activity(id, &entry).unwrap();

    let loaded = db.load_activity(&id, 10).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].detail, "Agent connected");
}

#[test]
fn test_activity_limit() {
    let db = test_db();
    let id = AgentId::new();
    for i in 0..20 {
        let entry = ActivityEntry {
            timestamp: Utc::now(),
            kind: ActivityKind::StateChange,
            detail: format!("action-{}", i),
        };
        db.save_activity(id, &entry).unwrap();
    }

    let loaded = db.load_activity(&id, 5).unwrap();
    assert_eq!(loaded.len(), 5);
}

// ── Task persistence ─────────────────────────────────────────

#[test]
fn test_save_and_load_task() {
    let db = test_db();
    let id = AgentId::new();
    let task = TaskRecord {
        task_id: "task-001".to_string(),
        submitted_at: Utc::now(),
        state: "submitted".to_string(),
        last_updated: Utc::now(),
        response_summary: Some("processing".to_string()),
    };
    db.save_task(id, &task).unwrap();

    let loaded = db.load_tasks(&id, 10).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].task_id, "task-001");
    assert_eq!(loaded[0].state, "submitted");
}

#[test]
fn test_task_upsert() {
    let db = test_db();
    let id = AgentId::new();
    let task = TaskRecord {
        task_id: "task-002".to_string(),
        submitted_at: Utc::now(),
        state: "submitted".to_string(),
        last_updated: Utc::now(),
        response_summary: None,
    };
    db.save_task(id, &task).unwrap();

    // Update same task
    let updated = TaskRecord {
        state: "completed".to_string(),
        response_summary: Some("done".to_string()),
        ..task
    };
    db.save_task(id, &updated).unwrap();

    let loaded = db.load_tasks(&id, 10).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].state, "completed");
}

// ── Heartbeat persistence ────────────────────────────────────

#[test]
fn test_save_and_load_heartbeat() {
    let db = test_db();
    let id = AgentId::new();
    let hb = HeartbeatInfo {
        last_seen: Utc::now(),
        status: "healthy".to_string(),
        metadata: Some(serde_json::json!({"cpu": 0.42})),
    };
    db.save_heartbeat(id, &hb).unwrap();

    let loaded = db.load_heartbeats().unwrap();
    assert_eq!(loaded.len(), 1);
    assert!(loaded.contains_key(&id));
    assert_eq!(loaded[&id].status, "healthy");
}

#[test]
fn test_heartbeat_upsert() {
    let db = test_db();
    let id = AgentId::new();
    let hb1 = HeartbeatInfo {
        last_seen: Utc::now(),
        status: "ok".to_string(),
        metadata: None,
    };
    db.save_heartbeat(id, &hb1).unwrap();

    let hb2 = HeartbeatInfo {
        last_seen: Utc::now(),
        status: "degraded".to_string(),
        metadata: None,
    };
    db.save_heartbeat(id, &hb2).unwrap();

    let loaded = db.load_heartbeats().unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[&id].status, "degraded");
}

// ── Purge agent ──────────────────────────────────────────────

#[test]
fn test_purge_agent_removes_all_data() {
    let db = test_db();
    let id = AgentId::new();
    let other = AgentId::new();

    // Create agent with messages, activity, task, heartbeat
    let agent = Agent::new("purge-me", AgentKind::Local, Position::new(1, 1));
    let id = agent.id;
    db.save_agent(&agent).unwrap();
    db.save_message(&AgentMessage::new(other, id, "hello"))
        .unwrap();
    db.save_activity(
        id,
        &ActivityEntry {
            timestamp: Utc::now(),
            kind: ActivityKind::Spawned,
            detail: "connected".to_string(),
        },
    )
    .unwrap();
    db.save_task(
        id,
        &TaskRecord {
            task_id: "t1".to_string(),
            submitted_at: Utc::now(),
            state: "done".to_string(),
            last_updated: Utc::now(),
            response_summary: None,
        },
    )
    .unwrap();
    db.save_heartbeat(
        id,
        &HeartbeatInfo {
            last_seen: Utc::now(),
            status: "ok".to_string(),
            metadata: None,
        },
    )
    .unwrap();

    // Purge
    db.purge_agent(&id).unwrap();

    assert!(db.load_agents().unwrap().is_empty());
    assert!(db.load_messages_for(&id, 50).unwrap().is_empty());
    assert!(db.load_activity(&id, 50).unwrap().is_empty());
    assert!(db.load_tasks(&id, 50).unwrap().is_empty());
    assert!(db.load_heartbeats().unwrap().is_empty());
}

// ── Restore agent ────────────────────────────────────────────

#[test]
fn test_agent_restore_constructor() {
    let id = AgentId::new();
    let agent = Agent::restore(
        id,
        "restored".to_string(),
        AgentKind::External {
            endpoint: "http://test:8080".to_string(),
        },
        Position::new(7, 3),
        4,
    );
    assert_eq!(agent.id, id);
    assert_eq!(agent.name, "restored");
    assert_eq!(agent.state, AgentState::Idle);
    assert_eq!(agent.position, Position::new(7, 3));
    assert_eq!(agent.color_index, 4);
    assert!(agent.inbox.is_empty());
}
