use super::{Direction, Grid, WorldEvent};
use crate::agent::{AgentId, AgentRegistry, AgentState};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

pub struct Simulation {
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub event_tx: mpsc::Sender<WorldEvent>,
    pub tick_ms: u64,
    tick_count: u64,
}

impl Simulation {
    pub fn new(
        grid: Arc<RwLock<Grid>>,
        registry: Arc<RwLock<AgentRegistry>>,
        event_tx: mpsc::Sender<WorldEvent>,
        tick_ms: u64,
    ) -> Self {
        Self {
            grid,
            registry,
            event_tx,
            tick_ms,
            tick_count: 0,
        }
    }

    pub async fn tick(&mut self) {
        self.tick_count += 1;

        let idle_agents: Vec<(AgentId, _)> = {
            let registry = self.registry.read().unwrap();
            registry
                .agents()
                .filter(|a| a.state == AgentState::Idle)
                .map(|a| (a.id, a.position))
                .collect()
        };

        for (agent_id, pos) in idle_agents {
            if rand::random::<f64>() < 0.3 {
                let dir = Direction::random();
                let bounds = {
                    let grid = self.grid.read().unwrap();
                    grid.bounds()
                };
                let new_pos = pos.moved(dir, bounds);
                let moved = {
                    let mut grid = self.grid.write().unwrap();
                    grid.move_agent(pos, new_pos)
                };
                if moved {
                    {
                        let mut reg = self.registry.write().unwrap();
                        if let Some(agent) = reg.get_mut(&agent_id) {
                            agent.position = new_pos;
                        }
                    }
                    let _ = self
                        .event_tx
                        .send(WorldEvent::AgentMoved {
                            agent_id,
                            from: pos,
                            to: new_pos,
                        })
                        .await;
                }
            }
        }

        let _ = self
            .event_tx
            .send(WorldEvent::Tick {
                count: self.tick_count,
            })
            .await;
    }

    pub async fn run(mut self, mut shutdown: mpsc::Receiver<()>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(self.tick_ms));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.tick().await;
                }
                _ = shutdown.recv() => {
                    break;
                }
            }
        }
    }
}
