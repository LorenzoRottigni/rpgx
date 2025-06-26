#[doc = include_str!("../../docs/delta.md")]
/// Represents a 2D movement or directional offset with signed deltas.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

/// Constructors
impl Delta {
    /// Creates a new delta with given x and y differences.
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    /// Returns a zero delta (no movement).
    pub fn zero() -> Self {
        Self { dx: 0, dy: 0 }
    }
}

/// Utilities
impl Delta {
    /// Returns the inverse of the delta (negates both components).
    pub fn invert(self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /// Returns true if the delta is zero.
    pub fn is_zero(self) -> bool {
        self.dx == 0 && self.dy == 0
    }

    /// Returns the Manhattan distance represented by this delta.
    pub fn manhattan(self) -> u32 {
        self.dx.unsigned_abs() + self.dy.unsigned_abs()
    }

    /// Returns true if the delta moves in only one axis.
    pub fn is_axis_aligned(self) -> bool {
        (self.dx == 0 && self.dy != 0) || (self.dy == 0 && self.dx != 0)
    }

    /// Returns true if this delta is diagonal (nonzero dx and dy).
    pub fn is_diagonal(self) -> bool {
        self.dx != 0 && self.dy != 0
    }
}

/// Arithmetic operations
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

impl Add for Delta {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl Sub for Delta {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl Neg for Delta {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.invert()
    }
}

impl AddAssign for Delta {
    fn add_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl SubAssign for Delta {
    fn sub_assign(&mut self, rhs: Self) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_delta() {
        let d = Delta::new(3, -2);
        assert_eq!(d.dx, 3);
        assert_eq!(d.dy, -2);
    }

    #[test]
    fn inverts_delta_correctly() {
        let d = Delta::new(2, -5);
        assert_eq!(d.invert(), Delta::new(-2, 5));
    }

    #[test]
    fn checks_zero_delta() {
        assert!(Delta::new(0, 0).is_zero());
        assert!(!Delta::new(1, 0).is_zero());
    }

    #[test]
    fn computes_manhattan_distance() {
        let d = Delta::new(-3, 4);
        assert_eq!(d.manhattan(), 7);
    }

    #[test]
    fn detects_axis_alignment() {
        assert!(Delta::new(0, 5).is_axis_aligned());
        assert!(Delta::new(-3, 0).is_axis_aligned());
        assert!(!Delta::new(1, 1).is_axis_aligned());
    }

    #[test]
    fn detects_diagonal_movement() {
        assert!(Delta::new(1, 1).is_diagonal());
        assert!(!Delta::new(0, 5).is_diagonal());
    }

    #[test]
    fn adds_and_subtracts_deltas() {
        let a = Delta::new(1, 2);
        let b = Delta::new(3, -1);
        assert_eq!(a + b, Delta::new(4, 1));
        assert_eq!(a - b, Delta::new(-2, 3));
    }

    #[test]
    fn negates_delta() {
        let d = Delta::new(4, -3);
        assert_eq!(-d, Delta::new(-4, 3));
    }

    #[test]
    fn add_assigns_and_sub_assigns() {
        let mut d = Delta::new(1, 1);
        d += Delta::new(2, 3);
        assert_eq!(d, Delta::new(3, 4));
        d -= Delta::new(1, 2);
        assert_eq!(d, Delta::new(2, 2));
    }
}
