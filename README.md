# agentverse

Privacy-first terminal world for AI agents. A Google-office-style pixel world where your agents live, work, eat, exercise, and play pinball — right in your terminal.

![Agentverse Demo](src/assets/demo.png)

Built in Rust. Connects to [OpenCrabs](https://github.com/adolfousier/opencrabs) agents via A2A protocol, and any other agent via HTTP.

## Features

- **Pixel-art world** — office with desks, break room with vending machines and coffee, lounge with couches, gym with treadmills, arcade with pinball machines
- **Animated agents** — unicode half-block sprites with walking animations, state-driven behavior, pathfinding, and speech bubbles
- **Privacy-first** — runs entirely locally on `127.0.0.1`, no telemetry, no cloud
- **A2A protocol** — wire-compatible A2A client for connecting OpenCrabs agents
- **HTTP API** — REST endpoints for non-crabs agents to connect
- **Modular architecture** — small focused files, each module < 100 lines

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

## Usage

```bash
agentverse
```

Agents spawn in the office world and autonomously:
- Walk to desks and work
- Grab food from vending machines
- Get coffee
- Work out on treadmills
- Play pinball
- Wander around

## Keybindings

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

## HTTP API

API runs on `127.0.0.1:18800` by default.

```bash
# Connect an agent
curl -X POST http://127.0.0.1:18800/agents/connect \
  -H "Content-Type: application/json" \
  -d '{"name": "my-bot"}'

# List agents
curl http://127.0.0.1:18800/agents

# World snapshot
curl http://127.0.0.1:18800/world
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
├── config/           # TOML config
├── world/
│   ├── grid/
│   │   ├── tiles.rs  # Tile/floor/wall enums
│   │   └── layout.rs # Office world builder
│   ├── pathfind.rs   # BFS pathfinding
│   ├── position.rs   # Coordinates + direction
│   └── simulation.rs # Tick loop, goal AI, movement
├── agent/            # Types, registry, messaging
├── avatar/
│   ├── agents.rs     # Agent pixel sprites per state
│   ├── furniture.rs  # Desk, vending, coffee, pinball sprites
│   ├── floors.rs     # Floor/wall/rug rendering
│   ├── palette.rs    # Colors (skin, shirt, hair, floors)
│   └── sprite.rs     # SpriteFrame + StyledCell types
├── a2a/              # A2A protocol client + bridge
├── api/              # HTTP API (axum)
├── tui/
│   ├── render/       # Direct buffer pixel renderer
│   ├── input.rs      # Keybindings
│   └── app.rs        # App state + camera
├── error/
└── tests/            # 92 tests across 5 modules
```

## License

MIT — see [LICENSE](LICENSE)
