use super::pathfind::find_path;
use super::{Grid, WorldEvent};
use crate::agent::{AgentGoal, AgentId, AgentRegistry, AgentState, Facing};
use crate::world::{Position, Tile};
use std::sync::{Arc, RwLock};
use tokio::sync::{broadcast, mpsc};

pub struct Simulation {
    pub grid: Arc<RwLock<Grid>>,
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub event_tx: mpsc::Sender<WorldEvent>,
    pub broadcast_tx: broadcast::Sender<WorldEvent>,
    pub tick_ms: u64,
    tick_count: u64,
    pub shared_tick: Arc<std::sync::atomic::AtomicU64>,
}

impl Simulation {
    pub fn new(
        grid: Arc<RwLock<Grid>>,
        registry: Arc<RwLock<AgentRegistry>>,
        event_tx: mpsc::Sender<WorldEvent>,
        tick_ms: u64,
    ) -> Self {
        let (broadcast_tx, _) = broadcast::channel(256);
        Self {
            grid,
            registry,
            event_tx,
            broadcast_tx,
            tick_ms,
            tick_count: 0,
            shared_tick: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub fn with_broadcast(mut self, broadcast_tx: broadcast::Sender<WorldEvent>) -> Self {
        self.broadcast_tx = broadcast_tx;
        self
    }

    /// Send event to both mpsc (TUI/GUI) and broadcast (SSE) channels.
    async fn emit(&self, event: WorldEvent) {
        let _ = self.event_tx.send(event.clone()).await;
        let _ = self.broadcast_tx.send(event);
    }

    pub async fn tick(&mut self) {
        self.tick_count += 1;
        self.shared_tick
            .store(self.tick_count, std::sync::atomic::Ordering::Relaxed);

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
                AgentState::Messaging => {
                    // Auto-transition back to Idle after 30 ticks
                    let mut reg = self.registry.write().unwrap();
                    if let Some(agent) = reg.get_mut(&id) {
                        agent.anim.activity_ticks += 1;
                        if agent.anim.activity_ticks >= 30 {
                            agent.set_state(AgentState::Idle);
                            agent.clear_speech();
                            agent.anim.activity_ticks = 0;
                        }
                    }
                }
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

        self.emit(WorldEvent::Tick {
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

        // Count how many agents already target each position
        let other_targets: Vec<Position> = reg
            .agents()
            .filter(|a| a.id != id)
            .filter_map(|a| a.goal.as_ref().map(|g| g.target()))
            .collect();

        // Capacity per tile type: ping pong = 2 (one per side), everything else = 1
        let capacity: usize = match tile_type {
            Tile::PingPongTableLeft | Tile::PingPongTableRight => 2,
            _ => 1,
        };

        let all_targets = grid.find_tiles(&tile_type);
        // Filter to furniture that still has capacity
        let available: Vec<Position> = all_targets
            .into_iter()
            .filter(|t| {
                let count = other_targets.iter().filter(|ot| *ot == t).count();
                count < capacity
            })
            .collect();
        if available.is_empty() {
            return;
        }
        let target = available[rand::rng().random_range(0..available.len())];

        // Find where other agents heading to same target will stand, so we pick a different spot
        let taken_spots: Vec<Position> = reg
            .agents()
            .filter(|a| a.id != id)
            .filter(|a| a.goal.as_ref().map(|g| g.target()) == Some(target))
            .map(|a| {
                // Their destination is the last step on their path, or their current position
                a.path.last().copied().unwrap_or(a.position)
            })
            .collect();

        if let Some(adj) = grid.find_adjacent_floor_avoiding(target, &taken_spots)
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
            // Arrived — transition to activity and face the target furniture
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
                // Face toward the target furniture
                // Agent typically at -x from furniture (to see LEFT face detail)
                // Furniture is at +x → face Right (toward top-right on screen)
                if let Some(goal) = &agent.goal {
                    let target = goal.target();
                    let dx = target.x as i32 - agent.position.x as i32;
                    let dy = target.y as i32 - agent.position.y as i32;
                    agent.anim.facing = if dx + dy > 0 {
                        Facing::Right
                    } else {
                        Facing::Left
                    };
                }
                agent.set_state(new_state);
                agent.anim.activity_ticks = 0;
            }
            return;
        };

        // Update facing direction based on iso movement
        {
            let mut reg = self.registry.write().unwrap();
            if let Some(agent) = reg.get_mut(&id) {
                let dx = next_pos.x as i32 - pos.x as i32;
                let dy = next_pos.y as i32 - pos.y as i32;
                agent.anim.facing = if dx + dy > 0 {
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
                    agent.anim.blocked_ticks = 0;
                }
            }
            self.emit(WorldEvent::AgentMoved {
                agent_id: id,
                from: pos,
                to: next_pos,
            })
            .await;
        } else {
            // Path hit a solid tile (furniture/wall) — repath or give up
            let mut reg = self.registry.write().unwrap();
            if let Some(agent) = reg.get_mut(&id) {
                agent.path.clear();
                agent.goal = None;
                agent.set_state(AgentState::Idle);
                agent.anim.activity_ticks = 0;
                agent.anim.blocked_ticks = 0;
            }
        }
    }

    fn maybe_finish(&self, id: AgentId) {
        use rand::Rng;
        let mut reg = self.registry.write().unwrap();
        if let Some(agent) = reg.get_mut(&id) {
            agent.anim.activity_ticks += 1;
            let min_ticks = match agent.state {
                AgentState::Working => 120,
                AgentState::Eating => 50,
                AgentState::Playing => 80,
                AgentState::Exercising => 90,
                _ => 60,
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
