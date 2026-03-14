# agentverse

Privacy-first terminal world for AI agents. Watch your agents live, think, work, and talk — all from your terminal.

Built in Rust. Connects to [OpenCrabs](https://github.com/adolfousier/opencrabs) agents via A2A protocol, and any other agent via HTTP.

## Features

- **Terminal-native** — ratatui TUI with grid world, agent avatars, speech bubbles, and real-time state visualization
- **Privacy-first** — runs entirely locally, binds to `127.0.0.1`, no telemetry, no cloud
- **A2A protocol** — full wire-compatible A2A client for connecting OpenCrabs agents (agent discovery, message/send, tasks/get, tasks/cancel)
- **HTTP API** — REST endpoints for non-crabs agents to connect, register, and communicate
- **Modular architecture** — clean separation: world engine, agent registry, A2A bridge, API server, TUI renderer

## Install

```bash
cargo install agentverse
```

Or build from source:

```bash
git clone https://github.com/adolfousier/agentverse.git
cd agentverse
cargo build --release
```

## Usage

```bash
# Run with defaults (32x16 grid, API on 127.0.0.1:18800)
agentverse

# With a config file
mkdir -p ~/.config/agentverse
cat > ~/.config/agentverse/config.toml << 'EOF'
[world]
width = 48
height = 24
tick_ms = 150

[server]
host = "127.0.0.1"
port = 18800
enabled = true

[a2a]
endpoints = ["http://localhost:18789"]
discovery_interval_secs = 30
EOF

agentverse
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `Ctrl+C` | Quit |
| `j` / `Down` | Select next agent |
| `k` / `Up` | Select previous agent |
| `Enter` | View agent details |
| `Tab` | Toggle message log |
| `:` | Command input mode |
| `Backspace` | Back to world view |

## HTTP API

When `server.enabled = true`, these endpoints are available:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/agents` | List all agents |
| `POST` | `/agents/connect` | Register a new agent |
| `POST` | `/agents/{id}/message` | Send message to agent |
| `GET` | `/world` | World state snapshot |

### Connect an external agent

```bash
curl -X POST http://127.0.0.1:18800/agents/connect \
  -H "Content-Type: application/json" \
  -d '{"name": "my-bot", "endpoint": "http://localhost:9090"}'
```

## A2A Protocol

Agentverse acts as an A2A **client** — it discovers and communicates with A2A-compatible agents (like OpenCrabs). Configure agent endpoints in `config.toml`:

```toml
[a2a]
endpoints = ["http://localhost:18789", "http://192.168.1.50:18789"]
```

Agentverse will:
1. Fetch the Agent Card from `/.well-known/agent.json`
2. Spawn a visual avatar on the grid
3. Map A2A task states to visual states (Working, Thinking, Idle, Error)
4. Display agent responses as speech bubbles

## Architecture

```
src/
├── config/      # TOML config loading
├── world/       # Grid, positions, simulation tick loop
├── agent/       # Agent types, registry, messaging
├── avatar/      # Sprites, color palettes
├── a2a/         # A2A protocol types, client, bridge
├── api/         # HTTP API server (axum)
├── tui/         # Terminal UI (ratatui + crossterm)
│   └── render/  # World view, sidebar, status bar, detail panel
├── error/       # Error types
└── tests/       # Integration tests
```

## License

MIT — see [LICENSE](LICENSE)
