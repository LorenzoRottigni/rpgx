use crate::prelude::{Coordinates, Direction, Map, MoveError, Pawn};

#[doc = include_str!("../docs/scene.md")]
/// RPG scene providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Scene {
    /// Scene name identifier
    pub name: String,
    /// The game map with multiple layers
    pub map: Map,
    /// Optional pawn currently in the scene
    pub pawn: Option<Pawn>,
}

impl Scene {
    /// Create a new `Scene` with name, map and optional pawn
    pub fn new(name: String, map: Map, pawn: Option<Pawn>) -> Self {
        Self { map, pawn, name }
    }

    /// Load a [`Pawn`] in the [`Scene`] using default [`Map`] spawn location
    pub fn load_pawn(&mut self, texture_id: u32) {
        self.pawn = Some(Pawn {
            texture_id,
            pointer: self.map.spawn,
        })
    }

    /// Load a [`Pawn`] in the [`Scene`] overriding default [`Map`] spawn location
    pub fn load_pawn_at(&mut self, pawn: Pawn) {
        self.pawn = Some(pawn);
    }

    /// Walk to the target [`Coordinates`] through the best path asynchronously
    ///
    /// Follows the computed path step-by-step, returning the final tile or an error.
    pub async fn walk_to(
        &mut self,
        target_position: Coordinates,
    ) -> Result<Coordinates, MoveError> {
        let start = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or(MoveError::TileNotFound)?;

        // Find path from current pawn position to target
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;

        let mut tile = None;
        // Move pawn along the path step-by-step
        for step_coords in path {
            tile = Some(self.move_to(step_coords)?);
        }

        tile.ok_or(MoveError::TileNotFound)
    }

    /// Make a single step in the given [`Direction`]
    ///
    /// Returns the tile stepped onto or an error if movement is blocked or invalid.
    pub fn step_to(&mut self, direction: Direction) -> Result<Coordinates, MoveError> {
        let delta = direction.to_delta();
        let current = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or(MoveError::TileNotFound)?;

        // Compute new position by applying direction delta
        if let Some(target_position) = current + delta {
            self.move_to(target_position)
        } else {
            Err(MoveError::TileNotFound)
        }
    }

    /// Move pawn directly to the specified [`Coordinates`] if accessible
    ///
    /// Checks for blocking tiles and updates the pawn's pointer.
    pub fn move_to(&mut self, target_position: Coordinates) -> Result<Coordinates, MoveError> {
        // if self.map.is_blocking_at(target_position) {
        //     return Err(MoveError::TileBlocked);
        // }

        // self.map.movement_allowed(); pawn.pointer = target_position:
        if self.map.move_allowed(target_position) {
            if let Some(pawn) = self.pawn.as_mut() {
                pawn.pointer = target_position;
                Ok(target_position)
            } else {
                Err(MoveError::TileNotFound)
            }
        } else {
            Err(MoveError::TileNotFound)
        }
        // let base_layer = self.map.get_base_layer().ok_or(MoveError::TileNotFound)?;
        // let tile = base_layer
        //     .get_tile_at(target_position)
        //     .ok_or(MoveError::TileNotFound)?;
        //
        // if let Some(pawn) = self.pawn.as_mut() {
        //     pawn.pointer = tile.pointer;
        // } else {
        //     return Err(MoveError::TileNotFound);
        // }
        //
        // Ok(tile)
    }

    /// Get all steps from the current pawn position to the target [`Coordinates`]
    ///
    /// Returns a vector of coordinates for the computed path or an error.
    pub fn steps_to(&self, target_position: Coordinates) -> Result<Vec<Coordinates>, MoveError> {
        let start = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or(MoveError::TileNotFound)?;

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

    /// Helper: create a basic 3x3 map with a single texture layer, no blocks
    fn basic_test_map() -> Map {
        let shape = Shape {
            width: 3,
            height: 3,
        };
        let masks = vec![];
        let layer = Layer::new("ground".to_string(), masks, 1);
        Map::new("test_map".to_string(), vec![layer], Coordinates::default())
    }

    /// Helper: create a pawn at given coordinates with texture_id=0
    fn pawn_at(x: u32, y: u32) -> Pawn {
        Pawn {
            pointer: Coordinates { x, y },
            texture_id: 0,
        }
    }

    #[test]
    fn test_scene_move_to_success() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, Some(pawn_at(0, 0)));
        let pointer = scene.move_to(Coordinates { x: 1, y: 0 }).unwrap();
        assert_eq!(pointer, Coordinates { x: 1, y: 0 });
    }

    /* #[test]
    fn test_scene_move_to_blocked() {
        let mut map = basic_test_map();

        // Manually block tile at (1, 0)
        let tile = Tile {
            shape: Shape::from_square(1),
            pointer: Coordinates { x: 1, y: 0 },
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        map.layers[0].tiles.push(tile);

        let mut scene = Scene::new("test".into(), map, Some(pawn_at(0, 0)));
        let result = scene.move_to(Coordinates { x: 1, y: 0 });
        assert!(matches!(result, Err(MoveError::TileBlocked)));
    } */

    #[test]
    fn test_scene_step_to() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, Some(pawn_at(1, 1)));
        let pointer = scene.step_to(Direction::Right).unwrap();
        assert_eq!(pointer, Coordinates { x: 2, y: 1 });
    }

    #[test]
    fn test_scene_step_to_out_of_bounds() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, Some(pawn_at(2, 2)));
        let result = scene.step_to(Direction::Right);
        assert!(matches!(result, Err(MoveError::TileNotFound)));
    }

    #[test]
    fn test_scene_steps_to() {
        let map = basic_test_map();
        let scene = Scene::new("test".into(), map, Some(pawn_at(0, 0)));
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
    fn test_scene_steps_to_no_pawn() {
        let map = basic_test_map();
        let scene = Scene::new("no_pawn".into(), map, None);
        let result = scene.steps_to(Coordinates { x: 1, y: 0 });
        assert!(matches!(result, Err(MoveError::TileNotFound)));
    }

    #[test]
    fn test_scene_walk_to_success() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, Some(pawn_at(0, 0)));
        let pointer =
            futures::executor::block_on(scene.walk_to(Coordinates { x: 2, y: 0 })).unwrap();
        assert_eq!(pointer, Coordinates { x: 2, y: 0 });
    }

    /* #[test]
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

        let mut scene = Scene::new("test".into(), map, Some(pawn_at(0, 0)));
        let result = futures::executor::block_on(scene.walk_to(Coordinates { x: 2, y: 0 }));
        assert!(matches!(result, Err(MoveError::PathNotFound)));
    }*/

    #[test]
    fn test_scene_no_pawn_error() {
        let map = basic_test_map();
        let mut scene = Scene::new("no_pawn".into(), map, None);
        let result = scene.move_to(Coordinates { x: 1, y: 0 });
        assert!(matches!(result, Err(MoveError::TileNotFound)));
    }

    #[test]
    fn test_load_pawn_and_load_pawn_at() {
        let map = basic_test_map();
        let mut scene = Scene::new("test".into(), map, None);

        // Test load_pawn sets pawn at map spawn
        scene.load_pawn(7);
        assert!(scene.pawn.is_some());
        assert_eq!(scene.pawn.as_ref().unwrap().texture_id, 7);
        assert_eq!(scene.pawn.as_ref().unwrap().pointer, scene.map.spawn);

        // Test load_pawn_at overrides pawn
        let custom_pawn = pawn_at(2, 2);
        scene.load_pawn_at(custom_pawn.clone());
        assert_eq!(scene.pawn.as_ref().unwrap().pointer, custom_pawn.pointer);
    }
}
