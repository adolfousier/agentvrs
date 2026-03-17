use crate::world::{Grid, Position};
use std::collections::{HashMap, VecDeque};

/// BFS pathfinding on the grid. Returns the path excluding `from`, including `to`.
pub fn find_path(grid: &Grid, from: Position, to: Position) -> Option<Vec<Position>> {
    if from == to {
        return Some(Vec::new());
    }

    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    queue.push_back(from);
    came_from.insert(from, from);

    while let Some(current) = queue.pop_front() {
        if current == to {
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = to;
            while pos != from {
                path.push(pos);
                pos = came_from[&pos];
            }
            path.reverse();
            return Some(path);
        }

        let neighbors = [
            Position::new(current.x.wrapping_add(1), current.y),
            Position::new(current.x.wrapping_sub(1), current.y),
            Position::new(current.x, current.y.wrapping_add(1)),
            Position::new(current.x, current.y.wrapping_sub(1)),
        ];

        for next in neighbors {
            if came_from.contains_key(&next) {
                continue;
            }
            // Pathfinding ignores occupants — only checks tile solidity.
            // Actual collision is handled at move time by move_agent().
            // This prevents agents from permanently blocking each other's paths.
            let walkable = grid.get(next).map(|c| !c.tile.is_solid()).unwrap_or(false);
            if walkable {
                came_from.insert(next, current);
                queue.push_back(next);
            }
        }
    }

    None
}
