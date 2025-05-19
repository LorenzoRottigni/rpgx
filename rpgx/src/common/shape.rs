use super::coordinates::Coordinates;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Shape {
    pub width: i32,
    pub height: i32,
}

impl Shape {
    pub fn from_square(side: i32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    pub fn from_rectangle(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn from_bounds(start: Coordinates, end: Coordinates) -> Self {
        let width = (start.x.max(end.x) - start.x.min(end.x) + 1).abs();
        let height = (start.y.max(end.y) - start.y.min(end.y) + 1).abs();

        Self { width, height }
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        coordinates.x >= 0
            && coordinates.x < self.width
            && coordinates.y >= 0
            && coordinates.y < self.height
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
        F: FnMut(Coordinates) -> bool,
    {
        let mut coords = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x, y };
                if filter_fn(coord) {
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
