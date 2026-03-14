# Testing Guide

## Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test tests::world_test
cargo test tests::agent_test
cargo test tests::a2a_test
cargo test tests::api_test
cargo test tests::tui_test

# Single test
cargo test tests::world_test::test_grid_move_agent

# With output
cargo test -- --nocapture

# Run only ignored/integration tests
cargo test -- --ignored
```

## Test Architecture

All tests live under `src/tests/` with modular organization:

```
src/tests/
├── mod.rs           # Module declarations
├── world_test.rs    # Grid, position, simulation tests
├── agent_test.rs    # Agent types, registry, messaging tests
├── a2a_test.rs      # A2A protocol wire compatibility tests
├── api_test.rs      # HTTP API endpoint tests (tower::ServiceExt)
└── tui_test.rs      # TUI state machine and input handling tests
```

## Test Coverage by Module

### world_test.rs
- Position creation, movement, boundary clamping
- Distance calculation
- Grid creation (empty and walled)
- Agent placement, removal, movement on grid
- Collision detection (walls, occupied cells)
- Tile types and walkability
- Empty floor finding

### agent_test.rs
- AgentId generation and display
- Agent creation, state transitions, speech
- AgentKind variants
- AgentRegistry CRUD operations
- Registry search by name
- MessageLog push, recent, count

### a2a_test.rs
- TaskState serialization (camelCase wire format)
- All task states roundtrip
- Role serialization
- Message roundtrip (serialize + deserialize)
- JSON-RPC request/response construction
- AgentCard full and minimal serialization
- Task with status and artifacts
- SendMessageParams serialization
- Wire compatibility: camelCase, skip_serializing_if
- Error codes constants
- AgentSkill serialization

### api_test.rs
- Health endpoint response
- List agents (empty)
- Connect agent (local and external)
- World snapshot
- Message to nonexistent agent (404)

### tui_test.rs
- App initial state
- Quit keybindings (q, Esc, Ctrl+C)
- Mode switching (Tab, :, Enter, Esc, Backspace)
- Command input typing and clearing
- Agent list navigation (j/k, Up/Down)
- Boundary behavior (navigate up at index 0)

## Writing New Tests

1. Add tests to the appropriate `*_test.rs` file
2. For async tests, use `#[tokio::test]`
3. For API tests, use the `test_state()` helper with `tower::ServiceExt::oneshot`
4. For TUI tests, use the `test_app()` and `key()` helpers

## CI

Tests run automatically on every push and PR via GitHub Actions. See `.github/workflows/ci.yml`.
