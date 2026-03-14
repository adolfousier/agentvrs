# Contributing to agentverse

Thanks for your interest in contributing!

## Getting Started

1. Fork the repo and clone your fork
2. Install Rust stable (1.91+): `rustup update stable`
3. Build: `cargo build`
4. Run tests: `cargo test`

## Development

```bash
# Build
cargo build

# Run
cargo run

# Run tests
cargo test

# Run with logging
RUST_LOG=agentverse=debug cargo run

# Check formatting
cargo fmt --check

# Lint
cargo clippy -- -D warnings
```

## Pull Requests

- Keep PRs focused — one feature or fix per PR
- Add tests for new functionality (see `src/tests/`)
- Run `cargo fmt` and `cargo clippy` before submitting
- Write clear commit messages

## Module Guide

| Module | Purpose |
|--------|---------|
| `config/` | TOML config types and loading |
| `world/` | Grid engine, positions, simulation loop |
| `agent/` | Agent types, registry, inter-agent messaging |
| `avatar/` | Sprite definitions and color palettes |
| `a2a/` | A2A protocol types, HTTP client, bridge layer |
| `api/` | REST API for external agents (axum) |
| `tui/` | Terminal UI rendering and input handling |
| `error/` | Error types |
| `tests/` | Integration and unit tests |

## Testing

See [TESTING.md](TESTING.md) for full testing guide.

## Code of Conduct

Be respectful. Be constructive. Ship good code.
