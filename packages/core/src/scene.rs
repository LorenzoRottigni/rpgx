use crate::prelude::{Coordinates, Direction, Map, MoveError, Pawn, Tile};

/// RPG scene providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Scene {
    pub name: String,
    pub map: Map,
    pub pawn: Pawn,
}

impl Scene {
    pub fn new(name: String, map: Map, pawn: Pawn) -> Self {
        Self { map, pawn, name }
    }

    /// Walk to the target [`Coordinates`] through the best path
    pub async fn walk_to(&mut self, target_position: Coordinates) -> Result<Tile, MoveError> {
        let start = self.pawn.pointer;
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;

        let mut tile = None;
        for step_coords in path {
            tile = Some(self.move_to(step_coords)?);
        }

        if let Some(tile) = tile {
            Ok(tile)
        } else {
            Err(MoveError::TileNotFound)
        }
    }

    /// Make a step into the provided [`Direction`]
    pub fn step_to(&mut self, direction: Direction) -> Result<Tile, MoveError> {
        let delta = direction.to_delta();
        let target_position = self.pawn.pointer + delta;
        self.move_to(target_position)
    }

    /// Move to the provided [`Coordinates`] if allowed
    pub fn move_to(&mut self, target_position: Coordinates) -> Result<Tile, MoveError> {
        if self.map.is_blocking_at(target_position) {
            return Err(MoveError::TileBlocked);
        }

        let base_layer = self.map.get_base_layer().ok_or(MoveError::TileNotFound)?;
        let tile = base_layer
            .get_tile_at(target_position)
            .ok_or(MoveError::TileNotFound)?;

        self.pawn.pointer = tile.pointer;

        // Trigger actions on all layers for the tile pointer
        // self.map.trigger_actions_at(tile.pointer);

        // return the tile to make the caller able to dispatch its actions itself using its library
        // (allows the caller to use its context within the callback)
        Ok(tile)
    }

    /// Get steps to reach the target [`Coordinates`] from the current position
    pub fn steps_to(&self, target_position: Coordinates) -> Result<Vec<Coordinates>, MoveError> {
        let start = self.pawn.pointer;
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;
        Ok(path)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::*;

    fn basic_test_map() -> Map {
        let shape = Shape {
            width: 3,
            height: 3,
        };
        let masks = vec![];
        let layer = Layer::new("ground".to_string(), LayerType::Texture, shape, masks, 1);
        Map::new("test_map".to_string(), vec![layer])
    }

    fn pawn_at(x: i32, y: i32) -> Pawn {
        Pawn {
            pointer: Coordinates { x: 0, y: 0 },
            texture_id: 0,
        }
    }

    #[test]
    fn test_scene_move_to_success() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, pawn_at(0, 0));
        let tile = scene.move_to(Coordinates { x: 1, y: 0 }).unwrap();
        assert_eq!(tile.pointer, Coordinates { x: 1, y: 0 });
    }

    #[test]
    fn test_scene_move_to_blocked() {
        let mut map = basic_test_map();

        // Manually block tile at (1, 0)
        let tile = Tile {
            id: 0,
            shape: Shape::from_square(1),
            pointer: Coordinates { x: 1, y: 0 },
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        map.layers[0].tiles.push(tile);

        let mut scene = Scene::new("test".into(), map, pawn_at(0, 0));
        let result = scene.move_to(Coordinates { x: 1, y: 0 });
        assert!(matches!(result, Err(MoveError::TileBlocked)));
    }

    #[test]
    fn test_scene_step_to() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, pawn_at(1, 1));
        let tile = scene.step_to(Direction::Right).unwrap();
        assert_eq!(tile.pointer, Coordinates { x: 2, y: 1 });
    }

    #[test]
    fn test_scene_steps_to() {
        let map = basic_test_map();
        let scene = Scene::new("test".into(), map, pawn_at(0, 0));
        let steps = scene.steps_to(Coordinates { x: 2, y: 0 }).unwrap();
        assert_eq!(
            steps,
            vec![
                Coordinates { x: 0, y: 0 },
                Coordinates { x: 1, y: 0 },
                Coordinates { x: 2, y: 0 }
            ]
        );
    }

    #[test]
    fn test_scene_walk_to_success() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, pawn_at(0, 0));
        let final_tile =
            futures::executor::block_on(scene.walk_to(Coordinates { x: 2, y: 0 })).unwrap();
        assert_eq!(final_tile.pointer, Coordinates { x: 2, y: 0 });
    }

    #[test]
    fn test_scene_walk_to_fail_no_path() {
        let mut map = basic_test_map();

        // Block all possible routes
        for x in 1..3 {
            let tile = Tile {
                id: 0,
                shape: Shape::from_square(1),
                pointer: Coordinates { x, y: 0 },
                effect: Effect {
                    block: true,
                    ..Default::default()
                },
            };
            map.layers[0].tiles.push(tile);
        }

        let mut scene = Scene::new("test".into(), map, pawn_at(0, 0));
        let result = futures::executor::block_on(scene.walk_to(Coordinates { x: 2, y: 0 }));
        assert!(matches!(result, Err(MoveError::PathNotFound)));
    }
}
