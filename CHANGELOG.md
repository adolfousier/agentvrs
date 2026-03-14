# Changelog

## [0.1.1] - 2026-03-14

### Added

**Production-Ready API**
- API key authentication via `X-API-Key` header middleware
- Rate limiting with `ConcurrencyLimitLayer`
- JSON error responses (`ApiError` enum → `{"error":"...","message":"..."}`)
- SSE real-time event streaming (`GET /events`)
- Agent control endpoints: move, goal, state, agent-to-agent messaging
- Agent disconnect endpoint (`DELETE /agents/{id}`)
- World tile map endpoint (`GET /world/tiles`)
- Agent ID prefix matching for convenience

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
- 177 tests across 8 modules (api, observability, world, agent, a2a, simulation, tui, config)
- Agent integration guides: OpenCrabs (Rust), OpenClaws (Python), Hermes (TypeScript), generic HTTP
- Multi-machine setup documentation
- TESTING.md with full coverage breakdown

### Changed
- Compact world layout: 28x20 grid with thin walls and no wasted space
- Agents stay at activities longer (working: 120 ticks, eating: 50, playing: 80, exercising: 90)
- Furniture details rendered on both isometric faces for visibility from any angle
- Agents front-face furniture when stopped at activity locations
- Messaging agents auto-transition back to Idle after 30 ticks
- Shared atomic tick counter between simulation and API for accurate world snapshots

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
