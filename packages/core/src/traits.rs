use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape};

pub trait Shaped {
    fn get_shape(&self) -> Shape;
}

pub trait Shiftable {
    /// Offsets the object by the given delta.
    fn offset(&mut self, delta: Delta);
    fn translate(&self, delta: Delta) -> Self;
}

pub trait Grid {
    fn contains(&self, coord: &Coordinates) -> bool;
    // fn get_tiles_at(&self, pointer: Coordinates) -> Vec<Rect>;
    // fn is_blocking_at(&self, target: &Coordinates) -> bool;
    // fn move_allowed(&self, target: Coordinates) -> bool;
}
