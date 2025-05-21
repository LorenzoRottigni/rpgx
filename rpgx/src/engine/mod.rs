pub mod map;
pub mod pawn;
pub mod library;

use super::common::{coordinates::Coordinates, direction::Direction};
use crate::common::errors::MoveError;
use map::{tile::Tile, Map};
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
    pub async fn walk_to(&mut self, target_position: Coordinates) -> Result<Tile, MoveError> {
        let start = self.pawn.tile.pointer;
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;

        let mut tile= None;
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
        let target_position = self.pawn.tile.pointer + delta;
        Ok(self.move_to(target_position)?)
    }

    /// Move to the provided [`Coordinates`] if allowed
    pub fn move_to(&mut self, target_position: Coordinates) -> Result<Tile, MoveError> {
        if self.map.is_tile_blocked(target_position) {
            return Err(MoveError::TileBlocked);
        }

        let base_layer = self.map.get_base_layer().ok_or(MoveError::TileNotFound)?;
        let tile = base_layer
            .get_tile(target_position)
            .ok_or(MoveError::TileNotFound)?;

        self.pawn.tile = tile.clone();

        // Trigger actions on all layers for the tile pointer
        // self.map.trigger_actions_at(tile.pointer);

        // return the tile to make the caller able to dispatch its actions itself using its library
        // (allows the caller to use its context within the callback)
        Ok(tile)
    }

    /// Get steps to reach the target [`Coordinates`] from the current position
    pub fn steps_to(
        &self,
        target_position: Coordinates,
    ) -> Result<Vec<Coordinates>, MoveError> {
        let start = self.pawn.tile.pointer;
        let path = self
            .map
            .find_path(&start, &target_position)
            .ok_or(MoveError::PathNotFound)?;
        Ok(path)
    }
}
