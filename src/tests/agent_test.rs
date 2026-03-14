use crate::agent::*;
use crate::world::Position;

#[test]
fn test_agent_id_new() {
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    assert_ne!(id1, id2);
}

#[test]
fn test_agent_id_display() {
    let id = AgentId::new();
    let display = format!("{}", id);
    assert_eq!(display.len(), 8); // short UUID prefix
}

#[test]
fn test_agent_new() {
    let agent = Agent::new("test-agent", AgentKind::Local, Position::new(5, 5));
    assert_eq!(agent.name, "test-agent");
    assert_eq!(agent.state, AgentState::Idle);
    assert_eq!(agent.position, Position::new(5, 5));
    assert_eq!(agent.task_count, 0);
    assert!(agent.speech.is_none());
}

#[test]
fn test_agent_set_state() {
    let mut agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    agent.set_state(AgentState::Working);
    assert_eq!(agent.state, AgentState::Working);
}

#[test]
fn test_agent_say() {
    let mut agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    agent.say("hello world");
    assert_eq!(agent.speech.as_deref(), Some("hello world"));
}

#[test]
fn test_agent_clear_speech() {
    let mut agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    agent.say("hello");
    agent.clear_speech();
    assert!(agent.speech.is_none());
}

#[test]
fn test_agent_state_label() {
    assert_eq!(AgentState::Idle.label(), "idle");
    assert_eq!(AgentState::Thinking.label(), "thinking");
    assert_eq!(AgentState::Working.label(), "working");
    assert_eq!(AgentState::Messaging.label(), "messaging");
    assert_eq!(AgentState::Error.label(), "error");
    assert_eq!(AgentState::Offline.label(), "offline");
}

#[test]
fn test_agent_kind_opencrabs() {
    let kind = AgentKind::OpenCrabs {
        endpoint: "http://localhost:18789/a2a/v1".to_string(),
    };
    assert!(matches!(kind, AgentKind::OpenCrabs { .. }));
}

#[test]
fn test_agent_kind_external() {
    let kind = AgentKind::External {
        endpoint: "http://other:8080".to_string(),
    };
    assert!(matches!(kind, AgentKind::External { .. }));
}

// ─── Registry Tests ──────────────────────────────────────────

#[test]
fn test_registry_new() {
    let registry = AgentRegistry::new();
    assert_eq!(registry.count(), 0);
}

#[test]
fn test_registry_register() {
    let mut registry = AgentRegistry::new();
    let agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    let id = agent.id;
    registry.register(agent);
    assert_eq!(registry.count(), 1);
    assert!(registry.get(&id).is_some());
}

#[test]
fn test_registry_remove() {
    let mut registry = AgentRegistry::new();
    let agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    let id = agent.id;
    registry.register(agent);
    let removed = registry.remove(&id);
    assert!(removed.is_some());
    assert_eq!(registry.count(), 0);
}

#[test]
fn test_registry_remove_nonexistent() {
    let mut registry = AgentRegistry::new();
    let id = AgentId::new();
    assert!(registry.remove(&id).is_none());
}

#[test]
fn test_registry_get_mut() {
    let mut registry = AgentRegistry::new();
    let agent = Agent::new("test", AgentKind::Local, Position::new(0, 0));
    let id = agent.id;
    registry.register(agent);
    if let Some(a) = registry.get_mut(&id) {
        a.set_state(AgentState::Working);
    }
    assert_eq!(registry.get(&id).unwrap().state, AgentState::Working);
}

#[test]
fn test_registry_find_by_name() {
    let mut registry = AgentRegistry::new();
    let agent = Agent::new("special-agent", AgentKind::Local, Position::new(0, 0));
    registry.register(agent);
    assert!(registry.find_by_name("special-agent").is_some());
    assert!(registry.find_by_name("nonexistent").is_none());
}

#[test]
fn test_registry_ids() {
    let mut registry = AgentRegistry::new();
    let a1 = Agent::new("one", AgentKind::Local, Position::new(0, 0));
    let a2 = Agent::new("two", AgentKind::Local, Position::new(1, 1));
    let id1 = a1.id;
    let id2 = a2.id;
    registry.register(a1);
    registry.register(a2);
    let ids = registry.ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&id1));
    assert!(ids.contains(&id2));
}

#[test]
fn test_registry_agents_iterator() {
    let mut registry = AgentRegistry::new();
    registry.register(Agent::new("a", AgentKind::Local, Position::new(0, 0)));
    registry.register(Agent::new("b", AgentKind::Local, Position::new(1, 1)));
    registry.register(Agent::new("c", AgentKind::Local, Position::new(2, 2)));
    let names: Vec<_> = registry.agents().map(|a| a.name.clone()).collect();
    assert_eq!(names.len(), 3);
}

// ─── Message Tests ───────────────────────────────────────────

#[test]
fn test_agent_message_new() {
    let from = AgentId::new();
    let to = AgentId::new();
    let msg = AgentMessage::new(from, to, "hello");
    assert_eq!(msg.from, from);
    assert_eq!(msg.to, to);
    assert_eq!(msg.text, "hello");
}

#[test]
fn test_message_log_push() {
    let mut log = MessageLog::new();
    let msg = AgentMessage::new(AgentId::new(), AgentId::new(), "test");
    log.push(msg);
    assert_eq!(log.count(), 1);
}

#[test]
fn test_message_log_recent() {
    let mut log = MessageLog::new();
    for i in 0..10 {
        log.push(AgentMessage::new(
            AgentId::new(),
            AgentId::new(),
            format!("msg-{}", i),
        ));
    }
    let recent = log.recent(3);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].text, "msg-7");
    assert_eq!(recent[2].text, "msg-9");
}

#[test]
fn test_message_log_recent_more_than_available() {
    let mut log = MessageLog::new();
    log.push(AgentMessage::new(AgentId::new(), AgentId::new(), "only"));
    let recent = log.recent(100);
    assert_eq!(recent.len(), 1);
}
