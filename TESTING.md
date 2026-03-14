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
cargo test tests::config_test
cargo test tests::simulation_test
cargo test tests::observability_test

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
├── mod.rs              # Module declarations
├── world_test.rs       # Grid, position, pathfinding, tiles, layout, events
├── agent_test.rs       # Agent types, registry, messaging
├── a2a_test.rs         # A2A protocol wire compatibility
├── api_test.rs         # HTTP API endpoint tests (tower::ServiceExt)
├── tui_test.rs         # TUI state machine and input handling
├── config_test.rs         # Configuration loading, saving, serialization
├── simulation_test.rs     # Simulation tick loop, agent behavior, state transitions
└── observability_test.rs  # Observability endpoints (detail, activity, heartbeat, status, tasks, dashboard)
```

## Test Coverage by Module

### api_test.rs (33 tests)

**Health & CRUD:**
- Health endpoint response
- List agents (empty + after connect)
- Connect agent (local, external, multiple)
- Delete agent (success, nonexistent, grid cleanup)

**Messaging:**
- Self-message (speech bubble)
- Agent-to-agent message with `to` field
- Message to nonexistent agent/target (404)

**Agent Actions:**
- Move agent to position via pathfinding
- Move to wall (400) / out of bounds (400)
- Set goal: wander, invalid goal (400)
- Set state: all 10 valid states, invalid state (400)
- Set idle clears path and goal

**World:**
- World snapshot (empty + with agents)
- World tiles (grid dimensions, wall/floor detection)

**Auth:**
- API key required when configured
- Wrong API key rejected
- Correct API key accepted
- Health bypasses auth
- No auth when key not configured

**Error Responses:**
- JSON error body format validation (`{"error":"...","message":"..."}`)

**Misc:**
- Agent ID prefix matching (short ID lookup)
- SSE event stream endpoint exists (content-type check)

### world_test.rs (44 tests)

**Position:**
- Creation, movement in all 4 directions
- Boundary clamping (all edges)
- Distance calculation (Pythagorean, same point)

**Grid:**
- Creation (empty and walled)
- Out-of-bounds access
- Agent placement (floor, wall, occupied)
- Agent removal (existing, empty)
- Agent movement (valid, to wall, to occupied, same position)
- Empty floor finding
- Grid bounds accessor
- Cells accessor
- Tile setting (wall, furniture)

**Tiles:**
- `is_solid` for all floor types (Wood, Tile, Carpet, Concrete)
- `is_solid` for walkable specials (Rug, DoorOpen)
- `is_solid` for all wall types
- `is_solid` for all 15 furniture types

**find_tiles:**
- Empty result for missing tile type
- Finds placed tiles
- Distinguishes between tile types

**find_adjacent_floor:**
- Basic adjacency
- Prefers LEFT face (-x direction)
- Avoiding taken spots
- Fallback when all preferred spots avoided
- Returns None when surrounded by walls

**Pathfinding:**
- Same position (empty path)
- Adjacent cell (1 step)
- Straight line
- Around wall obstacles
- No path available
- Path excludes start, includes end
- Avoids occupied cells
- Target occupied still works

**Office World Layout:**
- Correct dimensions
- Perimeter walls
- Has furniture (desks, vending, coffee)
- Has walkable space
- Ping pong table (2 adjacent tiles, correct orientation)

**World Events:**
- Tick serialization
- AgentSpawned serialization
- AgentMoved serialization
- MessageSent serialization

### simulation_test.rs (8 tests)

- Tick emits WorldEvent::Tick
- Tick count increments correctly
- Shared tick counter updates atomically
- Messaging auto-transition to Idle after 30 ticks (speech cleared)
- Idle agent eventually gets assigned a goal
- Walking agent moves along path
- Activity state tracks tick count past minimum
- Multiple agents handled without conflicts

### agent_test.rs (18 tests)

- AgentId generation and uniqueness
- AgentId display (8-char prefix)
- Agent creation, state transitions, speech
- Agent clear speech
- State label mapping (all states)
- AgentKind variants (OpenCrabs, External)
- Registry CRUD (new, register, remove, get_mut)
- Registry search by name
- Registry IDs listing
- Registry agents iterator
- MessageLog push, recent, count, overflow

### a2a_test.rs (17 tests)

- TaskState serialization (camelCase wire format)
- All task states roundtrip
- Role serialization
- Message roundtrip (serialize + deserialize)
- JSON-RPC request/response/error construction
- AgentCard full and minimal serialization
- Task with status and artifacts
- SendMessageParams serialization
- Wire compatibility: camelCase, skip_serializing_if
- Error codes constants
- AgentSkill serialization

### config_test.rs (8 tests)

- Default config values (world, server, a2a, gui)
- GUI config defaults
- Parse from minimal TOML (empty string)
- Parse from partial TOML (overrides + defaults)
- Config roundtrip (serialize → deserialize)
- Config save and load (filesystem)
- API key not serialized when None
- API key serialized when set

### tui_test.rs (16 tests)

- App initial state
- Quit keybindings (q, Esc, Ctrl+C)
- Mode switching (Tab, :, Enter, Esc, Backspace)
- Command input typing and clearing
- Agent list navigation (j/k, Up/Down)
- Boundary behavior (navigate up at index 0)

### observability_test.rs (11 tests)

**Detail:**
- Agent detail response (name, state, kind)
- Detail for nonexistent agent (404)

**Activity:**
- Connect records spawned activity
- Activity limit query parameter
- State change recorded in activity log
- Message recorded in activity log

**Heartbeat:**
- Post heartbeat with metadata

**Status:**
- Status with heartbeat (online health)
- Status without heartbeat (unknown health)

**Tasks:**
- Empty task history

**Dashboard:**
- Full dashboard aggregation (detail + activity + health)

## Writing New Tests

1. Add tests to the appropriate `*_test.rs` file
2. For async tests, use `#[tokio::test]`
3. For API tests, use the `test_state()` helper with `tower::ServiceExt::oneshot`
4. For TUI tests, use the `test_app()` and `key()` helpers
5. For simulation tests, use `setup_sim()` and `spawn_agent()` helpers
6. For config tests, use `tempfile` for filesystem operations

## Test Helpers

### API Tests
- `test_state()` — Creates router with no auth, 16x12 walled grid
- `test_state_with_auth(key)` — Creates router with API key auth enabled
- `connect_helper(router, name)` — Connects an agent, returns agent ID string

### Observability Tests
- `test_state()` — Creates router with observer, no auth, 16x12 walled grid
- `connect(router, name)` — Connects an agent, returns agent ID string
- `set_state(router, id, state)` — Sets agent state via API

### Simulation Tests
- `setup_sim()` — Creates simulation with 28x20 office world
- `spawn_agent(registry, grid, name)` — Spawns agent on empty floor, returns AgentId

## CI

Tests run automatically on every push and PR via GitHub Actions. See `.github/workflows/ci.yml`.
