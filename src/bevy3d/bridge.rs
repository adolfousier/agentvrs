use crate::agent::AgentRegistry;
use crate::db::Database;
use crate::world::Grid;
use bevy::prelude::*;
use std::sync::{Arc, Mutex, RwLock};

/// Shared state bridge between the Tokio simulation and Bevy ECS.
#[derive(Resource)]
pub struct WorldBridge {
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub db: Arc<Mutex<Database>>,
}
