use std::collections::{BinaryHeap, HashMap};

use crate::prelude::{Coordinates, Map};

#[derive(Eq, PartialEq)]
struct Node {
    position: Coordinates,
    cost: i32,     // Cost from start node (g)
    estimate: i32, // Estimated total cost (f = g + h)
}

// Implement ordering for BinaryHeap (min-heap by estimate)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering to make BinaryHeap a min-heap by estimate
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
    pub fn find_path(&self, start: &Coordinates, goal: &Coordinates) -> Option<Vec<Coordinates>> {
        // Heuristic function (Manhattan distance)
        fn heuristic(a: Coordinates, b: Coordinates) -> i32 {
            (a.x - b.x).abs() + (a.y - b.y).abs()
        }

        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            position: *start,
            cost: 0,
            estimate: heuristic(*start, *goal),
        });

        let mut came_from: HashMap<Coordinates, Coordinates> = HashMap::new();
        let mut g_score: HashMap<Coordinates, i32> = HashMap::new();
        g_score.insert(*start, 0);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.position;

            if current == *goal {
                // Reconstruct path
                let mut path = vec![current];
                let mut cur = current;
                while let Some(prev) = came_from.get(&cur) {
                    cur = *prev;
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }

            // Neighbors: up, down, left, right
            let neighbors = [
                Coordinates {
                    x: current.x + 1,
                    y: current.y,
                },
                Coordinates {
                    x: current.x - 1,
                    y: current.y,
                },
                Coordinates {
                    x: current.x,
                    y: current.y + 1,
                },
                Coordinates {
                    x: current.x,
                    y: current.y - 1,
                },
            ];

            for neighbor in neighbors {
                if neighbor.x < 0 || neighbor.y < 0 {
                    continue; // Ignore negative coords
                }

                if self.get_base_tile(neighbor).is_none() {
                    continue;
                }

                // Check if blocked in any layer
                if self.is_tile_blocked(neighbor) {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).unwrap_or(&i32::MAX) + 1;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);

                    open_set.push(Node {
                        position: neighbor,
                        cost: tentative_g_score,
                        estimate: tentative_g_score + heuristic(neighbor, *goal),
                    });
                }
            }
        }

        None // No path found
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{Effect, Layer, LayerType, Map, Shape, Tile};

    fn blocking_tile_at(coord: Coordinates) -> Tile {
        Tile {
            id: 0,
            pointer: coord,
            shape: Shape::from_square(1),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        }
    }

    fn map_with_layer(blocks: Vec<Coordinates>, width: i32, height: i32) -> Map {
        let shape = Shape { width, height };

        let block_tiles = blocks.into_iter().map(blocking_tile_at).collect::<Vec<_>>();
        let block_layer = Layer {
            name: "block".into(),
            kind: LayerType::Block,
            shape,
            tiles: block_tiles,
            masks: vec![],
        };

        // Construct via Map::new to trigger base layer creation automatically
        Map::new("test_map".into(), vec![block_layer])
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
        let blocked = vec![Coordinates { x: 1, y: 0 }, Coordinates { x: 1, y: 1 }];
        let map = map_with_layer(blocked, 3, 3);
        let start = Coordinates { x: 0, y: 0 };
        let goal = Coordinates { x: 2, y: 0 };

        let path = map.find_path(&start, &goal).unwrap();
        assert!(path.contains(&Coordinates { x: 0, y: 1 }));
        assert!(!path.contains(&Coordinates { x: 1, y: 0 }));
    }

    #[test]
    fn returns_none_if_no_path() {
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
    fn handles_start_equals_goal() {
        let map = map_with_layer(vec![], 3, 3);
        let start = Coordinates { x: 1, y: 1 };

        let path = map.find_path(&start, &start).unwrap();
        assert_eq!(path, vec![start]);
    }
}
