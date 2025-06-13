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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        common::rect::Rect,
        prelude::{Effect, Layer, Map, Shape, Tile},
    };

    // Helper: create a blocking tile at the given coordinate
    fn blocking_tile_at(coord: Coordinates) -> Tile {
        Tile {
            area: Rect {
                origin: coord,
                shape: Shape::from_square(1),
            },
            effect: Effect {
                block: true,
                ..Default::default()
            },
        }
    }

    // Helper: create a Map with a Block layer containing blocking tiles at specified coordinates
    fn map_with_layer(blocks: Vec<Coordinates>, width: u32, height: u32) -> Map {
        // let shape = Shape { width, height };
        // let block_tiles = blocks.into_iter().map(blocking_tile_at).collect::<Vec<_>>();

        let block_layer = Layer {
            name: "block".into(),
            masks: vec![],
            z: 1,
        };

        Map::new(
            "test_map".into(),
            vec![block_layer],
            Coordinates { x: 0, y: 0 },
        )
    }

    #[test]
    fn finds_clear_path() {
        let map = map_with_layer(vec![], 5, 5);
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 2, y: 2 };

        let path = map.find_path(&start, &goal).unwrap();
        assert_eq!(path.first().unwrap(), &start);
        assert_eq!(path.last().unwrap(), &goal);
        assert!(path.len() >= 3);
    }

    #[test]
    fn avoids_blocked_tiles() {
        // Vertical wall blocking horizontal crossing at x=1
        let blocked = vec![
            Coordinates { x: 1, y: 0 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 1, y: 2 },
        ];
        let map = map_with_layer(blocked, 3, 3);
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 2, y: 0 };

        let path = map.find_path(&start, &goal);
        assert!(path.is_none());
    }

    #[test]
    fn returns_none_if_no_path() {
        // Completely blocked scenario
        let blocked = vec![
            Coordinates { x: 0, y: 1 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 2, y: 1 },
        ];
        let map = map_with_layer(blocked, 3, 3);
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 2, y: 2 };

        let path = map.find_path(&start, &goal);
        assert!(path.is_none());
    }

    #[test]
    fn handles_start_equals_goal() {
        let map = map_with_layer(vec![], 3, 3);
        let start = Coordinates { x: 1, y: 1 };

        let path = map.find_path(&start, &start).unwrap();
        assert_eq!(path, vec![start]);
    }

    #[test]
    fn path_respects_blocking_tiles() {
        let blocked = vec![Coordinates { x: 1, y: 1 }];
        let map = map_with_layer(blocked, 3, 3);
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 2, y: 2 };

        let path = map.find_path(&start, &goal).unwrap();

        // Path should not include the blocking tile
        assert!(!path.contains(&Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn no_path_when_start_or_goal_out_of_bounds() {
        let map = map_with_layer(vec![], 3, 3);
        let start = Coordinates { x: 10, y: 10 };
        let goal = Coordinates { x: 2, y: 2 };

        // No tile at start coordinate, should return None
        assert!(map.find_path(&start, &goal).is_none());

        let start = Coordinates { x: 1, y: 1 };
        let goal = Coordinates { x: 20, y: 20 };

        // No tile at goal coordinate, should return None
        assert!(map.find_path(&start, &goal).is_none());
    }
}
