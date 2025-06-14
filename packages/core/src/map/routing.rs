use std::collections::{BinaryHeap, HashMap};

use crate::prelude::{Coordinates, Map};

#[derive(Eq, PartialEq)]
struct Node {
    position: Coordinates,
    cost: i32,     // g(n): Cost from start node to current node
    estimate: i32, // f(n): Estimated total cost (g + h)
}

// Implement ordering for BinaryHeap as a min-heap by estimate (f)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering so the node with smallest estimate is popped first
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    /// Finds a path from `start` to `goal` coordinates using A* pathfinding.
    /// Returns `Some(Vec<Coordinates>)` if a path exists, else `None`.
    pub fn find_path(&self, start: &Coordinates, goal: &Coordinates) -> Option<Vec<Coordinates>> {
        // Manhattan distance heuristic for grid-based movement
        fn heuristic(a: Coordinates, b: Coordinates) -> u32 {
            a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
        }

        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            position: *start,
            cost: 0,
            estimate: heuristic(*start, *goal) as i32,
        });

        let mut came_from: HashMap<Coordinates, Coordinates> = HashMap::new();
        let mut g_score: HashMap<Coordinates, i32> = HashMap::new();
        g_score.insert(*start, 0);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.position;

            if current == *goal {
                // Reconstruct path by walking backwards
                let mut path = vec![current];
                let mut cur = current;
                while let Some(prev) = came_from.get(&cur) {
                    cur = *prev;
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }

            // Generate valid neighbors (up, down, left, right)
            let neighbors = [
                Some(Coordinates {
                    x: current.x + 1,
                    y: current.y,
                }),
                Some(Coordinates {
                    x: current.x,
                    y: current.y + 1,
                }),
                current
                    .x
                    .checked_sub(1)
                    .map(|x| Coordinates { x, y: current.y }),
                current
                    .y
                    .checked_sub(1)
                    .map(|y| Coordinates { x: current.x, y }),
            ]
            .iter()
            .filter_map(|opt| *opt) // unwrap Option<Coordinates> safely
            .collect::<Vec<_>>();

            for neighbor in neighbors {
                // Skip if no tile at neighbor or tile is blocking
                if !self.move_allowed(neighbor) {
                    continue;
                }

                // Tentative cost from start to neighbor
                let tentative_g_score =
                    g_score.get(&current).unwrap_or(&i32::MAX).saturating_add(1);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);

                    open_set.push(Node {
                        position: neighbor,
                        cost: tentative_g_score,
                        estimate: tentative_g_score
                            .saturating_add(heuristic(neighbor, *goal) as i32),
                    });
                }
            }
        }

        None
    }
}
