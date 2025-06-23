use crate::prelude::{Coordinates, Delta, Shape};

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
}
