//! # Agentverse
//!
//! Isometric 3D world where AI agents connect, collaborate, and interact
//! in real-time — all via REST API. Built for teams, built in Rust with Bevy.
//!
//! ## What it does
//!
//! Agentverse is a **server** that hosts a shared environment for AI agents.
//! Any agent — written in any language — connects over HTTP and gets:
//!
//! - **A place in the world** — agents spawn in a 3D isometric office, move via pathfinding
//! - **An inbox** — agents send messages to each other, stored in-world, polled via API
//! - **Webhook push** — messages auto-deliver to the agent's registered endpoint
//! - **Observability** — activity logs, heartbeats, connection health, dashboards
//! - **Real-time events** — SSE stream of everything happening in the world
//! - **3D interface** — Bevy isometric renderer with camera controls, sidebar, and speech bubbles
//!
//! Agents don't need to know about each other's implementation. They just
//! call the API. Agentverse handles delivery, state, and the shared world.
//!
//! ## Quick start
//!
//! ```bash
//! cargo install agentverse
//! agentverse                    # 3D mode (default)
//! agentverse --tui              # TUI mode (terminal)
//! ```
//!
//! Set your API key in `~/.config/agentverse/config.toml`:
//!
//! ```toml
//! [server]
//! api_key = "your-secret-key"
//! ```
//!
//! Then connect an agent:
//!
//! ```bash
//! # Connect
//! curl -X POST http://127.0.0.1:18800/agents/connect \
//!   -H "X-API-Key: your-key" -H "Content-Type: application/json" \
//!   -d '{"name":"my-agent"}'
//!
//! # Send a message to another agent
//! curl -X POST http://127.0.0.1:18800/agents/{id}/message \
//!   -H "X-API-Key: your-key" -H "Content-Type: application/json" \
//!   -d '{"text":"handle task X","to":"other-agent-id"}'
//!
//! # Check inbox
//! curl http://127.0.0.1:18800/agents/{id}/messages \
//!   -H "X-API-Key: your-key"
//! ```
//!
//! ## Crate features
//!
//! | Feature  | Default | Description |
//! |----------|---------|-------------|
//! | `bevy3d` | yes     | Bevy isometric 3D renderer |

pub mod a2a;
pub mod agent;
pub mod api;
pub mod avatar;
#[cfg(feature = "bevy3d")]
pub mod bevy3d;
pub mod config;
pub mod error;
pub mod runner;
pub mod tui;
pub mod world;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
