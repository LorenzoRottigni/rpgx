use std::ops::{Add, Div, Sub};

use crate::prelude::{Coordinates, Delta};

/// Represents a rectangular area by its width and height.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Shape {
    pub width: u32,
    pub height: u32,
}

/// Constructors
impl Shape {
    /// Creates a square shape with equal width and height.
    pub fn from_square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    /// Creates a rectangular shape from width and height.
    pub fn from_rectangle(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Creates a shape that covers the area between two coordinates.
    pub fn from_bounds(start: Coordinates, end: Coordinates) -> Self {
        let width = end.x.saturating_sub(start.x);
        let height = end.y.saturating_sub(start.y);
        Shape { width, height }
    }

    /// Computes the smallest shape that can encompass all given shapes.
    pub fn bounding_shape(shapes: &[Self]) -> Self {
        shapes
            .iter()
            .fold(Self::default(), |acc, shape| acc.union(*shape))
    }
}

/// Core utilities
impl Shape {
    /// Computes the union of two shapes by taking the max of each dimension.
    pub fn union(&self, other: Shape) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    /// Checks if the given coordinates fall within the shape.
    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        coordinates.x < self.width && coordinates.y < self.height
    }

    /// Checks if the given delta is within bounds (non-negative and smaller than shape).
    pub fn delta_in_bounds(&self, delta: Delta) -> bool {
        delta.dx >= 0
            && delta.dy >= 0
            && (delta.dx as u32) < self.width
            && (delta.dy as u32) < self.height
    }

    /// Returns the area (width * height) of this shape.
    pub fn area(&self) -> u32 {
        self.width.saturating_mul(self.height)
    }

    /// Returns a new shape offset by the given coordinates.
    pub fn offset_by(&self, offset: Coordinates) -> Self {
        Self {
            width: self.width + offset.x,
            height: self.height + offset.y,
        }
    }

    /// Expands this shape to include another shape placed at an offset.
    pub fn expand_to_include(&mut self, offset: Coordinates, other: Shape) {
        self.width = self.width.max(offset.x + other.width);
        self.height = self.height.max(offset.y + other.height);
    }
}

/// Iteration and filtering
impl Shape {
    /// Iterates through coordinates in a given range clamped by this shape.
    pub fn coordinates_in_range(&self, start: Coordinates, end: Coordinates) -> Vec<Coordinates> {
        let start_x = start.x.max(0);
        let start_y = start.y.max(0);
        let end_x = end.x.min(self.width);
        let end_y = end.y.min(self.height);

        let mut coords = Vec::new();
        for y in start_y..end_y {
            for x in start_x..end_x {
                coords.push(Coordinates { x, y });
            }
        }
        coords
    }

    /// Returns coordinates that satisfy a filter predicate.
    pub fn filter_coordinates<F>(&self, mut filter_fn: F) -> Vec<Coordinates>
    where
        F: FnMut(Coordinates, Shape) -> bool,
    {
        let mut coords = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x, y };
                if filter_fn(coord, *self) {
                    coords.push(coord);
                }
            }
        }

        coords
    }
}

/// Arithmetic operations between Shapes
impl Add for Shape {
    type Output = Shape;

    fn add(self, other: Shape) -> Shape {
        Shape {
            width: self.width.saturating_add(other.width),
            height: self.height.saturating_add(other.height),
        }
    }
}

impl Sub for Shape {
    type Output = Shape;

    fn sub(self, other: Shape) -> Shape {
        Shape {
            width: self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
        }
    }
}

/// Arithmetic with scalar values
impl Add<u32> for Shape {
    type Output = Shape;

    fn add(self, value: u32) -> Shape {
        Shape {
            width: self.width.saturating_add(value),
            height: self.height.saturating_add(value),
        }
    }
}

impl Sub<u32> for Shape {
    type Output = Shape;

    fn sub(self, value: u32) -> Shape {
        Shape {
            width: self.width.saturating_sub(value),
            height: self.height.saturating_sub(value),
        }
    }
}

/// Arithmetic with coordinates
impl Add<Coordinates> for Shape {
    type Output = Shape;

    fn add(self, coordinates: Coordinates) -> Shape {
        Shape {
            width: self.width.saturating_add(coordinates.x),
            height: self.height.saturating_add(coordinates.y),
        }
    }
}

impl Sub<Coordinates> for Shape {
    type Output = Shape;

    fn sub(self, coordinates: Coordinates) -> Shape {
        Shape {
            width: self.width.saturating_sub(coordinates.x),
            height: self.height.saturating_sub(coordinates.y),
        }
    }
}

/// Allows dividing the dimensions of a shape by a scalar.
impl Div<u32> for Shape {
    type Output = Shape;

    fn div(self, divisor: u32) -> Shape {
        Shape {
            width: self.width / divisor,
            height: self.height / divisor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_shape() {
        let shape = Shape::from_rectangle(8, 6);
        let divided = shape / 2;
        assert_eq!(divided, Shape::from_rectangle(4, 3));
    }

    #[test]
    fn area_is_correct() {
        let shape = Shape::from_rectangle(4, 3);
        assert_eq!(shape.area(), 12);
    }

    #[test]
    fn checked_delta_bounds() {
        let shape = Shape::from_rectangle(5, 5);
        assert!(shape.delta_in_bounds(Delta { dx: 4, dy: 4 }));
        assert!(!shape.delta_in_bounds(Delta { dx: -1, dy: 2 }));
        assert!(!shape.delta_in_bounds(Delta { dx: 5, dy: 0 }));
    }

    #[test]
    fn creates_square_shape() {
        let shape = Shape::from_square(5);
        assert_eq!(shape.width, 5);
        assert_eq!(shape.height, 5);
    }

    #[test]
    fn computes_shape_from_bounds() {
        let start = Coordinates { x: 2, y: 3 };
        let end = Coordinates { x: 7, y: 8 };
        let shape = Shape::from_bounds(start, end);
        assert_eq!(shape, Shape::from_rectangle(5, 5));
    }

    #[test]
    fn shape_in_bounds_check() {
        let shape = Shape::from_rectangle(4, 4);
        assert!(shape.in_bounds(Coordinates { x: 3, y: 3 }));
        assert!(!shape.in_bounds(Coordinates { x: 4, y: 0 }));
        assert!(!shape.in_bounds(Coordinates { x: 0, y: 4 }));
    }

    #[test]
    fn union_of_shapes() {
        let a = Shape::from_rectangle(4, 2);
        let b = Shape::from_rectangle(3, 5);
        assert_eq!(a.union(b), Shape::from_rectangle(4, 5));
    }

    #[test]
    fn offset_and_expand_shape() {
        let mut shape = Shape::from_rectangle(4, 4);
        shape.expand_to_include(Coordinates { x: 3, y: 3 }, Shape::from_rectangle(4, 2));
        assert_eq!(shape, Shape::from_rectangle(7, 5));
    }

    #[test]
    fn filter_coordinates_works() {
        let shape = Shape::from_rectangle(3, 3);
        let filtered = shape.filter_coordinates(|coord, _| coord.x == coord.y);
        assert_eq!(
            filtered,
            vec![
                Coordinates { x: 0, y: 0 },
                Coordinates { x: 1, y: 1 },
                Coordinates { x: 2, y: 2 },
            ]
        );
    }

    #[test]
    fn shape_arithmetic_operations() {
        let a = Shape::from_rectangle(5, 5);
        let b = Shape::from_rectangle(2, 3);
        assert_eq!(a + b, Shape::from_rectangle(7, 8));
        assert_eq!(a - b, Shape::from_rectangle(3, 2));

        assert_eq!(a + 2, Shape::from_rectangle(7, 7));
        assert_eq!(a - 2, Shape::from_rectangle(3, 3));

        let coord = Coordinates { x: 1, y: 2 };
        assert_eq!(a + coord, Shape::from_rectangle(6, 7));
        assert_eq!(a - coord, Shape::from_rectangle(4, 3));
    }
}
