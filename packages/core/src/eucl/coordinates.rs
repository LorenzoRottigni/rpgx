use crate::prelude::{Delta, Shape};
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Coordinates {
    pub fn bounding_box(coords: &[Self]) -> Option<(Self, Self)> {
        if coords.is_empty() {
            return None;
        }

        let min_x = coords.iter().map(|c| c.x).min()?;
        let max_x = coords.iter().map(|c| c.x).max()?;
        let min_y = coords.iter().map(|c| c.y).min()?;
        let max_y = coords.iter().map(|c| c.y).max()?;

        // Add +1 to max_x and max_y to make bounds exclusive
        Some((
            Self { x: min_x, y: min_y },
            Self {
                x: max_x + 1,
                y: max_y + 1,
            },
        ))
    }

    pub fn is_within(&self, origin: Coordinates, shape: Shape) -> bool {
        let end = origin + shape; // exclusive end coordinate
                                  // Check if self is inside [origin, end) range
        self.x >= origin.x && self.x < end.x && self.y >= origin.y && self.y < end.y
    }

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

    pub fn to_delta(self) -> Delta {
        Delta {
            dx: self.x as i32,
            dy: self.y as i32,
        }
    }
}

// Coordinates ± Coordinates
impl Add for Coordinates {
    type Output = Coordinates;
    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;
    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Sub<usize> for Coordinates {
    type Output = Coordinates;

    fn sub(self, value: usize) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(value as u32),
            y: self.y.saturating_sub(value as u32),
        }
    }
}

// Coordinates ± Shape
impl Add<Shape> for Coordinates {
    type Output = Coordinates;
    fn add(self, shape: Shape) -> Coordinates {
        Coordinates {
            x: self.x + shape.width,
            y: self.y + shape.height,
        }
    }
}

impl Sub<Shape> for Coordinates {
    type Output = Coordinates;
    fn sub(self, shape: Shape) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(shape.width),
            y: self.y.saturating_sub(shape.height),
        }
    }
}

// Coordinates ± Delta
impl Add<Delta> for Coordinates {
    type Output = Option<Coordinates>;
    fn add(self, delta: Delta) -> Self::Output {
        self.try_offseted(delta)
    }
}

impl Sub<Delta> for Coordinates {
    type Output = Option<Coordinates>;
    fn sub(self, delta: Delta) -> Self::Output {
        self.try_offseted(delta.invert())
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
/*#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bounding_box_returns_none_for_empty_slice() {
        let coords: Vec<Coordinates> = vec![];
        assert_eq!(Coordinates::bounding_box(&coords), None);
    }

    #[test]
    fn add_adds_coordinates_correctly() {
        let c1 = Coordinates { x: 3, y: 7 };
        let c2 = Coordinates { x: 1, y: 4 };
        let result = c1 + c2;
        assert_eq!(result, Coordinates { x: 4, y: 11 });
    }
}
     */
