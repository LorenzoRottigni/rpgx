use crate::prelude::{Coordinates, Shape};

/// Axis-aligned rectangle on a 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Position of the top-left corner.
    pub origin: Coordinates, // e.g., { x, y }

    /// Size of the rectangle.
    pub shape: Shape, // e.g., { width, height }
}

impl Rect {
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
}
