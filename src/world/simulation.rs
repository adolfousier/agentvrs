use super::pathfind::find_path;
use super::{Grid, WorldEvent};
use crate::agent::{AgentGoal, AgentId, AgentRegistry, AgentState, Facing};
use crate::world::{Position, Tile};
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

        let agents: Vec<(AgentId, AgentState, Position)> = {
            let reg = self.registry.read().unwrap();
            reg.agents()
                .map(|a| (a.id, a.state.clone(), a.position))
                .collect()
        };

        for (id, state, pos) in agents {
            match state {
                AgentState::Idle => {
                    // Linger for a while before picking a new goal
                    let mut reg = self.registry.write().unwrap();
                    if let Some(agent) = reg.get_mut(&id) {
                        agent.anim.activity_ticks += 1;
                        if agent.anim.activity_ticks < 15 {
                            continue;
                        }
                    }
                    drop(reg);
                    self.assign_random_goal(id);
                }
                AgentState::Walking => {
                    // Move every 2nd tick for smoother, slower walking
                    if self.tick_count.is_multiple_of(2) {
                        self.step_along_path(id, pos).await;
                    }
                }
                AgentState::Working
                | AgentState::Eating
                | AgentState::Playing
                | AgentState::Exercising => self.maybe_finish(id),
                _ => {}
            }
        }

        // Toggle walk animation frames
        if self.tick_count.is_multiple_of(3) {
            let mut reg = self.registry.write().unwrap();
            for id in reg.ids() {
                if let Some(a) = reg.get_mut(&id)
                    && a.state == AgentState::Walking
                {
                    a.anim.frame = (a.anim.frame + 1) % 2;
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

    fn assign_random_goal(&self, id: AgentId) {
        use rand::Rng;
        let grid = self.grid.read().unwrap();
        let mut reg = self.registry.write().unwrap();

        let choice: u8 = rand::rng().random_range(0..13);
        let (tile_type, goal_fn): (Tile, fn(Position) -> AgentGoal) = match choice {
            0..=3 => (Tile::Desk, AgentGoal::GoToDesk),
            4..=5 => (Tile::VendingMachine, AgentGoal::GoToVending),
            6 => (Tile::CoffeeMachine, AgentGoal::GoToCoffee),
            7 => (Tile::PinballMachine, AgentGoal::GoToPinball),
            8 => (Tile::GymTreadmill, AgentGoal::GoToGym),
            9 => (Tile::WeightBench, AgentGoal::GoToGym),
            10 => (Tile::YogaMat, AgentGoal::GoToGym),
            11 => (Tile::PingPongTableLeft, AgentGoal::GoToPinball),
            _ => {
                // Wander to random floor
                if let Some(target) = grid.find_empty_floor()
                    && let Some(agent) = reg.get_mut(&id)
                    && let Some(path) = find_path(&grid, agent.position, target)
                {
                    agent.goal = Some(AgentGoal::Wander(target));
                    agent.path = path;
                    agent.set_state(AgentState::Walking);
                }
                return;
            }
        };

        let targets = grid.find_tiles(&tile_type);
        if targets.is_empty() {
            return;
        }
        let target = targets[rand::rng().random_range(0..targets.len())];

        if let Some(adj) = grid.find_adjacent_floor(target)
            && let Some(agent) = reg.get_mut(&id)
            && let Some(path) = find_path(&grid, agent.position, adj)
        {
            agent.goal = Some(goal_fn(target));
            agent.path = path;
            agent.set_state(AgentState::Walking);
            agent.anim.activity_ticks = 0;
        }

        drop(reg);
        drop(grid);
    }

    async fn step_along_path(&self, id: AgentId, pos: Position) {
        let next = {
            let reg = self.registry.read().unwrap();
            let agent = match reg.get(&id) {
                Some(a) => a,
                None => return,
            };
            agent.path.first().copied()
        };

        let Some(next_pos) = next else {
            // Arrived — transition to activity
            let mut reg = self.registry.write().unwrap();
            if let Some(agent) = reg.get_mut(&id) {
                let new_state = match &agent.goal {
                    Some(AgentGoal::GoToDesk(_)) => AgentState::Working,
                    Some(AgentGoal::GoToVending(_) | AgentGoal::GoToCoffee(_)) => {
                        AgentState::Eating
                    }
                    Some(AgentGoal::GoToPinball(_)) => AgentState::Playing,
                    Some(AgentGoal::GoToGym(_)) => AgentState::Exercising,
                    _ => AgentState::Idle,
                };
                agent.set_state(new_state);
                agent.anim.activity_ticks = 0;
            }
            return;
        };

        // Update facing direction
        {
            let mut reg = self.registry.write().unwrap();
            if let Some(agent) = reg.get_mut(&id) {
                agent.anim.facing = if next_pos.x > pos.x {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
        }

        let moved = {
            let mut grid = self.grid.write().unwrap();
            grid.move_agent(pos, next_pos)
        };

        if moved {
            {
                let mut reg = self.registry.write().unwrap();
                if let Some(agent) = reg.get_mut(&id) {
                    agent.position = next_pos;
                    agent.path.remove(0);
                }
            }
            let _ = self
                .event_tx
                .send(WorldEvent::AgentMoved {
                    agent_id: id,
                    from: pos,
                    to: next_pos,
                })
                .await;
        } else {
            // Path blocked — go idle and retry next tick
            let mut reg = self.registry.write().unwrap();
            if let Some(agent) = reg.get_mut(&id) {
                agent.path.clear();
                agent.goal = None;
                agent.set_state(AgentState::Idle);
                agent.anim.activity_ticks = 0;
            }
        }
    }

    fn maybe_finish(&self, id: AgentId) {
        use rand::Rng;
        let mut reg = self.registry.write().unwrap();
        if let Some(agent) = reg.get_mut(&id) {
            agent.anim.activity_ticks += 1;
            let min_ticks = match agent.state {
                AgentState::Working => 40,
                AgentState::Eating => 15,
                AgentState::Playing => 25,
                AgentState::Exercising => 30,
                _ => 20,
            };
            if agent.anim.activity_ticks > min_ticks && rand::rng().random_range(0..10) < 2 {
                agent.set_state(AgentState::Idle);
                agent.goal = None;
                agent.path.clear();
                agent.anim.activity_ticks = 0;
            }
        }
    }

    pub async fn run(mut self, mut shutdown: mpsc::Receiver<()>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(self.tick_ms));
        loop {
            tokio::select! {
                _ = interval.tick() => self.tick().await,
                _ = shutdown.recv() => break,
            }
        }
    }
}
