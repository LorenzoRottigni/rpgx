use crate::{
    common::rect::Rect,
    prelude::{Coordinates, Shape},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    /// Selects a single tile at the given coordinates.
    Single(Coordinates),

    /// Selects a rectangular block of tiles defined by a Rect.
    Block(Rect),

    Sparse(Vec<Coordinates>),
}

impl Selector {
    pub fn get_shape(&self) -> Shape {
        match self {
            Selector::Single(_) => Shape::from_square(1),
            Selector::Block(rect) => rect.shape,
            Selector::Sparse(coords) => {
                if coords.is_empty() {
                    return Shape::default();
                }

                let (min_x, max_x) = coords
                    .iter()
                    .map(|c| c.x)
                    .fold((u32::MAX, 0), |(min, max), x| (min.min(x), max.max(x)));

                let (min_y, max_y) = coords
                    .iter()
                    .map(|c| c.y)
                    .fold((u32::MAX, 0), |(min, max), y| (min.min(y), max.max(y)));

                Shape::from_bounds(
                    Coordinates { x: min_x, y: min_y },
                    Coordinates {
                        x: max_x + 1,
                        y: max_y + 1,
                    },
                )
            }
        }
    }

    pub fn get_shape_offset(&self) -> Coordinates {
        match self {
            Selector::Single(coord) => *coord,
            Selector::Block(rect) => rect.origin, // Use Rect.origin instead of tuple destructuring
            Selector::Sparse(coords) => {
                if coords.is_empty() {
                    Coordinates { x: 0, y: 0 }
                } else {
                    let min_x = coords.iter().map(|c| c.x).min().unwrap();
                    let min_y = coords.iter().map(|c| c.y).min().unwrap();
                    Coordinates { x: min_x, y: min_y }
                }
            }
        }
    }

    pub fn get_absolute_shape(&self) -> Shape {
        match self {
            Selector::Single(_coord) => Shape {
                width: 1, // Single tile is always 1x1
                height: 1,
            },

            Selector::Block(rect) => rect.shape, // Just return Rect shape

            Selector::Sparse(coords) => {
                if coords.is_empty() {
                    return Shape::default();
                }

                let max_x = coords.iter().map(|c| c.x).max().unwrap_or(0);
                let max_y = coords.iter().map(|c| c.y).max().unwrap_or(0);

                Shape {
                    width: max_x + 1,
                    height: max_y + 1,
                }
            }
        }
    }
}
