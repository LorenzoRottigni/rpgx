use crate::prelude::Coordinates;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn from_delta(delta: &Coordinates) -> Option<Self> {
        if delta.x == 0 && delta.y == -1 {
            Some(Direction::Up)
        } else if delta.x == 0 && delta.y == 1 {
            Some(Direction::Down)
        } else if delta.x == -1 && delta.y == 0 {
            Some(Direction::Left)
        } else if delta.x == 1 && delta.y == 0 {
            Some(Direction::Right)
        } else {
            None
        }
    }

    pub fn to_delta(&self) -> Coordinates {
        match self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::Coordinates;

    #[test]
    fn from_delta_returns_correct_direction() {
        assert_eq!(
            Direction::from_delta(&Coordinates { x: 0, y: -1 }),
            Some(Direction::Up)
        );
        assert_eq!(
            Direction::from_delta(&Coordinates { x: 0, y: 1 }),
            Some(Direction::Down)
        );
        assert_eq!(
            Direction::from_delta(&Coordinates { x: -1, y: 0 }),
            Some(Direction::Left)
        );
        assert_eq!(
            Direction::from_delta(&Coordinates { x: 1, y: 0 }),
            Some(Direction::Right)
        );
    }

    #[test]
    fn from_delta_returns_none_for_invalid_delta() {
        assert_eq!(Direction::from_delta(&Coordinates { x: 0, y: 0 }), None);
        assert_eq!(Direction::from_delta(&Coordinates { x: 1, y: 1 }), None);
        assert_eq!(Direction::from_delta(&Coordinates { x: -1, y: -1 }), None);
        assert_eq!(Direction::from_delta(&Coordinates { x: 2, y: 0 }), None);
    }

    #[test]
    fn to_delta_returns_correct_coordinates() {
        assert_eq!(Direction::Up.to_delta(), Coordinates { x: 0, y: -1 });
        assert_eq!(Direction::Down.to_delta(), Coordinates { x: 0, y: 1 });
        assert_eq!(Direction::Left.to_delta(), Coordinates { x: -1, y: 0 });
        assert_eq!(Direction::Right.to_delta(), Coordinates { x: 1, y: 0 });
    }
}
