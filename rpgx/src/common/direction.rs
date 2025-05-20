use super::coordinates::Coordinates;

#[derive(Debug)]
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
