use crate::prelude::Delta;

/// Represents a cardinal direction in a 2D grid.
///
/// Directions correspond to unit moves along the X and Y axes.
/// - `Up` moves along negative Y
/// - `Down` moves along positive Y
/// - `Left` moves along negative X
/// - `Right` moves along positive X
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    /// Upward direction (0, -1)
    Up,
    /// Downward direction (0, 1)
    Down,
    /// Leftward direction (-1, 0)
    Left,
    /// Rightward direction (1, 0)
    Right,
}

impl Direction {
    /// Attempts to create a `Direction` from a given `Delta`.
    ///
    /// Returns `Some(Direction)` if the delta corresponds exactly to one of
    /// the four cardinal directions (unit steps), otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::prelude::{Direction, Delta};
    ///
    /// assert_eq!(Direction::from_delta(&Delta { dx: 0, dy: -1 }), Some(Direction::Up));
    /// assert_eq!(Direction::from_delta(&Delta { dx: 2, dy: 0 }), None);
    /// ```
    pub fn from_delta(delta: &Delta) -> Option<Self> {
        if delta.dx == 0 && delta.dy == -1 {
            Some(Direction::Up)
        } else if delta.dx == 0 && delta.dy == 1 {
            Some(Direction::Down)
        } else if delta.dx == -1 && delta.dy == 0 {
            Some(Direction::Left)
        } else if delta.dx == 1 && delta.dy == 0 {
            Some(Direction::Right)
        } else {
            None
        }
    }

    /// Converts the `Direction` into a corresponding `Delta` (unit step).
    ///
    /// This is useful for grid-based movement calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::prelude::{Direction, Delta};
    ///
    /// assert_eq!(Direction::Left.to_delta(), Delta { dx: -1, dy: 0 });
    /// assert_eq!(Direction::Up.to_delta(), Delta { dx: 0, dy: -1 });
    /// ```
    pub fn to_delta(&self) -> Delta {
        match self {
            Direction::Up => Delta { dx: 0, dy: -1 },
            Direction::Down => Delta { dx: 0, dy: 1 },
            Direction::Left => Delta { dx: -1, dy: 0 },
            Direction::Right => Delta { dx: 1, dy: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Delta;

    #[test]
    fn test_from_delta_returns_direction_for_unit_steps() {
        assert_eq!(
            Direction::from_delta(&Delta { dx: 0, dy: -1 }),
            Some(Direction::Up)
        );
        assert_eq!(
            Direction::from_delta(&Delta { dx: 0, dy: 1 }),
            Some(Direction::Down)
        );
        assert_eq!(
            Direction::from_delta(&Delta { dx: -1, dy: 0 }),
            Some(Direction::Left)
        );
        assert_eq!(
            Direction::from_delta(&Delta { dx: 1, dy: 0 }),
            Some(Direction::Right)
        );
    }

    #[test]
    fn test_from_delta_returns_none_for_non_unit_steps() {
        assert_eq!(Direction::from_delta(&Delta { dx: 0, dy: 0 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: 1, dy: 1 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: -1, dy: -1 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: 2, dy: 0 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: 0, dy: 2 }), None);
    }

    #[test]
    fn test_to_delta_returns_correct_delta() {
        assert_eq!(Direction::Up.to_delta(), Delta { dx: 0, dy: -1 });
        assert_eq!(Direction::Down.to_delta(), Delta { dx: 0, dy: 1 });
        assert_eq!(Direction::Left.to_delta(), Delta { dx: -1, dy: 0 });
        assert_eq!(Direction::Right.to_delta(), Delta { dx: 1, dy: 0 });
    }
}
