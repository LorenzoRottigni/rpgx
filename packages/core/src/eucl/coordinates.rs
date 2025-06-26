use crate::prelude::{Delta, Shape};
use std::ops::{Add, AddAssign, Sub};

#[doc = include_str!("../../docs/coordinates.md")]
/// Represents a 2D grid coordinate with `x` and `y` components.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

/// Constructors
impl Coordinates {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    /// Computes the exclusive bounding box that contains all the given coordinates.
    ///
    /// Returns `None` if the input slice is empty.
    pub fn bounding_box(coords: &[Self]) -> Option<(Self, Self)> {
        if coords.is_empty() {
            return None;
        }

        let min_x = coords.iter().map(|c| c.x).min()?;
        let max_x = coords.iter().map(|c| c.x).max()?;
        let min_y = coords.iter().map(|c| c.y).min()?;
        let max_y = coords.iter().map(|c| c.y).max()?;

        // Return exclusive bounds: [min, max)
        Some((
            Self { x: min_x, y: min_y },
            Self {
                x: max_x + 1,
                y: max_y + 1,
            },
        ))
    }
}

/// Coordinates utils
impl Coordinates {
    /// Returns true if the coordinate is at the origin (0, 0).
    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Returns true if the two coordinates share either x or y axis.
    pub fn is_aligned_with(self, other: Self) -> bool {
        self.x == other.x || self.y == other.y
    }
}

/// Shape integration
impl Coordinates {
    /// Checks whether this coordinate lies within the rectangular area
    /// defined by an origin and a `Shape` (width × height).
    ///
    /// The bounds are half-open: `[origin, origin + shape)`.
    pub fn is_within(&self, origin: Coordinates, shape: Shape) -> bool {
        let end = origin + shape;
        self.x >= origin.x && self.x < end.x && self.y >= origin.y && self.y < end.y
    }
}

/// Delta integration
impl Coordinates {
    /// Returns a new coordinate by applying the given `Delta`.
    /// Will saturate at `u32::MIN` or `u32::MAX` when under-/overflowing.
    pub fn offseted(self, delta: Delta) -> Self {
        let x = if delta.dx < 0 {
            self.x.saturating_sub((-delta.dx) as u32)
        } else {
            self.x.saturating_add(delta.dx as u32)
        };

        let y = if delta.dy < 0 {
            self.y.saturating_sub((-delta.dy) as u32)
        } else {
            self.y.saturating_add(delta.dy as u32)
        };

        Coordinates { x, y }
    }

    /// Returns a new coordinate by applying the given `Delta`, or `None` if it would
    /// result in a negative coordinate (invalid in unsigned space).
    pub fn try_offseted(self, delta: Delta) -> Option<Self> {
        let x = self.x as i32 + delta.dx;
        let y = self.y as i32 + delta.dy;
        if x >= 0 && y >= 0 {
            Some(Self {
                x: x as u32,
                y: y as u32,
            })
        } else {
            None
        }
    }

    /// Converts this coordinate into a `Delta` where `dx = x` and `dy = y`.
    pub fn to_delta(self) -> Delta {
        Delta {
            dx: self.x as i32,
            dy: self.y as i32,
        }
    }
}

impl Add for Coordinates {
    type Output = Coordinates;

    /// Adds another coordinate component-wise.
    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    /// Subtracts another coordinate component-wise using saturating subtraction.
    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Add<Shape> for Coordinates {
    type Output = Coordinates;

    /// Adds a `Shape` (width, height) to the coordinate.
    fn add(self, shape: Shape) -> Coordinates {
        Coordinates {
            x: self.x + shape.width,
            y: self.y + shape.height,
        }
    }
}

impl Sub<Shape> for Coordinates {
    type Output = Coordinates;

    /// Subtracts a `Shape` (width, height) from the coordinate using saturating subtraction.
    fn sub(self, shape: Shape) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(shape.width),
            y: self.y.saturating_sub(shape.height),
        }
    }
}

impl Add<Delta> for Coordinates {
    type Output = Option<Coordinates>;

    /// Adds a `Delta`, returning `None` if the result is negative.
    fn add(self, delta: Delta) -> Self::Output {
        self.try_offseted(delta)
    }
}

impl Sub<Delta> for Coordinates {
    type Output = Option<Coordinates>;

    /// Subtracts a `Delta`, returning `None` if the result is negative.
    fn sub(self, delta: Delta) -> Self::Output {
        self.try_offseted(delta.invert())
    }
}

impl AddAssign for Coordinates {
    /// Adds another coordinate in-place component-wise.
    fn add_assign(&mut self, rhs: Coordinates) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Delta, Shape};

    #[test]
    fn computes_bounding_box_of_coordinates() {
        // The system computes the smallest bounding rectangle that contains all coordinates.
        let coords = vec![
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 3, y: 4 },
            Coordinates { x: 2, y: 1 },
        ];
        let (min, max) = Coordinates::bounding_box(&coords).unwrap();
        assert_eq!(min, Coordinates { x: 1, y: 1 });
        assert_eq!(max, Coordinates { x: 4, y: 5 }); // Exclusive bounds
    }

    #[test]
    fn returns_none_for_empty_bounding_box() {
        // When no coordinates are provided, the system returns None.
        assert_eq!(Coordinates::bounding_box(&[]), None);
    }

    #[test]
    fn verifies_bounds_are_exclusive() {
        // The system ensures that the bounding box max values are exclusive.
        let coords = vec![Coordinates { x: 0, y: 0 }, Coordinates { x: 2, y: 2 }];
        let (_, max) = Coordinates::bounding_box(&coords).unwrap();
        assert_eq!(max, Coordinates { x: 3, y: 3 }); // Exclusive (2 + 1)
    }

    #[test]
    fn determines_if_coordinate_is_within_shape() {
        // A coordinate is tested for containment within a rectangular shape.
        let origin = Coordinates { x: 2, y: 2 };
        let shape = Shape {
            width: 3,
            height: 3,
        };
        assert!(Coordinates { x: 3, y: 4 }.is_within(origin, shape));
        assert!(!Coordinates { x: 5, y: 5 }.is_within(origin, shape));
    }

    #[test]
    fn offsets_coordinate_with_saturation() {
        // The coordinate is offset using a delta with saturating arithmetic.
        let coord = Coordinates { x: 5, y: 5 };
        let delta = Delta { dx: -2, dy: 3 };
        assert_eq!(coord.offseted(delta), Coordinates { x: 3, y: 8 });
    }

    #[test]
    fn attempts_safe_offset_with_optional_return() {
        // The system returns Some if offset remains within valid range, None otherwise.
        let coord = Coordinates { x: 1, y: 1 };
        assert_eq!(
            coord.try_offseted(Delta { dx: -1, dy: -1 }),
            Some(Coordinates { x: 0, y: 0 })
        );
        assert_eq!(coord.try_offseted(Delta { dx: -2, dy: -2 }), None);
    }

    #[test]
    fn converts_coordinate_to_delta() {
        // The coordinate is converted into a delta with matching components.
        let coord = Coordinates { x: 7, y: 9 };
        assert_eq!(coord.to_delta(), Delta { dx: 7, dy: 9 });
    }

    #[test]
    fn adds_coordinates_component_wise() {
        // Two coordinates are added by summing their x and y components.
        let a = Coordinates { x: 1, y: 2 };
        let b = Coordinates { x: 3, y: 4 };
        assert_eq!(a + b, Coordinates { x: 4, y: 6 });
    }

    #[test]
    fn subtracts_coordinates_with_saturation() {
        // Subtraction between coordinates saturates at zero.
        let a = Coordinates { x: 1, y: 1 };
        let b = Coordinates { x: 2, y: 3 };
        assert_eq!(a - b, Coordinates { x: 0, y: 0 });
    }

    #[test]
    fn adds_shape_to_coordinate() {
        // A shape is added to a coordinate as width and height.
        let c = Coordinates { x: 3, y: 4 };
        let s = Shape {
            width: 2,
            height: 1,
        };
        assert_eq!(c + s, Coordinates { x: 5, y: 5 });
    }

    #[test]
    fn subtracts_shape_using_saturation() {
        // A shape is subtracted from a coordinate with saturation on underflow.
        let c = Coordinates { x: 3, y: 2 };
        let s = Shape {
            width: 4,
            height: 3,
        };
        assert_eq!(c - s, Coordinates { x: 0, y: 0 });
    }

    #[test]
    fn adds_delta_returning_option() {
        // Adding a delta returns Some when valid, None when underflow would occur.
        let c = Coordinates { x: 2, y: 2 };
        assert_eq!(c + Delta { dx: 1, dy: 1 }, Some(Coordinates { x: 3, y: 3 }));
        assert_eq!(c + Delta { dx: -3, dy: 0 }, None);
    }

    #[test]
    fn subtracts_delta_by_inverting_and_applying_offset() {
        // Subtracting a delta is equivalent to adding its inverse.
        let c = Coordinates { x: 3, y: 3 };
        assert_eq!(c - Delta { dx: 1, dy: 2 }, Some(Coordinates { x: 2, y: 1 }));
        assert_eq!(c - Delta { dx: 5, dy: 5 }, None);
    }

    #[test]
    fn adds_assign_coordinate_components() {
        // A coordinate is modified in-place by another coordinate.
        let mut a = Coordinates { x: 1, y: 1 };
        a += Coordinates { x: 2, y: 3 };
        assert_eq!(a, Coordinates { x: 3, y: 4 });
    }

    #[test]
    fn are_bound_exclusive() {
        // The maximum coordinate in the bounding box should be one more than the highest point.
        let coords = vec![
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 4, y: 2 },
            Coordinates { x: 3, y: 5 },
        ];
        let (_, max) = Coordinates::bounding_box(&coords).unwrap();
        assert_eq!(max, Coordinates { x: 5, y: 6 }); // Exclusive upper bounds
    }
}
