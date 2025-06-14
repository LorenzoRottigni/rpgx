use std::{
    fmt,
    ops::{Add, Sub},
};

use crate::prelude::{Coordinates, Delta, Shape};

/// Errors related to [`Rect`] construction and manipulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RectError {
    /// Returned when trying to construct a [`Rect`] from an empty list of rectangles.
    EmptyRectList,
}

/// A rectangular region on a 2D grid, aligned to the grid axes.
///
/// Represented by a top-left origin [`Coordinates`] and a [`Shape`] defining its width and height.
/// All dimensions and coordinates are unsigned and non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    /// Top-left corner of the rectangle.
    pub origin: Coordinates,

    /// Dimensions of the rectangle (width × height).
    pub shape: Shape,
}

impl Rect {
    /// Creates a new `Rect` with the given origin and shape.
    ///
    /// # Examples
    /// ```
    /// let rect = Rect::new(Coordinates { x: 0, y: 0 }, Shape::from_square(3));
    /// ```
    pub fn new(origin: Coordinates, shape: Shape) -> Self {
        Self { origin, shape }
    }

    /// Creates a `Rect` with the given shape at origin (0, 0).
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            origin: Coordinates { x: 0, y: 0 },
            shape,
        }
    }

    /// Creates a 1×1 `Rect` at the given origin.
    pub fn from_origin(origin: Coordinates) -> Self {
        Self {
            origin,
            shape: Shape::from_square(1),
        }
    }

    /// Attempts to create a single `Rect` that bounds a set of 1×1 rectangles.
    ///
    /// Returns an error if the input list is empty.
    ///
    /// # Errors
    /// Returns [`RectError::EmptyRectList`] if `rects` is empty.
    pub fn from_many(rects: Vec<Self>) -> Result<Self, RectError> {
        if rects.is_empty() {
            return Err(RectError::EmptyRectList);
        }

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
        let shape = Shape {
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        };

        Ok(Rect { origin, shape })
    }
}

impl Rect {
    /// Splits this `Rect` into multiple 1×1 `Rect`s, one per tile.
    ///
    /// This is useful for applying per-tile logic or reconstruction.
    ///
    /// # Examples
    /// ```
    /// let rect = Rect::from_shape(Shape { width: 2, height: 2 });
    /// let many = rect.as_many();
    /// assert_eq!(many.len(), 4);
    /// ```
    pub fn as_many(&self) -> Vec<Self> {
        self.iter()
            .map(|coord| Rect {
                origin: coord,
                shape: Shape::from_square(1),
            })
            .collect()
    }

    /// Returns true if this rectangle intersects with another.
    pub fn intersects(&self, other: &Rect) -> bool {
        let self_right = self.origin.x + self.shape.width;
        let self_bottom = self.origin.y + self.shape.height;
        let other_right = other.origin.x + other.shape.width;
        let other_bottom = other.origin.y + other.shape.height;

        !(self_right <= other.origin.x
            || other_right <= self.origin.x
            || self_bottom <= other.origin.y
            || other_bottom <= self.origin.y)
    }

    /// Returns the overlapping region between two rectangles, or `None` if they don’t intersect.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.origin.x.max(other.origin.x);
        let y1 = self.origin.y.max(other.origin.y);
        let x2 = (self.origin.x + self.shape.width).min(other.origin.x + other.shape.width);
        let y2 = (self.origin.y + self.shape.height).min(other.origin.y + other.shape.height);

        if x2 > x1 && y2 > y1 {
            Some(Rect {
                origin: Coordinates { x: x1, y: y1 },
                shape: Shape {
                    width: x2 - x1,
                    height: y2 - y1,
                },
            })
        } else {
            None
        }
    }

    /// Returns a new `Rect` expanded by a uniform margin in all directions.
    ///
    /// Expansion is clamped to origin (0, 0) to avoid underflow.
    pub fn expand(&self, margin: u32) -> Self {
        let ox = self.origin.x.saturating_sub(margin);
        let oy = self.origin.y.saturating_sub(margin);
        let w = self.shape.width + margin * 2;
        let h = self.shape.height + margin * 2;

        Rect {
            origin: Coordinates { x: ox, y: oy },
            shape: Shape {
                width: w,
                height: h,
            },
        }
    }
}

impl Rect {
    /// Returns the top-left corner of the rectangle (equal to `origin`).
    pub fn top_left(&self) -> Coordinates {
        self.origin
    }

    /// Returns the top-right corner of the rectangle.
    pub fn top_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width.saturating_sub(1),
            y: self.origin.y,
        }
    }

    /// Returns the bottom-left corner of the rectangle.
    pub fn bottom_left(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x,
            y: self.origin.y + self.shape.height.saturating_sub(1),
        }
    }

    /// Returns the bottom-right corner of the rectangle.
    pub fn bottom_right(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width.saturating_sub(1),
            y: self.origin.y + self.shape.height.saturating_sub(1),
        }
    }

    /// Returns the center point of the rectangle using integer division.
    ///
    /// For even dimensions, this returns the upper-left center tile.
    pub fn center(&self) -> Coordinates {
        Coordinates {
            x: self.origin.x + self.shape.width / 2,
            y: self.origin.y + self.shape.height / 2,
        }
    }

    /// Returns `true` if the given coordinates fall within the bounds of the rectangle.
    ///
    /// Bounds are inclusive at the top-left and exclusive at the bottom-right.
    pub fn contains(&self, pt: Coordinates) -> bool {
        let x = pt.x;
        let y = pt.y;
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.shape.width;
        let h = self.shape.height;
        x >= ox && x < ox + w && y >= oy && y < oy + h
    }

    /// Returns an iterator over all coordinates contained in this rectangle.
    ///
    /// Iteration order is row-major (left to right, top to bottom).
    pub fn iter(&self) -> impl Iterator<Item = Coordinates> {
        let ox = self.origin.x;
        let oy = self.origin.y;
        let w = self.shape.width;
        let h = self.shape.height;
        (0..h).flat_map(move |dy| {
            (0..w).map(move |dx| Coordinates {
                x: ox + dx,
                y: oy + dy,
            })
        })
    }
}

impl Rect {
    /// Offsets the rectangle’s origin by the given delta.
    ///
    /// The result is clamped to non-negative values (0, 0 minimum).
    ///
    /// # Examples
    /// ```
    /// let mut rect = Rect::from_origin(Coordinates { x: 5, y: 5 });
    /// rect.offset(Delta { dx: -10, dy: -10 });
    /// assert_eq!(rect.origin, Coordinates { x: 0, y: 0 });
    /// ```
    pub fn offset(&mut self, delta: Delta) {
        let new_x = self.origin.x as i32 + delta.dx;
        let new_y = self.origin.y as i32 + delta.dy;

        self.origin.x = new_x.max(0) as u32;
        self.origin.y = new_y.max(0) as u32;
    }

    /// Returns a new `Rect` translated by the given `Delta`, clamped at zero.
    pub fn translate(&self, delta: Delta) -> Self {
        let mut rect = *self;
        rect.offset(delta);
        rect
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rect(origin: {}, {}, shape: {}×{})",
            self.origin.x, self.origin.y, self.shape.width, self.shape.height
        )
    }
}

impl Add<Delta> for Rect {
    type Output = Self;

    fn add(self, delta: Delta) -> Self {
        self.translate(delta)
    }
}

impl Sub<Delta> for Rect {
    type Output = Self;

    fn sub(self, delta: Delta) -> Self {
        self.translate(Delta {
            dx: -delta.dx,
            dy: -delta.dy,
        })
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

        assert!(rect.contains(Coordinates { x: 2, y: 3 }));
        assert!(rect.contains(Coordinates { x: 5, y: 7 }));
        assert!(!rect.contains(Coordinates { x: 6, y: 3 }));
        assert!(!rect.contains(Coordinates { x: 2, y: 8 }));
        assert!(!rect.contains(Coordinates { x: 6, y: 8 }));
        assert!(!rect.contains(Coordinates { x: 1, y: 3 }));
        assert!(!rect.contains(Coordinates { x: 2, y: 2 }));
    }

    #[test]
    pub fn splits_into_many() {
        let rect = rect_6x6();
        let many = rect.as_many();
        assert_eq!(many.len(), 36);

        let expected_coords: Vec<_> = rect.iter().collect();
        for (i, tile_rect) in many.iter().enumerate() {
            assert_eq!(tile_rect.origin, expected_coords[i]);
            assert_eq!(tile_rect.shape, Shape::from_square(1));
        }
    }

    #[test]
    pub fn joins_into() {
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

        assert_eq!(joined.unwrap(), expected);
    }

    #[test]
    pub fn splits_and_rejoins() {
        let rect = rect_6x6();
        let many = rect.as_many();
        let new_rect = Rect::from_many(many);
        assert_eq!(rect, new_rect.unwrap())
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

        rect.offset(Delta { dx: 5, dy: 3 });
        assert_eq!(rect.origin, Coordinates { x: 15, y: 13 });

        rect.offset(Delta { dx: -10, dy: -10 });
        assert_eq!(rect.origin, Coordinates { x: 5, y: 3 });

        rect.offset(Delta { dx: -10, dy: -10 });
        assert_eq!(rect.origin, Coordinates { x: 0, y: 0 });
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

        assert_eq!(rect.center(), Coordinates { x: 4, y: 5 });

        let even_rect = Rect::new(
            Coordinates { x: 0, y: 0 },
            Shape {
                width: 4,
                height: 4,
            },
        );

        assert_eq!(even_rect.center(), Coordinates { x: 2, y: 2 });
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

        assert_eq!(rect.top_left(), Coordinates { x: 3, y: 7 });
        assert_eq!(rect.top_right(), Coordinates { x: 7, y: 7 });
        assert_eq!(rect.bottom_left(), Coordinates { x: 3, y: 10 });
        assert_eq!(rect.bottom_right(), Coordinates { x: 7, y: 10 });
    }

    #[test]
    pub fn iterates_all_points() {
        let rect = Rect::new(
            Coordinates { x: 1, y: 1 },
            Shape {
                width: 2,
                height: 2,
            },
        );

        let expected = vec![
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 2, y: 1 },
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 2, y: 2 },
        ];

        let actual: Vec<_> = rect.iter().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn expands_rect() {
        let rect = Rect::new(
            Coordinates { x: 3, y: 3 },
            Shape {
                width: 2,
                height: 2,
            },
        );

        let expanded = rect.expand(2);
        assert_eq!(expanded.origin, Coordinates { x: 1, y: 1 });
        assert_eq!(
            expanded.shape,
            Shape {
                width: 6,
                height: 6
            }
        );
    }

    #[test]
    pub fn rect_intersects_and_intersection() {
        let a = Rect::new(
            Coordinates { x: 1, y: 1 },
            Shape {
                width: 4,
                height: 4,
            },
        );
        let b = Rect::new(
            Coordinates { x: 3, y: 3 },
            Shape {
                width: 4,
                height: 4,
            },
        );

        assert!(a.intersects(&b));

        let intersection = a.intersection(&b).unwrap();
        assert_eq!(
            intersection,
            Rect::new(
                Coordinates { x: 3, y: 3 },
                Shape {
                    width: 2,
                    height: 2
                }
            )
        );

        // Non-overlapping case
        let c = Rect::new(
            Coordinates { x: 10, y: 10 },
            Shape {
                width: 2,
                height: 2,
            },
        );

        assert!(!a.intersects(&c));
        assert!(a.intersection(&c).is_none());
    }

    #[test]
    pub fn translate_produces_offset_rect() {
        let base = Rect::new(
            Coordinates { x: 5, y: 5 },
            Shape {
                width: 3,
                height: 3,
            },
        );

        let delta = Delta { dx: 2, dy: -1 };
        let translated = base.translate(delta);

        assert_eq!(
            translated.origin,
            Coordinates { x: 7, y: 4 },
            "translate should move origin correctly"
        );

        assert_eq!(translated.shape, base.shape, "shape should not change");
    }
}
