#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
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

        Some((Self { x: min_x, y: min_y }, Self { x: max_x, y: max_y }))
    }
}

use std::ops::Add;

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
