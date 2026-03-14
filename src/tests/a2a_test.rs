use crate::a2a::types::*;

#[test]
fn test_part_text_creation() {
    let part = Part::text("hello world");
    assert_eq!(part.text.as_deref(), Some("hello world"));
    assert!(part.data.is_none());
    assert!(part.url.is_none());
}

#[test]
fn test_task_state_serialization() {
    let state = TaskState::Working;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, "\"working\"");
}

#[test]
fn test_task_state_deserialization() {
    let state: TaskState = serde_json::from_str("\"submitted\"").unwrap();
    assert_eq!(state, TaskState::Submitted);
}

#[test]
fn test_all_task_states_roundtrip() {
    let states = vec![
        TaskState::Submitted,
        TaskState::Working,
        TaskState::Completed,
        TaskState::Failed,
        TaskState::Canceled,
        TaskState::InputRequired,
        TaskState::Rejected,
        TaskState::AuthRequired,
    ];
    for state in states {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: TaskState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, parsed);
    }
}

#[test]
fn test_role_serialization() {
    assert_eq!(serde_json::to_string(&Role::User).unwrap(), "\"user\"");
    assert_eq!(serde_json::to_string(&Role::Agent).unwrap(), "\"agent\"");
}

#[test]
fn test_message_roundtrip() {
    let msg = Message {
        message_id: Some("msg-1".to_string()),
        context_id: None,
        task_id: None,
        role: Role::User,
        parts: vec![Part::text("Hello, agent!")],
        metadata: None,
    };
    let json = serde_json::to_string(&msg).unwrap();
    let parsed: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.role, Role::User);
    assert_eq!(parsed.parts.len(), 1);
    assert_eq!(parsed.parts[0].text.as_deref(), Some("Hello, agent!"));
}

#[test]
fn test_json_rpc_success_response() {
    let resp = JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({"status": "ok"}));
    assert_eq!(resp.jsonrpc, "2.0");
    assert!(resp.result.is_some());
    assert!(resp.error.is_none());
}

#[test]
fn test_json_rpc_error_response() {
    let resp = JsonRpcResponse::error(
        serde_json::json!(1),
        error_codes::METHOD_NOT_FOUND,
        "Method not found",
    );
    let err = resp.error.unwrap();
    assert_eq!(err.code, -32601);
    assert_eq!(err.message, "Method not found");
}

#[test]
fn test_json_rpc_request_serialization() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "message/send".to_string(),
        params: serde_json::json!({}),
        id: serde_json::json!("req-1"),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("message/send"));
    assert!(json.contains("2.0"));
}

#[test]
fn test_agent_card_serialization() {
    let card = AgentCard {
        name: "TestAgent".to_string(),
        description: Some("A test agent".to_string()),
        version: Some("0.1.0".to_string()),
        documentation_url: None,
        icon_url: None,
        supported_interfaces: vec![SupportedInterface {
            url: "http://localhost:18789/a2a/v1".to_string(),
            protocol_binding: "JSONRPC".to_string(),
            protocol_version: Some("1.0".to_string()),
        }],
        provider: Some(AgentProvider {
            organization: "OpenCrabs".to_string(),
            url: None,
        }),
        capabilities: Some(AgentCapabilities {
            streaming: false,
            push_notifications: false,
            state_transition_history: false,
        }),
        skills: vec![],
        default_input_modes: vec!["text/plain".to_string()],
        default_output_modes: vec!["text/plain".to_string()],
    };
    let json = serde_json::to_string_pretty(&card).unwrap();
    assert!(json.contains("TestAgent"));
    assert!(json.contains("JSONRPC"));

    // Roundtrip
    let parsed: AgentCard = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "TestAgent");
    assert_eq!(parsed.supported_interfaces.len(), 1);
}

#[test]
fn test_agent_card_minimal() {
    let json = r#"{"name": "minimal"}"#;
    let card: AgentCard = serde_json::from_str(json).unwrap();
    assert_eq!(card.name, "minimal");
    assert!(card.description.is_none());
    assert!(card.skills.is_empty());
}

#[test]
fn test_task_serialization() {
    let task = Task {
        id: "task-1".to_string(),
        context_id: None,
        status: TaskStatus {
            state: TaskState::Completed,
            message: Some(Message {
                message_id: None,
                context_id: None,
                task_id: None,
                role: Role::Agent,
                parts: vec![Part::text("Done!")],
                metadata: None,
            }),
            timestamp: Some("2026-03-14T00:00:00Z".to_string()),
        },
        artifacts: vec![],
        history: vec![],
        metadata: None,
    };
    let json = serde_json::to_string(&task).unwrap();
    let parsed: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "task-1");
    assert_eq!(parsed.status.state, TaskState::Completed);
}

#[test]
fn test_send_message_params() {
    let params = SendMessageParams {
        message: Message {
            message_id: Some("msg-1".to_string()),
            context_id: None,
            task_id: None,
            role: Role::User,
            parts: vec![Part::text("Hello")],
            metadata: None,
        },
        configuration: None,
        metadata: None,
    };
    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("Hello"));
}

#[test]
fn test_error_codes() {
    assert_eq!(error_codes::PARSE_ERROR, -32700);
    assert_eq!(error_codes::INVALID_REQUEST, -32600);
    assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
    assert_eq!(error_codes::INVALID_PARAMS, -32602);
    assert_eq!(error_codes::INTERNAL_ERROR, -32603);
    assert_eq!(error_codes::TASK_NOT_FOUND, -32001);
    assert_eq!(error_codes::UNSUPPORTED_OPERATION, -32003);
}

#[test]
fn test_agent_skill_serialization() {
    let skill = AgentSkill {
        id: "code-review".to_string(),
        name: "Code Review".to_string(),
        description: Some("Reviews code".to_string()),
        tags: vec!["code".to_string()],
        examples: vec!["Review this PR".to_string()],
        input_modes: vec!["text/plain".to_string()],
        output_modes: vec!["text/plain".to_string()],
    };
    let json = serde_json::to_string(&skill).unwrap();
    let parsed: AgentSkill = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "code-review");
    assert_eq!(parsed.tags.len(), 1);
}

/// Wire compatibility test: ensure our types serialize identically to OpenCrabs.
#[test]
fn test_wire_compatibility_task_state_camel_case() {
    // OpenCrabs uses camelCase for task states
    let json = serde_json::to_string(&TaskState::InputRequired).unwrap();
    assert_eq!(json, "\"inputRequired\"");

    let json = serde_json::to_string(&TaskState::AuthRequired).unwrap();
    assert_eq!(json, "\"authRequired\"");
}

#[test]
fn test_wire_compatibility_skip_serializing_if_none() {
    let part = Part::text("hello");
    let json = serde_json::to_string(&part).unwrap();
    // Optional None fields should not appear in JSON
    assert!(!json.contains("data"));
    assert!(!json.contains("url"));
    assert!(!json.contains("mediaType"));
}
