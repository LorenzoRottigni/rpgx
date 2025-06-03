use crate::prelude::{Coordinates, Direction, Map, MoveError, Pawn, Tile};

/// RPG scene providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Scene<'a> {
    pub name: String,
    pub map: Map<'a>,
    pub pawn: Pawn,
}

impl<'a> Scene<'a> {
    pub async fn walk_to(&'a mut self, target_position: Coordinates) -> bool {
        let start = self.pawn.pointer;
        let path = self.map.find_path(&start, &target_position);

        for step_coords in path.unwrap() {
            if !self.move_to(step_coords) {
                return false;
            }
        }

        true
    }

    pub fn step_to(&'a mut self, direction: Direction) -> bool {
        let delta = direction.to_delta();
        let target_position = self.pawn.pointer + delta;
        self.move_to(target_position)
    }

    pub fn move_to(&mut self, target_position: Coordinates) -> bool {
        if self.map.is_blocking_at(target_position) {
            return false;
        }

        let base_layer = match self.map.get_base_layer() {
            Some(layer) => layer,
            None => return false,
        };

        let tile = match base_layer.get_tile_at(target_position) {
            Some(tile) => tile.clone(),
            None => return false,
        };

        self.pawn.pointer = tile.pointer;

        true
    }
}
