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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bounding_box_returns_none_for_empty_slice() {
        let coords: Vec<Coordinates> = vec![];
        assert_eq!(Coordinates::bounding_box(&coords), None);
    }

    #[test]
    fn bounding_box_returns_correct_min_max_coordinates() {
        let coords = vec![
            Coordinates { x: 2, y: 3 },
            Coordinates { x: 5, y: 1 },
            Coordinates { x: -1, y: 4 },
            Coordinates { x: 3, y: 0 },
        ];
        let bbox = Coordinates::bounding_box(&coords);
        assert_eq!(
            bbox,
            Some((Coordinates { x: -1, y: 0 }, Coordinates { x: 5, y: 4 }))
        );
    }

    #[test]
    fn add_adds_coordinates_correctly() {
        let c1 = Coordinates { x: 3, y: 7 };
        let c2 = Coordinates { x: -1, y: 4 };
        let result = c1 + c2;
        assert_eq!(result, Coordinates { x: 2, y: 11 });
    }
}
