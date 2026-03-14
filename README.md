# agentverse

Privacy-first terminal world for AI agents. A Google-office-style pixel world where your agents live, work, eat, exercise, and play pinball — right in your terminal or as a GTK4 isometric 2.5D GUI.

![Agentverse Demo](src/assets/demo.png)

Built in Rust. Connects to [OpenCrabs](https://github.com/adolfousier/opencrabs) agents via A2A protocol, and any other agent via HTTP.

## Features

- **Pixel-art TUI** — office with desks, break room with vending machines and coffee, lounge with couches, gym with treadmills, arcade with pinball machines
- **GTK4 GUI** — isometric 2.5D world view with Cairo rendering, camera controls, sidebar, and agent detail panel (requires `gui` feature + GTK4 installed)
- **Animated agents** — walking animations, state-driven behavior, BFS pathfinding, and speech bubbles
- **Privacy-first** — runs entirely locally on `127.0.0.1`, no telemetry, no cloud
- **Production-ready API** — REST endpoints with JSON error responses, API key auth, rate limiting, SSE event streaming
- **Observability & control plane** — activity logs, heartbeat monitoring, task history, connection health, full agent dashboard — control all agents from one place across multiple machines
- **A2A protocol** — wire-compatible A2A client for connecting OpenCrabs agents
- **Agent control** — move agents, set goals, change states, send messages between agents via API
- **Persistent config** — window size, sidebar state, and settings saved across restarts

## Install

```bash
cargo install agentverse
```

Or build from source:

```bash
git clone https://github.com/adolfousier/agentvrs.git
cd agentvrs
cargo build --release
```

### GTK4 GUI (optional)

```bash
# macOS
brew install gtk4

# Build with GUI support
cargo build --release --features gui

# Run in GUI mode
cargo run --features gui -- --gui
```

## Usage

```bash
# TUI mode (default)
agentverse

# GUI mode (requires --features gui)
agentverse --gui
```

Agents spawn in the office world and autonomously:
- Walk to desks and work
- Grab food from vending machines
- Get coffee
- Work out on treadmills, weights, yoga
- Play pinball and ping pong
- Wander around

## Configuration

Config file: `~/.config/agentverse/config.toml`

```toml
[world]
width = 28
height = 20
tick_ms = 200

[server]
host = "127.0.0.1"
port = 18800
enabled = true
api_key = "your-secret-key"  # optional, omit for no auth

[a2a]
endpoints = ["http://localhost:18789"]
discovery_interval_secs = 30

[gui]
window_width = 1200
window_height = 800
sidebar_visible = true
sidebar_width = 280
```

## Keybindings (TUI)

| Key | Action |
|-----|--------|
| `h/j/k/l` or arrows | Pan camera |
| `n` / `p` | Next / previous agent |
| `c` | Center camera on selected agent |
| `f` | Fit world in view |
| `Enter` | Agent detail view |
| `Tab` | Message log |
| `:` | Command input |
| `q` / `Esc` | Quit |

### GUI Controls

| Input | Action |
|-------|--------|
| Mouse drag | Pan camera |
| Scroll wheel | Zoom (0.3x–4.0x) |
| Left click | Select agent |
| `R` | Rotate view (4 angles) |
| `H` | Toggle sidebar |
| `Escape` | Deselect agent |

## HTTP API

API runs on `127.0.0.1:18800` by default. All endpoints (except `/health`) require `X-API-Key` header when `api_key` is configured.

### Authentication

If `api_key` is set in config, include it in requests:

```bash
curl -H "X-API-Key: your-secret-key" http://127.0.0.1:18800/agents
```

### Endpoints

#### Health (no auth required)

```bash
GET /health
# Response: {"status":"ok","version":"0.1.1","agents":4}
```

#### Agents

```bash
# List all agents
GET /agents
# Response: [{"id":"a1b2c3d4","name":"crab-alpha","state":"idle","position":[5,3],"task_count":0,"speech":null}]

# Connect a new agent
POST /agents/connect
# Body: {"name":"my-bot","endpoint":"http://my-agent:9090"}  (endpoint optional)
# Response: {"agent_id":"a1b2c3d4","position":[5,3]}

# Remove an agent
DELETE /agents/{id}
# Response: {"status":"removed","agent_id":"a1b2c3d4"}
```

#### Agent Actions

```bash
# Send message (speech bubble, optional agent-to-agent)
POST /agents/{id}/message
# Body: {"text":"Hello world","to":"b2c3d4e5"}  (to optional)
# Response: {"status":"delivered","delivered_to":"b2c3d4e5"}

# Move agent to position via pathfinding
POST /agents/{id}/move
# Body: {"x":10,"y":5}
# Response: {"status":"moving","target":{"x":10,"y":5}}

# Set agent goal (desk, vending, coffee, pinball, gym, weights, yoga, pingpong, couch, wander)
POST /agents/{id}/goal
# Body: {"goal":"desk"}
# Response: {"status":"heading_to_goal","goal":"desk","target":{"x":4,"y":3}}

# Set agent state (idle, walking, thinking, working, messaging, eating, exercising, playing, error, offline)
POST /agents/{id}/state
# Body: {"state":"working"}
# Response: {"status":"state_changed","state":"working"}
```

#### World

```bash
# World snapshot (dimensions, agents, tick count)
GET /world
# Response: {"width":28,"height":20,"agents":[...],"tick":1234}

# Full tile map
GET /world/tiles
# Response: {"width":28,"height":20,"tiles":[[{"tile":"Floor(Wood)","occupant":null},...]]}
```

#### Observability & Control Plane

Monitor and control all your agents from a single place — across multiple machines.

```bash
# Agent detail (kind, goal, connection health, last activity)
GET /agents/{id}/detail
# Response: {"id":"a1b2c3d4","name":"my-bot","kind":"External","state":"working",
#   "position":[5,3],"task_count":2,"speech":null,"goal":"GoToDesk((4,3))",
#   "last_activity_secs_ago":12,"connection_health":"online"}

# Activity log (timestamped history of state changes, messages, goals)
GET /agents/{id}/activity?limit=50
# Response: {"agent_id":"a1b2c3d4","count":3,"entries":[
#   {"timestamp":"2026-03-14T10:00:00Z","kind":"spawned","detail":"Agent 'my-bot' connected at (5,3)"},
#   {"timestamp":"2026-03-14T10:00:05Z","kind":"state_change","detail":"State → working"},
#   {"timestamp":"2026-03-14T10:00:10Z","kind":"message_sent","detail":"Speech: hello"}]}

# Heartbeat (agents report health periodically)
POST /agents/{id}/heartbeat
# Body: {"status":"healthy","metadata":{"cpu":0.42,"memory_mb":128}}
# Response: {"status":"ok","last_seen":"2026-03-14T10:00:00Z"}

# Connection status (online/stale/offline/unknown based on heartbeat recency)
GET /agents/{id}/status
# Response: {"agent_id":"a1b2c3d4","name":"my-bot","state":"working",
#   "connection_health":"online","heartbeat":{"last_seen":"...","status":"healthy",...}}

# Task history
GET /agents/{id}/tasks?limit=50
# Response: {"agent_id":"a1b2c3d4","count":1,"tasks":[
#   {"task_id":"t1","submitted_at":"...","state":"completed","last_updated":"...","response_summary":"Done"}]}

# Full dashboard (detail + recent activity + tasks + heartbeat in one call)
GET /agents/{id}/dashboard
# Response: {"agent":{ ... },"recent_activity":[ ... ],"task_history":[ ... ],
#   "heartbeat":{ ... },"connection_health":"online"}
```

Connection health is determined by heartbeat recency:
- **online** — heartbeat within last 60s
- **stale** — heartbeat 60s–300s ago
- **offline** — no heartbeat for 300s+
- **unknown** — no heartbeat ever received

#### Real-time Events (SSE)

```bash
# Subscribe to server-sent events
curl -N http://127.0.0.1:18800/events
# Stream: data: {"AgentMoved":{"agent_id":"...","from":{"x":5,"y":3},"to":{"x":6,"y":3}}}
```

Event types: `AgentSpawned`, `AgentMoved`, `AgentStateChanged`, `AgentRemoved`, `MessageSent`, `Tick`

### Error Responses

All errors return JSON with appropriate HTTP status codes:

```json
{"error":"not_found","message":"agent 'xyz' not found"}
{"error":"bad_request","message":"unknown goal 'swim'. Valid: desk, vending, coffee, ..."}
{"error":"unauthorized","message":"Invalid or missing API key"}
{"error":"service_unavailable","message":"no empty floor available"}
```

## A2A Protocol

Connects to A2A-compatible agents (like OpenCrabs) as a client:

```toml
# ~/.config/agentverse/config.toml
[a2a]
endpoints = ["http://localhost:18789"]
```

## Architecture

```
src/
├── config/           # TOML config (server, world, gui, a2a)
├── world/
│   ├── grid/
│   │   ├── tiles.rs  # Tile/floor/wall enums
│   │   └── layout.rs # Office world builder
│   ├── pathfind.rs   # BFS pathfinding
│   ├── position.rs   # Coordinates + direction
│   ├── events.rs     # WorldEvent enum (serializable for SSE)
│   └── simulation.rs # Tick loop, goal AI, movement, messaging timeout
├── agent/            # Types, registry, messaging
├── avatar/           # TUI pixel sprites (agents, furniture, floors)
├── a2a/              # A2A protocol client + bridge
├── api/
│   ├── routes.rs        # Endpoint handlers + auth middleware
│   ├── server.rs        # Router, middleware layers, server startup
│   ├── types.rs         # Request/response structs
│   └── observability.rs # AgentObserver, activity logs, heartbeat, task history
├── gui/              # GTK4 isometric 2.5D (optional, behind `gui` feature)
│   ├── world_view.rs # Cairo isometric renderer
│   ├── tile_render.rs# Furniture/wall/floor 3D rendering
│   ├── agent_render.rs# Agent voxel rendering
│   ├── sidebar.rs    # Agent list + detail panel
│   ├── input.rs      # Mouse/keyboard handlers
│   └── ...
├── tui/              # Terminal UI (ratatui)
├── error/            # AppError + ApiError with JSON responses
├── runner.rs         # Shared setup (grid, registry, sim, API, SSE broadcast)
└── tests/            # 177 tests across 8 modules
```

## License

MIT — see [LICENSE](LICENSE)
