use crate::prelude::{Coordinates, Delta, Effect, Shape, Tile};

pub trait Shaped {
    fn get_shape(&self) -> Shape;
}

pub trait Shiftable {
    /// Offsets the object by the given delta.
    fn offset(&mut self, delta: Delta);
    fn translate(&self, delta: Delta) -> Self;
}

pub trait Grid {
    fn contains(&self, coord: Coordinates) -> bool;
    fn get_tiles_at(&self, pointer: Coordinates) -> Vec<Tile>;
    fn is_blocking_at(&self, target: &Coordinates) -> bool;
    fn move_allowed(&self, target: Coordinates) -> bool;
    fn get_actions_at(&self, pointer: Coordinates) -> Vec<u32>;
    fn get_effects_at(&self, pointer: Coordinates) -> Vec<Effect>;
}
