use crate::prelude::{Coordinates, Delta, Shape};

/// Axis-aligned rectangle on a 2D grid.
///
/// Represented by an origin (top-left corner) and a shape (width, height).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Top-left corner of the rectangle.
    pub origin: Coordinates,

    /// Dimensions of the rectangle.
    pub shape: Shape,
}

impl Rect {
    /// Creates a new rectangle from the given origin and shape.
    pub fn new(origin: Coordinates, shape: Shape) -> Self {
        Self { origin, shape }
    }

    /// Returns the top-left corner (i.e., the origin).
    pub fn top_left(&self) -> Coordinates {
        self.origin
    }

    /// Returns the top-right corner of the rectangle.
    pub fn top_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 - 1,
            y: self.origin.y,
        }
    }

    /// Returns the bottom-left corner of the rectangle.
    pub fn bottom_left(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x,
            y: self.origin.y + self.shape.height as u32 - 1,
        }
    }

    /// Returns the bottom-right corner of the rectangle.
    pub fn bottom_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 - 1,
            y: self.origin.y + self.shape.height as u32 - 1,
        }
    }

    /// Returns the center point of the rectangle, flooring the result
    /// when the dimensions are even.
    pub fn center(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width as u32 / 2,
            y: self.origin.y + self.shape.height as u32 / 2,
        }
    }

    /// Checks whether a point lies within the rectangle (inclusive of origin,
    /// exclusive of bottom-right).
    pub fn contains(&self, pt: Coordinates) -> bool {
        let x = pt.x;
        let y = pt.y;
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.shape.width as u32;
        let h = self.shape.height as u32;
        x >= ox && x < ox + w && y >= oy && y < oy + h
    }

    /// Returns an iterator over all grid points inside the rectangle,
    /// from top-left to bottom-right, row-major order.
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

    /// Offsets the rectangle by a delta in-place.
    /// Clamps resulting coordinates to a minimum of (0, 0).
    pub fn offset(&mut self, delta: Delta) {
        let new_x = self.origin.x as i32 + delta.dx;
        let new_y = self.origin.y as i32 + delta.dy;

        self.origin.x = if new_x < 0 { 0 } else { new_x as u32 };
        self.origin.y = if new_y < 0 { 0 } else { new_y as u32 };
    }

    /// Splits the current rectangle into many 1x1 rectangles,
    /// each corresponding to a single tile-sized sub-area.
    pub fn as_many(&self) -> Vec<Self> {
        self.iter()
            .map(|coord| Rect {
                origin: coord,
                shape: Shape::from_square(1),
            })
            .collect()
    }

    /// Combines a list of 1x1 rectangles into the minimal bounding rectangle
    /// that contains all of them.
    ///
    /// # Panics
    /// Panics if the list is empty.
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

#[cfg(test)]
mod tests {
    use super::*;

    pub fn rect_6x6() -> Rect {
        Rect::new(
            Coordinates { x: 0, y: 0 },
            Shape {
                width: 6,
                height: 6,
            },
        )
    }

    #[test]
    pub fn is_bound_exclusive() {
        let rect = Rect::new(
            Coordinates { x: 2, y: 3 },
            Shape {
                width: 4,
                height: 5,
            },
        );

        // Points inside the bounds
        assert!(
            rect.contains(Coordinates { x: 2, y: 3 }),
            "origin should be included"
        );
        assert!(
            rect.contains(Coordinates { x: 5, y: 7 }),
            "bottom-right inside bounds should be included"
        );

        // Points on the exclusive boundary (just outside)
        assert!(
            !rect.contains(Coordinates { x: 6, y: 3 }),
            "right edge should be excluded"
        );
        assert!(
            !rect.contains(Coordinates { x: 2, y: 8 }),
            "bottom edge should be excluded"
        );
        assert!(
            !rect.contains(Coordinates { x: 6, y: 8 }),
            "bottom-right corner should be excluded"
        );

        // Point before the origin
        assert!(
            !rect.contains(Coordinates { x: 1, y: 3 }),
            "left of origin should be excluded"
        );
        assert!(
            !rect.contains(Coordinates { x: 2, y: 2 }),
            "above origin should be excluded"
        );
    }

    #[test]
    pub fn splits_into_many() {
        let rect = rect_6x6();
        let many = rect.as_many();
        assert_eq!(many.len(), 36); // 6x6 grid

        // Collect expected coordinates from the original Rect
        let expected_coords: Vec<_> = rect.iter().collect();

        // Check that all origins from `as_many()` match the expected coordinates
        for (i, tile_rect) in many.iter().enumerate() {
            assert_eq!(
                tile_rect.origin, expected_coords[i],
                "Mismatch at index {}: expected {:?}, got {:?}",
                i, expected_coords[i], tile_rect.origin
            );
            assert_eq!(
                tile_rect.shape,
                Shape::from_square(1),
                "Non-1x1 shape at index {}: got {:?}",
                i,
                tile_rect.shape
            );
        }
    }

    #[test]
    pub fn joins_into() {
        // Manually create 1x1 rects covering a 3x2 area starting at (4, 5)
        let rects = vec![
            Rect::new(Coordinates { x: 4, y: 5 }, Shape::from_square(1)),
            Rect::new(Coordinates { x: 5, y: 5 }, Shape::from_square(1)),
            Rect::new(Coordinates { x: 6, y: 5 }, Shape::from_square(1)),
            Rect::new(Coordinates { x: 4, y: 6 }, Shape::from_square(1)),
            Rect::new(Coordinates { x: 5, y: 6 }, Shape::from_square(1)),
            Rect::new(Coordinates { x: 6, y: 6 }, Shape::from_square(1)),
        ];

        let joined = Rect::from_many(rects);

        let expected = Rect::new(
            Coordinates { x: 4, y: 5 },
            Shape {
                width: 3,
                height: 2,
            },
        );

        assert_eq!(
            joined, expected,
            "Joined rect does not match expected bounds"
        );
    }

    #[test]
    pub fn splits_and_rejoins() {
        let rect = rect_6x6();
        let many = rect.as_many();
        let new_rect = Rect::from_many(many);
        assert_eq!(rect, new_rect)
    }

    #[test]
    pub fn offsets() {
        let mut rect = Rect::new(
            Coordinates { x: 10, y: 10 },
            Shape {
                width: 4,
                height: 4,
            },
        );

        // Positive offset
        rect.offset(Delta { dx: 5, dy: 3 });
        assert_eq!(
            rect.origin,
            Coordinates { x: 15, y: 13 },
            "Positive offset failed"
        );

        // Negative offset (still in bounds)
        rect.offset(Delta { dx: -10, dy: -10 });
        assert_eq!(
            rect.origin,
            Coordinates { x: 5, y: 3 },
            "Negative offset failed"
        );

        // Negative offset that would go out of bounds (should clamp to 0)
        rect.offset(Delta { dx: -10, dy: -10 });
        assert_eq!(
            rect.origin,
            Coordinates { x: 0, y: 0 },
            "Offset should clamp to (0, 0)"
        );
    }

    #[test]
    pub fn computes_center() {
        let rect = Rect::new(
            Coordinates { x: 2, y: 4 },
            Shape {
                width: 5,
                height: 3,
            },
        );

        // Center should be (2 + 5 / 2, 4 + 3 / 2) = (4, 5)
        assert_eq!(
            rect.center(),
            Coordinates { x: 4, y: 5 },
            "Incorrect center for odd width/height"
        );

        let even_rect = Rect::new(
            Coordinates { x: 0, y: 0 },
            Shape {
                width: 4,
                height: 4,
            },
        );

        // Center should be (0 + 2, 0 + 2) = (2, 2)
        assert_eq!(
            even_rect.center(),
            Coordinates { x: 2, y: 2 },
            "Incorrect center for even width/height"
        );
    }

    #[test]
    pub fn computes_bounds() {
        let rect = Rect::new(
            Coordinates { x: 3, y: 7 },
            Shape {
                width: 5,
                height: 4,
            },
        );

        assert_eq!(
            rect.top_left(),
            Coordinates { x: 3, y: 7 },
            "Incorrect top-left"
        );
        assert_eq!(
            rect.top_right(),
            Coordinates { x: 7, y: 7 },
            "Incorrect top-right"
        );
        assert_eq!(
            rect.bottom_left(),
            Coordinates { x: 3, y: 10 },
            "Incorrect bottom-left"
        );
        assert_eq!(
            rect.bottom_right(),
            Coordinates { x: 7, y: 10 },
            "Incorrect bottom-right"
        );
    }

    #[test]
    pub fn iterates_all_points() {
        let rect = Rect::new(
            Coordinates { x: 1, y: 2 },
            Shape {
                width: 3,
                height: 2,
            },
        );

        let expected = vec![
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 2, y: 2 },
            Coordinates { x: 3, y: 2 },
            Coordinates { x: 1, y: 3 },
            Coordinates { x: 2, y: 3 },
            Coordinates { x: 3, y: 3 },
        ];

        let actual: Vec<_> = rect.iter().collect();
        assert_eq!(actual, expected, "iter() yielded unexpected coordinates");
    }

    #[test]
    #[should_panic(expected = "Cannot create Rect from empty list")]
    pub fn join_fails_on_empty() {
        let _ = Rect::from_many(vec![]);
    }

    #[test]
    pub fn contains_edges_correctly() {
        let rect = Rect::new(
            Coordinates { x: 0, y: 0 },
            Shape {
                width: 2,
                height: 2,
            },
        );

        assert!(rect.contains(Coordinates { x: 0, y: 0 }));
        assert!(rect.contains(Coordinates { x: 1, y: 1 }));
        assert!(!rect.contains(Coordinates { x: 2, y: 1 }));
        assert!(!rect.contains(Coordinates { x: 1, y: 2 }));
    }

    #[test]
    pub fn split_preserves_area() {
        let rect = Rect::new(
            Coordinates { x: 10, y: 10 },
            Shape {
                width: 3,
                height: 2,
            },
        );
        let many = rect.as_many();
        let total_area = many
            .iter()
            .map(|r| r.shape.width * r.shape.height)
            .sum::<u32>();

        assert_eq!(
            total_area,
            rect.shape.width * rect.shape.height,
            "Total area from as_many() does not match original"
        );
    }

    #[test]
    pub fn offset_does_not_underflow() {
        let mut rect = Rect::new(Coordinates { x: 1, y: 1 }, Shape::from_square(1));
        rect.offset(Delta { dx: -5, dy: -5 });
        assert_eq!(
            rect.origin,
            Coordinates { x: 0, y: 0 },
            "Offset should clamp to 0,0"
        );
    }

    #[test]
    pub fn identity_behavior() {
        let rect = Rect::new(Coordinates { x: 5, y: 5 }, Shape::from_square(2));
        let mut offset_rect = rect;
        offset_rect.offset(Delta { dx: 0, dy: 0 });
        assert_eq!(offset_rect, rect, "Offset by zero should not change rect");

        let rejoined = Rect::from_many(rect.as_many());
        assert_eq!(
            rejoined, rect,
            "Split and rejoin should yield identical rect"
        );
    }
}
