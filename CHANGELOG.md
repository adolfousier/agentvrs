# Changelog

## [0.1.2] - 2026-03-15

### Added

**Bevy 3D Renderer**
- Full 3D world renderer using Bevy 0.15 (`--bevy` flag, `bevy3d` feature)
- Isometric camera with orbit controls (right-click drag to rotate, scroll to zoom)
- 3D furniture: desks with monitors and keyboards, vending machines, arcade cabinets, coffee machines, couches, armchairs, plants, floor lamps, whiteboards, kitchen counters, gym equipment
- Voxel-style 3D agents with body, head, hair, arms, legs, eyes, and shadow
- Agent color coding and floating name labels
- Walking animation with arm/leg swing and body bob
- Activity animations: typing (working), eating motion, exercise bounce, playing
- Click-to-select agents with highlight ring
- Sidebar overlay with agent list and detail panel
- Message input box for sending messages to selected agents
- Real-time sync between simulation state and 3D scene

**Meeting Table**
- Round meeting table with 4 chairs at cardinal positions
- Capacity system: meeting table supports 4 simultaneous agents
- `GoToMeeting` agent goal for meeting table destinations

### Changed
- Simulation runs inside Bevy's game loop as a chained system (eliminates lock contention between render and sim threads)
- Furniture faces camera (+z direction) for proper isometric viewing
- Desk monitors face camera with keyboard in front
- Agent adjacency placement prefers front-facing positions at furniture
- README updated from GTK4 2D to Bevy 3D throughout
- Crate description updated for crates.io

### Fixed
- Lock starvation causing agents to permanently freeze (moved sim into Bevy game loop)
- Agent movement deadlocks from multiple competing write locks
- Furniture orientation: vending machines, arcade cabinets now face camera
- Agents no longer stop at sides of desks (improved adjacent floor placement)
- Floating agent labels use colored dots instead of emoji (cross-platform compatibility)
- Sidebar ghost text artifacts cleared
- Removed all dead code warnings (unused struct fields, dead ping pong rendering)

### Removed
- Ping pong table (replaced by meeting table)
- `PingPongTableLeft`/`PingPongTableRight` tile variants

## [0.1.1] - 2026-03-15

### Added

**Production-Ready API**
- API key authentication via `X-API-Key` header — required for all endpoints except `/health`
- API key redacted from all debug output and logs (`[REDACTED]`)
- Server refuses to start without `api_key` configured
- Rate limiting with `ConcurrencyLimitLayer`
- JSON error responses (`ApiError` enum → `{"error":"...","message":"..."}`)
- SSE real-time event streaming (`GET /events`)
- Agent control endpoints: move, goal, state, agent-to-agent messaging
- Agent disconnect endpoint (`DELETE /agents/{id}`)
- World tile map endpoint (`GET /world/tiles`)
- Agent ID prefix matching for convenience
- Crate-level documentation for crates.io landing page

**Agent Inbox & Messaging**
- In-memory inbox per agent (`VecDeque<AgentMessage>`, capped at 500 messages)
- `GET /agents/{id}/messages` — poll inbox for received messages (with `?limit=N`)
- `POST /agents/{id}/messages/ack` — clear inbox after reading
- Webhook push delivery — messages auto-POST to agent's registered endpoint
- TUI detail panel shows inbox messages when agent is selected
- Messages include sender ID, sender name, text, and timestamp

**Observability & Control Plane**
- `AgentObserver` with ring-buffered activity logs, heartbeat tracking, task history
- 6 new endpoints: `/agents/{id}/detail`, `/activity`, `/heartbeat`, `/status`, `/tasks`, `/dashboard`
- Connection health tracking (online/stale/offline/unknown) based on heartbeat recency
- All API actions automatically record timestamped activity entries

**GTK4 GUI**
- Isometric 2.5D world view with Cairo rendering (`--gui` flag, requires `gui` feature)
- Isometric furniture: desks, vending machines, arcade cabinets, coffee machines, yoga mats, gym equipment, kitchen counters, ping pong tables
- Agent rendering with voxel-style bodies, name labels, and speech bubbles
- Camera controls: mouse drag to pan, scroll to zoom, R to rotate (4 angles)
- Sidebar with agent list and detail panel (toggle with H key)
- Status bar with tick count, agent count, and zoom level
- Ping pong table spans 2 grid tiles for realistic rectangular shape
- Furniture capacity system (ping pong: 2 agents, others: 1)
- Window size and sidebar state persist across restarts via config file

**Testing & Docs**
- 184 tests across 8 modules (api, observability, world, agent, a2a, simulation, tui, config)
- 8 inbox tests: empty, receives, multiple messages, limit, ack, self-message, 500 cap, multiple senders
- Agent integration guides: OpenCrabs (Rust), OpenClaws (Python), Hermes (TypeScript), generic HTTP
- Multi-machine setup documentation
- TESTING.md with full coverage breakdown
- README with badges, table of contents, full API docs with auth examples

### Changed
- Compact world layout: 28x20 grid with thin walls and no wasted space
- Agents stay at activities longer (working: 120 ticks, eating: 50, playing: 80, exercising: 90)
- Furniture details rendered on both isometric faces for visibility from any angle
- Agents front-face furniture when stopped at activity locations
- Messaging agents auto-transition back to Idle after 30 ticks
- Shared atomic tick counter between simulation and API for accurate world snapshots
- `api_key` changed from optional to required in `ServerConfig`
- `reqwest` uses `rustls-tls` for crates.io compatibility

### Fixed
- Kitchen furniture flickering during pan (switched from screen coords to grid coords for variant hashing)
- Duplicate speech log entries in sidebar
- Keyboard shortcuts (H, R, Escape) not working when sidebar has focus
- Vending machines no longer block hallway passage
- Agents no longer pile up on same furniture spot
- Removed non-functional ctrl+shift+scroll rotate binding
- Removed dead code (`ViewState::new()`, `Camera::rotate_ccw()`)
- Zero clippy warnings with `--all-features`
- Fixed flaky walking agent test (uses pathfinding for valid targets)

## [0.1.0] - 2025-12-01

### Added
- Initial agentverse scaffold
- TUI world with animated agents and BFS pathfinding
- A2A client for connecting OpenCrabs agents
- HTTP API for external agent integration
- Pixel-art world with camera system
- Dynamic world sizing and dark theme
