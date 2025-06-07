#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

        Some((Self { x: min_x, y: min_y }, Self { x: max_x, y: max_y }))
    }
}

use std::ops::Add;

use crate::common::delta::Delta;

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Delta> for Coordinates {
    type Output = Option<Coordinates>;

    fn add(self, delta: Delta) -> Self::Output {
        let x = self.x as i32 + delta.dx;
        let y = self.y as i32 + delta.dy;

        if x >= 0 && y >= 0 {
            Some(Coordinates {
                x: x as u32,
                y: y as u32,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
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
