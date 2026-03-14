# Changelog

## [0.1.1] - 2026-03-14

### Added
- Agent observability & control plane: activity logs, heartbeat monitoring, task history, connection health dashboard
- New API endpoints: `GET /agents/{id}/detail`, `GET /agents/{id}/activity`, `POST /agents/{id}/heartbeat`, `GET /agents/{id}/status`, `GET /agents/{id}/tasks`, `GET /agents/{id}/dashboard`
- Connection health tracking (online/stale/offline/unknown) based on heartbeat recency
- All API actions (connect, delete, message, move, goal, state) now record timestamped activity entries
- 11 new observability tests in dedicated `observability_test.rs`
- GTK4 GUI with isometric 2.5D world view (`--gui` flag, requires `gui` feature)
- Cairo-rendered isometric tiles with 3D extrusion and shading
- Isometric furniture: desks, vending machines, arcade cabinets, coffee machines, yoga mats, gym equipment, kitchen counters, ping pong tables
- Agent rendering with voxel-style bodies, name labels, and speech bubbles
- Camera controls: mouse drag to pan, scroll to zoom, R to rotate (4 angles)
- Sidebar with agent list and detail panel (toggle with H key)
- Status bar with tick count, agent count, and zoom level
- Ping pong table spans 2 grid tiles for realistic rectangular shape
- Furniture capacity system (ping pong: 2 agents, others: 1)
- Window size and sidebar state persist across restarts via config file

### Changed
- Compact world layout: 28x20 grid with thin walls and no wasted space
- Agents stay at activities longer (working: 120 ticks, eating: 50, playing: 80, exercising: 90)
- Furniture details rendered on both isometric faces for visibility from any angle
- Agents front-face furniture when stopped at activity locations

### Fixed
- Kitchen furniture flickering during pan (switched from screen coords to grid coords for variant hashing)
- Duplicate speech log entries in sidebar
- Keyboard shortcuts (H, R, Escape) not working when sidebar has focus
- Vending machines no longer block hallway passage
- Agents no longer pile up on same furniture spot

## [0.1.0] - 2025-12-01

### Added
- Initial agentverse scaffold
- TUI world with animated agents and BFS pathfinding
- A2A client for connecting OpenCrabs agents
- HTTP API for external agent integration
- Pixel-art world with camera system
- Dynamic world sizing and dark theme
