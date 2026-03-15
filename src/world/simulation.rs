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
        use rand::Rng;

        self.tick_count += 1;
        self.shared_tick
            .store(self.tick_count, std::sync::atomic::Ordering::Relaxed);

        let mut needs_goal: Vec<AgentId> = Vec::new();
        let mut events: Vec<WorldEvent> = Vec::new();

        // Single lock scope: grid.read + registry.write for all agent processing
        {
            let grid = self.grid.read().unwrap();
            let mut reg = self.registry.write().unwrap();

            let agent_data: Vec<(AgentId, AgentState, Position)> = reg
                .agents()
                .map(|a| (a.id, a.state.clone(), a.position))
                .collect();

            for (id, state, pos) in agent_data {
                match state {
                    AgentState::Idle => {
                        if let Some(agent) = reg.get_mut(&id) {
                            agent.anim.activity_ticks += 1;
                            if agent.anim.activity_ticks >= 15 {
                                needs_goal.push(id);
                            }
                        }
                    }
                    AgentState::Walking => {
                        if self.tick_count.is_multiple_of(2) {
                            let next = reg.get(&id).and_then(|a| a.path.first().copied());
                            if let Some(next_pos) = next {
                                let can_move = grid
                                    .get(next_pos)
                                    .map(|c| !c.tile.is_solid())
                                    .unwrap_or(false);
                                if can_move {
                                    if let Some(agent) = reg.get_mut(&id) {
                                        let dx = next_pos.x as i32 - pos.x as i32;
                                        let dy = next_pos.y as i32 - pos.y as i32;
                                        agent.anim.facing = if dx + dy > 0 {
                                            Facing::Right
                                        } else {
                                            Facing::Left
                                        };
                                        agent.position = next_pos;
                                        agent.path.remove(0);
                                        agent.anim.blocked_ticks = 0;
                                    }
                                    events.push(WorldEvent::AgentMoved {
                                        agent_id: id,
                                        from: pos,
                                        to: next_pos,
                                    });
                                } else {
                                    // Hit a solid tile — give up
                                    if let Some(agent) = reg.get_mut(&id) {
                                        agent.path.clear();
                                        agent.goal = None;
                                        agent.set_state(AgentState::Idle);
                                        agent.anim.activity_ticks = 0;
                                        agent.anim.blocked_ticks = 0;
                                    }
                                }
                            } else {
                                // Arrived — transition to activity
                                if let Some(agent) = reg.get_mut(&id) {
                                    let new_state = match &agent.goal {
                                        Some(AgentGoal::GoToDesk(_)) => AgentState::Working,
                                        Some(
                                            AgentGoal::GoToVending(_) | AgentGoal::GoToCoffee(_),
                                        ) => AgentState::Eating,
                                        Some(AgentGoal::GoToPinball(_)) => AgentState::Playing,
                                        Some(AgentGoal::GoToMeeting(_)) => AgentState::Working,
                                        Some(AgentGoal::GoToServer(_)) => AgentState::Thinking,
                                        Some(AgentGoal::GoToGym(_)) => AgentState::Exercising,
                                        _ => AgentState::Idle,
                                    };
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
                            }
                        }
                    }
                    AgentState::Working
                    | AgentState::Eating
                    | AgentState::Playing
                    | AgentState::Exercising
                    | AgentState::Thinking => {
                        if let Some(agent) = reg.get_mut(&id) {
                            agent.anim.activity_ticks += 1;
                            let min_ticks = match agent.state {
                                AgentState::Working => 40,
                                AgentState::Eating => 20,
                                AgentState::Playing => 30,
                                AgentState::Exercising => 35,
                                AgentState::Thinking => 25,
                                _ => 25,
                            };
                            if agent.anim.activity_ticks > min_ticks
                                && rand::rng().random_range(0..10u8) < 2
                            {
                                agent.set_state(AgentState::Idle);
                                agent.goal = None;
                                agent.path.clear();
                                agent.anim.activity_ticks = 0;
                            }
                        }
                    }
                    AgentState::Messaging => {
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
                for id in reg.ids() {
                    if let Some(a) = reg.get_mut(&id)
                        && a.state == AgentState::Walking
                    {
                        a.anim.frame = (a.anim.frame + 1) % 2;
                    }
                }
            }
        } // All locks dropped here

        // Goal assignment for idle agents (acquires its own locks briefly)
        for id in needs_goal {
            self.assign_random_goal(id);
        }

        // Emit events without holding any locks
        for event in events {
            self.emit(event).await;
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

        // Reset activity_ticks so we don't retry every tick if assignment fails
        if let Some(agent) = reg.get_mut(&id) {
            agent.anim.activity_ticks = 0;
        }

        let choice: u8 = rand::rng().random_range(0..15);
        let (tile_type, goal_fn): (Tile, fn(Position) -> AgentGoal) = match choice {
            0..=3 => (Tile::Desk, AgentGoal::GoToDesk),
            4..=5 => (Tile::VendingMachine, AgentGoal::GoToVending),
            6 => (Tile::CoffeeMachine, AgentGoal::GoToCoffee),
            7 => (Tile::PinballMachine, AgentGoal::GoToPinball),
            8 => (Tile::GymTreadmill, AgentGoal::GoToGym),
            9 => (Tile::WeightBench, AgentGoal::GoToGym),
            10 => (Tile::YogaMat, AgentGoal::GoToGym),
            11 => (Tile::MeetingTable, AgentGoal::GoToMeeting),
            12..=13 => (Tile::ServerRack, AgentGoal::GoToServer),
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

        // Capacity per tile type: meeting table = 4 (one per adjacent tile), everything else = 1
        let capacity: usize = match tile_type {
            Tile::MeetingTable => 4,
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
        if !available.is_empty() {
            let target = available[rand::rng().random_range(0..available.len())];

            // Find where other agents heading to same target will stand
            let taken_spots: Vec<Position> = reg
                .agents()
                .filter(|a| a.id != id)
                .filter(|a| a.goal.as_ref().map(|g| g.target()) == Some(target))
                .map(|a| a.path.last().copied().unwrap_or(a.position))
                .collect();

            if let Some(adj) = grid.find_adjacent_floor_avoiding(target, &taken_spots)
                && let Some(agent) = reg.get_mut(&id)
                && let Some(path) = find_path(&grid, agent.position, adj)
            {
                agent.goal = Some(goal_fn(target));
                agent.path = path;
                agent.set_state(AgentState::Walking);
                agent.anim.activity_ticks = 0;
                return;
            }
        }

        // Fallback: wander to any random floor tile (guarantees agent keeps moving)
        if let Some(target) = grid.find_empty_floor()
            && let Some(agent) = reg.get_mut(&id)
            && let Some(path) = find_path(&grid, agent.position, target)
        {
            agent.goal = Some(AgentGoal::Wander(target));
            agent.path = path;
            agent.set_state(AgentState::Walking);
            agent.anim.activity_ticks = 0;
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
