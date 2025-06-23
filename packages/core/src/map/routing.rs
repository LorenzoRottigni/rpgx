use std::collections::{BinaryHeap, HashMap};

use crate::{
    prelude::{Coordinates, Map},
    traits::Grid,
};

/// A node in the A* search graph.
#[derive(Eq, PartialEq)]
struct Node {
    position: Coordinates,
    cost: i32,     // g(n): Cost from start to current node
    estimate: i32, // f(n): Estimated total cost (g + h)
}

// Implement ordering for BinaryHeap as a min-heap by `estimate` (f)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order so the node with the *smallest* estimate is popped first
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
    /// Manhattan distance heuristic used in A* pathfinding.
    ///
    /// This version is **bound exclusive**, meaning it returns `distance - 1`
    /// unless the points are the same, in which case it returns `0`.
    ///
    /// For example:
    /// - `heuristic((0,0), (0,0)) == 0`
    /// - `heuristic((0,0), (1,0)) == 0`
    /// - `heuristic((0,0), (1,1)) == 1`
    pub fn heuristic(a: Coordinates, b: Coordinates) -> u32 {
        let distance = a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
        if distance > 0 {
            distance - 1
        } else {
            0
        }
    }

    /// Finds a path from `start` to `goal` coordinates using A* pathfinding.
    ///
    /// - Uses 4-directional movement (up, down, left, right).
    /// - Skips any tiles that are blocking.
    /// - Returns `Some(path)` if a path is found, or `None` if unreachable.
    pub fn find_path(&self, start: &Coordinates, goal: &Coordinates) -> Option<Vec<Coordinates>> {
        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            position: *start,
            cost: 0,
            estimate: Self::heuristic(*start, *goal) as i32,
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

            // Generate 4-directional neighbors
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
                // Skip if blocked
                if self.contains(&neighbor) && self.is_blocking_at(&neighbor) {
                    continue;
                }

                let tentative_g_score =
                    g_score.get(&current).unwrap_or(&i32::MAX).saturating_add(1);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);

                    open_set.push(Node {
                        position: neighbor,
                        cost: tentative_g_score,
                        estimate: tentative_g_score
                            .saturating_add(Self::heuristic(neighbor, *goal) as i32),
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
    use crate::prelude::{Effect, Layer, Mask, Rect, Shape};

    pub fn get_open_map() -> Map {
        Map::new(
            "test".into(),
            vec![Layer::new(
                "test".into(),
                vec![Mask::new(
                    "test".into(),
                    Rect::new(Coordinates::default(), Shape::from_square(10)).into_many(),
                    vec![],
                )],
                1,
            )],
            Coordinates::default(),
        )
    }

    pub fn get_block_map() -> Map {
        Map::new(
            "test".into(),
            vec![Layer::new(
                "test".into(),
                vec![
                    Mask::new(
                        "test".into(),
                        Rect::new(Coordinates::default(), Shape::from_square(10)).into_many(),
                        vec![],
                    ),
                    Mask::new(
                        "test".into(),
                        Rect::new(Coordinates::new(2, 2), Shape::from_square(4)).into_block(),
                        vec![Effect::Block(Rect::new(
                            Coordinates::new(1, 1),
                            Shape::from_square(2),
                        ))],
                    ),
                ],
                1,
            )],
            Coordinates::default(),
        )
    }

    #[test]
    pub fn finds_streight_path() {
        let map = get_open_map();
        let steps = map.find_path(&Coordinates::default(), &Coordinates::new(6, 0));

        assert_eq!(
            steps.clone().unwrap().get(0).unwrap().clone(),
            Coordinates::new(0, 0)
        );
        assert_eq!(
            steps.clone().unwrap().get(4).unwrap().clone(),
            Coordinates::new(4, 0)
        );
    }

    #[test]
    pub fn finds_path_around_block() {
        let map = get_block_map();
        let path = map.find_path(&Coordinates::new(1, 1), &Coordinates::new(6, 2));

        assert!(path.is_some(), "Expected a valid path around the block");
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&Coordinates::new(1, 1)));
        assert_eq!(path.last(), Some(&Coordinates::new(6, 2)));

        // Ensure none of the steps cross the blocked area (2..6, 2..6)
        for step in &path {
            assert!(
                step.x < 2 || step.x > 5 || step.y < 2 || step.y > 5,
                "Path goes through blocked area at: {:?}",
                step
            );
        }
    }

    #[test]
    pub fn returns_single_point_if_start_equals_goal() {
        let map = get_open_map();
        let coord = Coordinates::new(3, 3);
        let result = map.find_path(&coord, &coord);

        assert_eq!(result, Some(vec![coord]));
    }

    #[test]
    pub fn avoids_block_area_edges_correctly() {
        let map = get_block_map();

        // Attempt to walk along the edge of the block (just outside the 2..6 box)
        let result = map.find_path(&Coordinates::new(1, 1), &Coordinates::new(6, 1));

        assert!(result.is_some(), "Should be able to walk around block edge");
        for step in result.unwrap() {
            assert!(
                step.x < 2 || step.x > 5 || step.y < 2 || step.y > 5,
                "Step {:?} enters blocked area",
                step
            );
        }
    }

    #[test]
    pub fn can_path_in_full_unblocked_area() {
        let map = get_open_map();
        let result = map.find_path(&Coordinates::new(0, 0), &Coordinates::new(9, 9));
        assert!(result.is_some());

        let steps = result.unwrap();
        assert_eq!(steps.first().unwrap(), &Coordinates::new(0, 0));
        assert_eq!(steps.last().unwrap(), &Coordinates::new(9, 9));
    }

    #[test]
    pub fn heuristic_returns_zero_for_same_point() {
        let a = Coordinates::new(2, 2);
        assert_eq!(Map::heuristic(a, a), 0);
    }

    #[test]
    pub fn heuristic_returns_distance_minus_one() {
        let a = Coordinates::new(0, 0);
        let b = Coordinates::new(3, 4); // distance = 7
        assert_eq!(Map::heuristic(a, b), 6);
    }
}
