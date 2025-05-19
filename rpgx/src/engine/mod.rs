pub mod map;
pub mod pawn;

use super::common::{coordinates::Coordinates, direction::Direction};
use crate::common::errors::MoveError;
use map::Map;
use pawn::Pawn;

/// RPG engine providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Engine {
    pub map: Map,
    pub pawn: Pawn,
}

impl Engine {
    pub fn new(map: Map, pawn: Pawn) -> Self {
        Self { map, pawn }
    }

    /// Walk to the target [`Coordinates`] through the best path
    pub async fn walk_to(&mut self, target_position: Coordinates) -> Result<(), MoveError> {
        let start = self.pawn.tile.pointer;
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;

        for step_coords in path {
            self.move_to(step_coords)?;
        }
        Ok(())
    }

    /// Make a step into the provided [`Direction`]
    pub fn step_to(&mut self, direction: Direction) -> Result<(), MoveError> {
        let delta = direction.to_delta();
        let target_position = self.pawn.tile.pointer + delta;
        self.move_to(target_position)?;
        Ok(())
    }

    /// Move to the provided [`Coordinates`] if allowed
    pub fn move_to(&mut self, target_position: Coordinates) -> Result<(), MoveError> {
        if self.map.is_tile_blocked(target_position) {
            return Err(MoveError::TileBlocked);
        }

        let base_layer = self.map.get_base_layer().ok_or(MoveError::TileNotFound)?;
        let tile = base_layer
            .get_tile(target_position)
            .ok_or(MoveError::TileNotFound)?;

        self.pawn.tile = tile.clone();

        // Trigger actions on all layers for the tile pointer
        self.map.trigger_actions_at(tile.pointer);

        Ok(())
    }
}
