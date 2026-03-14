pub mod a2a;
pub mod agent;
pub mod api;
pub mod avatar;
pub mod config;
pub mod error;
pub mod tui;
pub mod world;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
