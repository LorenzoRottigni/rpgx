use super::coordinates::Coordinates;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn to_delta(&self) -> Coordinates {
        match self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
        }
    }
}
