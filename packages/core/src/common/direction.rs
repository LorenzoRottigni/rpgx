use crate::common::delta::Delta;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
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

    pub fn to_delta(&self) -> Delta {
        match self {
            Direction::Up => Delta { dx: 0, dy: -1 },
            Direction::Down => Delta { dx: 0, dy: 1 },
            Direction::Left => Delta { dx: -1, dy: 0 },
            Direction::Right => Delta { dx: 1, dy: 0 },
        }
    }
}

/*
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn from_delta_returns_correct_direction() {
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
    fn from_delta_returns_none_for_invalid_delta() {
        assert_eq!(Direction::from_delta(&Delta { dx: 0, dy: 0 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: 1, dy: 1 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: -1, dy: -1 }), None);
        assert_eq!(Direction::from_delta(&Delta { dx: 2, dy: 0 }), None);
    }

    #[test]
    fn to_delta_returns_correct_coordinates() {
        assert_eq!(Direction::Up.to_delta(), Delta { dx: 0, dy: -1 });
        assert_eq!(Direction::Down.to_delta(), Delta { dx: 0, dy: 1 });
        assert_eq!(Direction::Left.to_delta(), Delta { dx: -1, dy: 0 });
        assert_eq!(Direction::Right.to_delta(), Delta { dx: 1, dy: 0 });
    }
}
 */
