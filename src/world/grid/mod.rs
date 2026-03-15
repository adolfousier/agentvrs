mod layout;
mod tiles;

pub use layout::*;
pub use tiles::*;

use super::Position;
use crate::agent::AgentId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub tile: Tile,
    pub occupant: Option<AgentId>,
}

impl Cell {
    pub fn floor(kind: FloorKind) -> Self {
        Self {
            tile: Tile::Floor(kind),
            occupant: None,
        }
    }

    pub fn wall() -> Self {
        Self {
            tile: Tile::Wall(WallKind::Solid),
            occupant: None,
        }
    }

    pub fn is_walkable(&self) -> bool {
        !self.tile.is_solid() && self.occupant.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let cells = (0..width as usize * height as usize)
            .map(|_| Cell::floor(FloorKind::Wood))
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn with_walls(width: u16, height: u16) -> Self {
        let mut grid = Self::new(width, height);
        for x in 0..width {
            grid.set_tile(Position::new(x, 0), Tile::Wall(WallKind::Solid));
            grid.set_tile(Position::new(x, height - 1), Tile::Wall(WallKind::Solid));
        }
        for y in 0..height {
            grid.set_tile(Position::new(0, y), Tile::Wall(WallKind::Solid));
            grid.set_tile(Position::new(width - 1, y), Tile::Wall(WallKind::Solid));
        }
        grid
    }

    fn index(&self, pos: Position) -> Option<usize> {
        if pos.x < self.width && pos.y < self.height {
            Some(pos.y as usize * self.width as usize + pos.x as usize)
        } else {
            None
        }
    }

    pub fn get(&self, pos: Position) -> Option<&Cell> {
        self.index(pos).map(|i| &self.cells[i])
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        self.index(pos).map(|i| &mut self.cells[i])
    }

    pub fn set_tile(&mut self, pos: Position, tile: Tile) {
        if let Some(cell) = self.get_mut(pos) {
            cell.tile = tile;
        }
    }

    pub fn place_agent(&mut self, pos: Position, agent_id: AgentId) -> bool {
        if let Some(cell) = self.get_mut(pos)
            && cell.is_walkable()
        {
            cell.occupant = Some(agent_id);
            return true;
        }
        false
    }

    pub fn remove_agent(&mut self, pos: Position) -> Option<AgentId> {
        self.get_mut(pos).and_then(|cell| cell.occupant.take())
    }

    pub fn move_agent(&mut self, _from: Position, to: Position) -> bool {
        if _from == to {
            return true;
        }
        // Only block on solid tiles (walls/furniture).
        // Agent positions are tracked in AgentRegistry, not grid occupants.
        self.get(to).map(|c| !c.tile.is_solid()).unwrap_or(false)
    }

    pub fn find_empty_floor(&self) -> Option<Position> {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..200 {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            let pos = Position::new(x, y);
            if let Some(cell) = self.get(pos)
                && !cell.tile.is_solid()
            {
                return Some(pos);
            }
        }
        None
    }

    pub fn find_tiles(&self, tile_match: &Tile) -> Vec<Position> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Position::new(x, y)))
            .filter(|pos| {
                self.get(*pos)
                    .map(|c| std::mem::discriminant(&c.tile) == std::mem::discriminant(tile_match))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Find an adjacent walkable tile, preferring front of LEFT face (detail face).
    pub fn find_adjacent_floor(&self, pos: Position) -> Option<Position> {
        self.find_adjacent_floor_avoiding(pos, &[])
    }

    pub fn find_adjacent_floor_avoiding(
        &self,
        pos: Position,
        avoid: &[Position],
    ) -> Option<Position> {
        // Ordered: +y (front), -y (back/opposite side), +x, -x
        // This gives opposite-side placement for second agent (e.g. ping pong)
        let candidates = [
            Position::new(pos.x, pos.y + 1),
            Position::new(pos.x, pos.y.wrapping_sub(1)),
            Position::new(pos.x + 1, pos.y),
            Position::new(pos.x.wrapping_sub(1), pos.y),
        ];
        let is_floor = |p: &Position| {
            self.get(*p).map(|c| !c.tile.is_solid()).unwrap_or(false)
        };

        // 1st: not avoided, walkable (order already prefers front)
        candidates
            .iter()
            .copied()
            .find(|p| !avoid.contains(p) && is_floor(p))
            // 2nd: any floor at all
            .or_else(|| candidates.iter().copied().find(|p| is_floor(p)))
    }

    pub fn bounds(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
}
