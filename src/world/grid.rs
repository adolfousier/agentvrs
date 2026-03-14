use super::Position;
use crate::agent::AgentId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Floor,
    Wall,
    Desk,
    Decoration(char),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub tile: Tile,
    pub occupant: Option<AgentId>,
}

impl Cell {
    pub fn floor() -> Self {
        Self {
            tile: Tile::Floor,
            occupant: None,
        }
    }

    pub fn wall() -> Self {
        Self {
            tile: Tile::Wall,
            occupant: None,
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(self.tile, Tile::Floor | Tile::Desk) && self.occupant.is_none()
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
            .map(|_| Cell::floor())
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
            grid.set_tile(Position::new(x, 0), Tile::Wall);
            grid.set_tile(Position::new(x, height - 1), Tile::Wall);
        }
        for y in 0..height {
            grid.set_tile(Position::new(0, y), Tile::Wall);
            grid.set_tile(Position::new(width - 1, y), Tile::Wall);
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
        if let Some(cell) = self.get_mut(pos) {
            cell.occupant.take()
        } else {
            None
        }
    }

    pub fn move_agent(&mut self, from: Position, to: Position) -> bool {
        if from == to {
            return true;
        }
        let is_target_walkable = self.get(to).map(|c| c.is_walkable()).unwrap_or(false);

        if !is_target_walkable {
            return false;
        }

        let agent_id = if let Some(cell) = self.get_mut(from) {
            cell.occupant.take()
        } else {
            return false;
        };

        if let Some(id) = agent_id
            && let Some(cell) = self.get_mut(to)
        {
            cell.occupant = Some(id);
            return true;
        }
        false
    }

    pub fn find_empty_floor(&self) -> Option<Position> {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let x = rng.random_range(1..self.width.saturating_sub(1).max(1));
            let y = rng.random_range(1..self.height.saturating_sub(1).max(1));
            let pos = Position::new(x, y);
            if let Some(cell) = self.get(pos)
                && cell.is_walkable()
            {
                return Some(pos);
            }
        }
        None
    }

    pub fn bounds(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
}
