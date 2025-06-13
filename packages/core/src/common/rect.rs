use crate::{
    common::delta::Delta,
    prelude::{Coordinates, Shape},
};

/// Axis-aligned rectangle on a 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Position of the top-left corner.
    pub origin: Coordinates, // e.g., { x, y }

    /// Size of the rectangle.
    pub shape: Shape, // e.g., { width, height }
}

impl Rect {
    pub fn new(origin: Coordinates, shape: Shape) -> Self {
        Self { origin, shape }
    }

    /// Returns the top-left corner (same as origin).
    pub fn top_left(&self) -> Coordinates {
        self.origin
    }

    /// Returns the top-right corner.
    pub fn top_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 - 1,
            y: self.origin.y,
        }
    }

    /// Returns the bottom-left corner.
    pub fn bottom_left(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x,
            y: self.origin.y + self.shape.height as u32 - 1,
        }
    }

    /// Returns the bottom-right corner.
    pub fn bottom_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 - 1,
            y: self.origin.y + self.shape.height as u32 - 1,
        }
    }

    /// Returns the center of the rectangle, floored if even dimensions.
    pub fn center(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 / 2,
            y: self.origin.y + self.shape.height as u32 / 2,
        }
    }

    /// Test if a point lies within this rectangle.
    pub fn contains(&self, pt: Coordinates) -> bool {
        let x = pt.x;
        let y = pt.y;
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.shape.width as u32;
        let h = self.shape.height as u32;
        x >= ox && x < ox + w && y >= oy && y < oy + h
    }

    /// Iterate through all grid points inside the rectangle.
    pub fn iter(&self) -> impl Iterator<Item = Coordinates> {
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.shape.width;
        let h = self.shape.height;
        (0..h).flat_map(move |dy| {
            (0..w).map(move |dx| Coordinates {
                x: ox + dx as u32,
                y: oy + dy as u32,
            })
        })
    }

    pub fn offset(&mut self, delta: Delta) {
        let new_x = self.origin.x as i32 + delta.dx;
        let new_y = self.origin.y as i32 + delta.dy;

        self.origin.x = if new_x < 0 { 0 } else { new_x as u32 };
        self.origin.y = if new_y < 0 { 0 } else { new_y as u32 };
    }

    /// Parse a NxN shape area in its sub areas of 1x1
    pub fn as_many(&self) -> Vec<Self> {
        self.iter()
            .map(|coord| Rect {
                origin: coord,
                shape: Shape::from_square(1),
            })
            .collect()
    }

    /// Combine many 1x1 Rects into a single bounding Rect.
    /// Assumes the input Rects are all 1x1 in shape.
    pub fn from_many(rects: Vec<Self>) -> Self {
        assert!(!rects.is_empty(), "Cannot create Rect from empty list");

        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = u32::MIN;
        let mut max_y = u32::MIN;

        for rect in rects {
            let Coordinates { x, y } = rect.origin;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let origin = Coordinates { x: min_x, y: min_y };
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let shape = Shape { width, height };

        Rect { origin, shape }
    }
}
