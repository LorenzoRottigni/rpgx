use crate::prelude::Coordinates;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Shape {
    pub width: u32,
    pub height: u32,
}

impl Shape {
    pub fn from_square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    pub fn from_rectangle(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn from_bounds(start: Coordinates, end: Coordinates) -> Self {
        let width = start.x.max(end.x) - start.x.min(end.x) + 1;
        let height = start.y.max(end.y) - start.y.min(end.y) + 1;

        Self { width, height }
    }

    pub fn union(&self, other: Shape) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        coordinates.x < self.width && coordinates.y < self.height
    }

    pub fn offset_by(&self, offset: Coordinates) -> Self {
        Self {
            width: self.width + offset.x,
            height: self.height + offset.y,
        }
    }

    pub fn coordinates_in_range(&self, start: Coordinates, end: Coordinates) -> Vec<Coordinates> {
        let start_x = start.x.max(0);
        let start_y = start.y.max(0);
        let end_x = end.x.min(self.width - 1);
        let end_y = end.y.min(self.height - 1);

        let mut coords = Vec::new();
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                coords.push(Coordinates { x, y });
            }
        }
        coords
    }

    pub fn filter_coordinates<F>(&self, mut filter_fn: F) -> Vec<Coordinates>
    where
        F: FnMut(Coordinates, Shape) -> bool,
    {
        let mut coords = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x, y };
                if filter_fn(coord, *self) {
                    coords.push(coord);
                }
            }
        }

        coords
    }

    pub fn expand_to_include(&mut self, offset: Coordinates, other: Shape) {
        self.width = self.width.max(offset.x + other.width);
        self.height = self.height.max(offset.y + other.height);
    }
}

#[cfg(test)]
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::Coordinates;

    #[test]
    fn creates_square_shape() {
        let shape = Shape::from_square(5);
        assert_eq!(shape.width, 5);
        assert_eq!(shape.height, 5);
    }

    #[test]
    fn creates_rectangle_shape() {
        let shape = Shape::from_rectangle(4, 7);
        assert_eq!(shape.width, 4);
        assert_eq!(shape.height, 7);
    }

    #[test]
    fn creates_shape_from_bounds() {
        let start = Coordinates { x: 1, y: 2 };
        let end = Coordinates { x: 3, y: 4 };
        let shape = Shape::from_bounds(start, end);
        assert_eq!(shape.width, 3); // 1 to 3 is 3 tiles: 1, 2, 3
        assert_eq!(shape.height, 3); // 2 to 4 is 3 tiles: 2, 3, 4
    }

    #[test]
    fn in_bounds_checks_correctly() {
        let shape = Shape::from_rectangle(3, 3);
        assert!(shape.in_bounds(Coordinates { x: 1, y: 1 }));
        assert!(shape.in_bounds(Coordinates { x: 2, y: 2 }));
        assert!(!shape.in_bounds(Coordinates { x: 3, y: 3 }));
    }

    #[test]
    fn coordinates_in_range_returns_correct_coordinates() {
        let shape = Shape::from_rectangle(4, 4);
        let coords =
            shape.coordinates_in_range(Coordinates { x: 1, y: 1 }, Coordinates { x: 2, y: 2 });

        let expected = vec![
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 2, y: 1 },
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 2, y: 2 },
        ];

        assert_eq!(coords, expected);
    }

    #[test]
    fn filter_coordinates_filters_correctly() {
        let shape = Shape::from_rectangle(3, 3);
        let even_coords = shape.filter_coordinates(|c, _s| (c.x + c.y) % 2 == 0);

        let expected = vec![
            Coordinates { x: 0, y: 0 },
            Coordinates { x: 2, y: 0 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 0, y: 2 },
            Coordinates { x: 2, y: 2 },
        ];

        assert_eq!(even_coords, expected);
    }

    #[test]
    fn expand_to_include_updates_shape() {
        let mut shape = Shape::from_rectangle(2, 2);
        let offset = Coordinates { x: 1, y: 1 };
        let other = Shape::from_rectangle(3, 3);

        shape.expand_to_include(offset, other);

        assert_eq!(shape.width, 4); // max of 2 and 1+3 = 4
        assert_eq!(shape.height, 4); // max of 2 and 1+3 = 4
    }
}
