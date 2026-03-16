use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn moved(&self, direction: Direction, bounds: (u16, u16)) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Down => Self {
                x: self.x,
                y: (self.y + 1).min(bounds.1.saturating_sub(1)),
            },
            Direction::Left => Self {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Direction::Right => Self {
                x: (self.x + 1).min(bounds.0.saturating_sub(1)),
                y: self.y,
            },
        }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = (self.x as f64) - (other.x as f64);
        let dy = (self.y as f64) - (other.y as f64);
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn random() -> Self {
        use rand::RngExt;
        match rand::rng().random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}
