//! # Agentverse
//!
//! Privacy-first terminal world for AI agents. Connect OpenCrabs via A2A,
//! external agents via HTTP.
//!
//! Agentverse provides a simulated office environment where AI agents coexist,
//! navigate via BFS pathfinding, and interact through the A2A protocol or a
//! REST/SSE API. It ships with a TUI (default) and an optional GTK4 isometric
//! 2.5D GUI (`--features gui`).
//!
//! ## Quick start
//!
//! ```bash
//! cargo install agentverse
//! agentverse                    # TUI mode
//! agentverse --gui              # GTK4 GUI (requires gui feature)
//! agentverse --api-key SECRET   # enable API authentication
//! ```
//!
//! ## Crate features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `gui`   | no      | GTK4 isometric 2.5D world view |

pub mod a2a;
pub mod agent;
pub mod api;
pub mod avatar;
pub mod config;
pub mod error;
#[cfg(feature = "gui")]
pub mod gui;
pub mod runner;
pub mod tui;
pub mod world;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
