use crate::{
    errors::RPGXError,
    prelude::{Coordinates, Direction, Map, Pawn},
    traits::Grid,
};

#[doc = include_str!("../../docs/scene.md")]
/// RPG scene providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Scene {
    /// Scene name identifier.
    pub name: String,
    /// The game map with multiple layers defining terrain and obstacles.
    pub map: Map,
    /// Optional pawn currently active in the scene.
    pub pawn: Option<Pawn>,
}

impl Scene {
    /// Creates a new `Scene` with the specified name, map, and optional pawn.
    ///
    /// # Arguments
    ///
    /// * `name` - A string identifier for the scene.
    /// * `map` - The `Map` instance used in the scene.
    /// * `pawn` - Optional initial `Pawn` to place in the scene.
    pub fn new(name: String, map: Map, pawn: Option<Pawn>) -> Self {
        Self { map, pawn, name }
    }

    /// Load a [`Pawn`] into the scene at the map's default spawn position.
    ///
    /// # Arguments
    ///
    /// * `texture_id` - Identifier for the pawn's texture/sprite.
    pub fn load_pawn(&mut self, texture_id: u32) {
        self.pawn = Some(Pawn {
            texture_id,
            pointer: self.map.spawn,
        })
    }

    /// Load a [`Pawn`] into the scene at a specific location.
    ///
    /// # Arguments
    ///
    /// * `pawn` - The pawn instance with desired coordinates.
    pub fn load_pawn_at(&mut self, pawn: Pawn) {
        self.pawn = Some(pawn);
    }

    /// Walk asynchronously to the target coordinates along the best computed path.
    ///
    /// Moves the pawn step-by-step, returning the final position or an error.
    ///
    /// # Errors
    ///
    /// Returns `RPGXError` if the pawn is missing, no path is found, or a step fails.
    pub async fn walk_to(
        &mut self,
        target_position: Coordinates,
    ) -> Result<Coordinates, RPGXError> {
        let start = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or_else(|| RPGXError::TileNotFound(target_position))?;

        // Find the best path from current to target coordinates
        let path =
            self.map
                .find_path(&start, &target_position)
                .ok_or(RPGXError::PathNotFround {
                    from: start,
                    to: target_position,
                })?;

        let mut tile = None;
        // Walk each step in the path, returning early if any step fails
        for step_coords in path {
            tile = Some(self.move_to(step_coords)?);
        }

        tile.ok_or(RPGXError::WalkFailed {
            from: start,
            to: target_position,
        })
    }

    /// Take a single movement step in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction to move.
    ///
    /// # Errors
    ///
    /// Returns `RPGXError` if the pawn is missing, the target tile is invalid or blocked.
    pub fn step_to(&mut self, direction: Direction) -> Result<Coordinates, RPGXError> {
        let delta = direction.to_delta();
        let current = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or(RPGXError::PawnNotFound)?;

        // Calculate the target coordinates by applying the delta
        if let Some(target_position) = current + delta {
            self.move_to(target_position)
        } else {
            Err(RPGXError::StepFailed(direction))
        }
    }

    /// Move the pawn directly to the target coordinates if movement is allowed.
    ///
    /// Checks map blocking and updates the pawn's position if possible.
    ///
    /// # Errors
    ///
    /// Returns `RPGXError` if the pawn is missing or the target is blocked.
    pub fn move_to(&mut self, target_position: Coordinates) -> Result<Coordinates, RPGXError> {
        // Check if movement to the target is allowed by the map
        if self.map.contains(&target_position) && !self.map.is_blocking_at(&target_position) {
            if let Some(pawn) = self.pawn.as_mut() {
                pawn.pointer = target_position;
                Ok(target_position)
            } else {
                Err(RPGXError::PawnNotFound)
            }
        } else {
            Err(RPGXError::TileNotWalkable(target_position))
        }
    }

    /// Compute all the steps from the current pawn position to the target.
    ///
    /// Returns a vector of coordinates representing the path, or an error if no path.
    ///
    /// # Errors
    ///
    /// Returns `RPGXError` if the pawn is missing or no path is found.
    pub fn steps_to(&self, target_position: Coordinates) -> Result<Vec<Coordinates>, RPGXError> {
        let start = self
            .pawn
            .as_ref()
            .map(|p| p.pointer)
            .ok_or(RPGXError::PawnNotFound)?;

        let path =
            self.map
                .find_path(&start, &target_position)
                .ok_or(RPGXError::PathNotFround {
                    from: start,
                    to: target_position,
                })?;

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Layer, Map};

    // Helper to build a minimal Map with no blocking tiles
    fn create_test_map() -> Map {
        let layer = Layer::new("base".to_string(), vec![], 1);
        Map::new("test_map".to_string(), vec![layer], Coordinates::new(0, 0))
    }

    #[test]
    fn test_move_to_blocked_tile_returns_error() {
        let mut map = create_test_map();
        // Manually override map.move_allowed to simulate blocking
        // For this test, patch move_allowed to always return false
        map.layers.clear(); // no layers, no tiles = blocked
        let mut scene = Scene::new("test".into(), map, None);
        scene.load_pawn(1);

        let result = scene.move_to(Coordinates::new(0, 0));
        assert_eq!(
            result,
            Err(RPGXError::TileNotWalkable(Coordinates::new(0, 0)))
        );
    }
}
