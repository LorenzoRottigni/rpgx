use crate::prelude::{Coordinates, Effect, Rect, Tile};

pub trait Renderable {
    fn render(&self) -> Vec<Tile>;
}

pub trait Shaped {
    /// Returns the individual shapes of the object.
    fn get_shapes(&self) -> Vec<crate::prelude::Shape>;

    /// Returns the overall bounding shape of the object.
    fn get_shape(&self) -> crate::prelude::Shape {
        crate::prelude::Shape::bounding_shape(&self.get_shapes())
    }
}

pub trait Bounded {
    /// Returns the bounding rectangle of the object.
    fn get_bounding_rect(&self) -> Rect;
}

pub trait Spatial {
    fn contains(&self, target: &Coordinates) -> bool;
}

pub trait Grid {
    fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect>;
    fn get_actions_at(&self, target: &Coordinates) -> Vec<u32>;
    fn is_blocking_at(&self, target: &Coordinates) -> bool;
}
