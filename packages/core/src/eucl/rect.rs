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

    /// Creates a new `Rect` from origin `(x, y)` and dimensions `(width, height)`.
    pub fn from_xywh(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            origin: Coordinates { x, y },
            shape: Shape { width, height },
        }
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

    /// Returns the perimeter tiles of the rect as 1x1 Rects offset inward by `offset`.
    ///
    /// Tiles are returned clockwise starting from top-left corner.
    pub fn as_perimeter(&self, offset: u32) -> Vec<Self> {
        let mut perimeter = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            // No perimeter exists for this offset
            return perimeter;
        }

        let left = self.origin.x + offset;
        let right = self.origin.x + self.shape.width - 1 - offset;
        let top = self.origin.y + offset;
        let bottom = self.origin.y + self.shape.height - 1 - offset;

        // Top row (left to right)
        for x in left..=right {
            perimeter.push(Rect::from_xywh(x, top, 1, 1));
        }
        // Right column (top+1 to bottom-1)
        for y in (top + 1)..bottom {
            perimeter.push(Rect::from_xywh(right, y, 1, 1));
        }
        // Bottom row (right to left)
        if bottom > top {
            for x in (left..=right).rev() {
                perimeter.push(Rect::from_xywh(x, bottom, 1, 1));
            }
        }
        // Left column (bottom-1 down to top+1)
        if right > left {
            for y in ((top + 1)..bottom).rev() {
                perimeter.push(Rect::from_xywh(left, y, 1, 1));
            }
        }

        perimeter
    }

    /// Returns a vertical or horizontal bisector line of 1x1 Rects offset inward by `offset`.
    ///
    /// If the width is >= height, returns a vertical bisector line; otherwise, horizontal.
    pub fn as_bisector(&self, offset: u32) -> Vec<Self> {
        let mut bisector = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            return bisector;
        }

        let left = self.origin.x + offset;
        let top = self.origin.y + offset;
        let width = self.shape.width - 2 * offset;
        let height = self.shape.height - 2 * offset;

        if width >= height {
            // vertical bisector: middle column
            let mid_x = left + width / 2;
            for y in top..(top + height) {
                bisector.push(Rect::from_xywh(mid_x, y, 1, 1));
            }
        } else {
            // horizontal bisector: middle row
            let mid_y = top + height / 2;
            for x in left..(left + width) {
                bisector.push(Rect::from_xywh(x, mid_y, 1, 1));
            }
        }

        bisector
    }

    /// Returns the center tile(s) of the Rect offset inward by `offset`.
    ///
    /// For odd dimensions returns one tile; for even dimensions returns a 2x2 square.
    pub fn as_center(&self, offset: u32) -> Vec<Self> {
        let mut center = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            return center;
        }

        let left = self.origin.x + offset;
        let top = self.origin.y + offset;
        let width = self.shape.width - 2 * offset;
        let height = self.shape.height - 2 * offset;

        // Determine center coordinates
        let center_x = left + width / 2;
        let center_y = top + height / 2;

        if width % 2 == 1 && height % 2 == 1 {
            // Odd dimensions: single center tile
            center.push(Rect::from_xywh(center_x, center_y, 1, 1));
        } else {
            // Even dimension(s): 2x2 center block
            let cx_start = if width % 2 == 0 {
                center_x - 1
            } else {
                center_x
            };
            let cy_start = if height % 2 == 0 {
                center_y - 1
            } else {
                center_y
            };

            for x in cx_start..=cx_start + 1 {
                for y in cy_start..=cy_start + 1 {
                    center.push(Rect::from_xywh(x, y, 1, 1));
                }
            }
        }

        center
    }

    /// Returns tiles approximating a round (diamond-shaped) area around the center,
    /// including all tiles within `dial` distance (Manhattan distance) from the center tile(s).
    ///
    /// The circle is clamped to the rectangle bounds.
    pub fn as_round(&self, dial: u32) -> Vec<Self> {
        let mut tiles = Vec::new();

        if self.shape.width == 0 || self.shape.height == 0 {
            return tiles;
        }

        let left = self.origin.x;
        let top = self.origin.y;
        let width = self.shape.width;
        let height = self.shape.height;

        // Find center coordinates (can be 1 or 4 tiles for even dimensions)
        let center_x = left + width / 2;
        let center_y = top + height / 2;

        // For even width or height, center is between tiles, so consider all 1x1 tiles near center:
        // We'll just consider center points as in as_center:
        let centers = if width % 2 == 1 && height % 2 == 1 {
            vec![(center_x, center_y)]
        } else {
            let cx_start = if width % 2 == 0 {
                center_x - 1
            } else {
                center_x
            };
            let cy_start = if height % 2 == 0 {
                center_y - 1
            } else {
                center_y
            };

            vec![
                (cx_start, cy_start),
                (cx_start + 1, cy_start),
                (cx_start, cy_start + 1),
                (cx_start + 1, cy_start + 1),
            ]
        };

        // Collect tiles within dial (Manhattan distance) from any center tile, clamped to rect bounds
        for x in left..left + width {
            for y in top..top + height {
                if centers.iter().any(|&(cx, cy)| {
                    let dist = (cx as i32 - x as i32).abs() + (cy as i32 - y as i32).abs();
                    dist as u32 <= dial
                }) {
                    tiles.push(Rect::from_xywh(x, y, 1, 1));
                }
            }
        }

        tiles
    }

    /// Returns all 1×1 tiles within the `Rect` that are located on odd (x + y) sum positions.
    ///
    /// Useful for checkerboard or diagonal pattern logic.
    pub fn as_odds(&self) -> Vec<Self> {
        self.iter()
            .filter(|coord| (coord.x + coord.y) % 2 == 1)
            .map(|coord| Rect::from_xywh(coord.x, coord.y, 1, 1))
            .collect()
    }

    /// Returns all 1×1 tiles within the `Rect` that are located on even (x + y) sum positions.
    ///
    /// Useful for checkerboard or diagonal pattern logic.
    pub fn as_evens(&self) -> Vec<Self> {
        self.iter()
            .filter(|coord| (coord.x + coord.y) % 2 == 0)
            .map(|coord| Rect::from_xywh(coord.x, coord.y, 1, 1))
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn as_round_includes_center_and_neighbors_for_small_odd_rect() {
            let rect = Rect::from_shape(Shape {
                width: 3,
                height: 3,
            });
            let round_tiles = rect.as_round(1);

            let expected_coords = vec![
                (1, 1), // center
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1), // neighbors at distance 1
            ];

            let round_coords: Vec<_> = round_tiles
                .iter()
                .map(|r| (r.origin.x, r.origin.y))
                .collect();

            for coord in expected_coords {
                assert!(
                    round_coords.contains(&coord),
                    "Expected coordinate {:?} in round tiles",
                    coord
                );
            }

            assert!(
                round_coords.iter().all(|&(x, y)| x < 3 && y < 3),
                "All tiles should be within rectangle bounds"
            );
        }

        #[test]
        fn as_round_correctly_handles_even_sized_rectangle() {
            let rect = Rect::from_shape(Shape {
                width: 4,
                height: 4,
            });
            let round_tiles = rect.as_round(1);

            let centers = [(1, 1), (1, 2), (2, 1), (2, 2)];

            for &(cx, cy) in &centers {
                assert!(
                    round_tiles
                        .iter()
                        .any(|r| r.origin.x == cx && r.origin.y == cy),
                    "Expected center tile at ({}, {})",
                    cx,
                    cy
                );
            }

            assert!(
                round_tiles.iter().all(|r| r.origin.x < 4 && r.origin.y < 4),
                "All tiles should be within rectangle bounds"
            );
        }

        #[test]
        fn as_round_with_zero_dial_returns_center_tiles_only() {
            let rect = Rect::from_shape(Shape {
                width: 5,
                height: 5,
            });
            let round_tiles = rect.as_round(0);

            let expected_count = if rect.shape.width % 2 == 1 && rect.shape.height % 2 == 1 {
                1
            } else {
                4
            };

            assert_eq!(
                round_tiles.len(),
                expected_count,
                "Expected only center tiles with zero dial"
            );
        }

        #[test]
        fn as_round_with_large_dial_returns_all_tiles() {
            let rect = Rect::from_shape(Shape {
                width: 3,
                height: 3,
            });
            let round_tiles = rect.as_round(10);

            assert_eq!(
                round_tiles.len(),
                9,
                "With large dial, all tiles in the rectangle should be included"
            );
        }

        #[test]
        fn as_perimeter_returns_correct_number_of_tiles() {
            let rect = Rect::from_shape(Shape {
                width: 4,
                height: 4,
            });
            let perimeter = rect.as_perimeter(0);

            assert_eq!(
                perimeter.len(),
                12,
                "Perimeter length should be number of tiles around rectangle"
            );

            assert_eq!(
                perimeter.first().unwrap().origin,
                Coordinates { x: 0, y: 0 },
                "First perimeter tile should be top-left corner"
            );

            let top_row_xs: Vec<_> = perimeter.iter().take(4).map(|r| r.origin.x).collect();
            assert_eq!(
                top_row_xs,
                vec![0, 1, 2, 3],
                "Top row perimeter tiles should be in ascending x order"
            );
        }

        #[test]
        fn as_perimeter_with_offset_returns_shrunk_perimeter() {
            let rect = Rect::from_shape(Shape {
                width: 6,
                height: 6,
            });
            let perimeter = rect.as_perimeter(1);

            assert_eq!(
                perimeter.len(),
                12,
                "Perimeter length with offset 1 matches shrunk rectangle perimeter"
            );
        }

        #[test]
        fn as_bisector_vertical_for_wider_rectangle_returns_column() {
            let rect = Rect::from_shape(Shape {
                width: 6,
                height: 4,
            });
            let bisector = rect.as_bisector(0);

            assert_eq!(
                bisector.len(),
                4,
                "Bisector length equals height for vertical bisector"
            );

            let expected_x = rect.origin.x + rect.shape.width / 2;
            for tile in bisector.iter() {
                assert_eq!(
                    tile.origin.x, expected_x,
                    "Bisector tiles should be aligned on bisector column"
                );
            }
        }

        #[test]
        fn as_bisector_horizontal_for_taller_rectangle_returns_row() {
            let rect = Rect::from_shape(Shape {
                width: 4,
                height: 6,
            });
            let bisector = rect.as_bisector(0);

            assert_eq!(
                bisector.len(),
                4,
                "Bisector length equals width for horizontal bisector"
            );

            let expected_y = rect.origin.y + rect.shape.height / 2;
            for tile in bisector.iter() {
                assert_eq!(
                    tile.origin.y, expected_y,
                    "Bisector tiles should be aligned on bisector row"
                );
            }
        }

        #[test]
        fn as_center_returns_single_tile_for_odd_dimensions() {
            let rect = Rect::from_shape(Shape {
                width: 5,
                height: 5,
            });
            let center = rect.as_center(0);

            assert_eq!(
                center.len(),
                1,
                "Center for odd-sized rectangle is a single tile"
            );
            assert_eq!(
                center[0].origin,
                Coordinates { x: 2, y: 2 },
                "Center tile is at center coordinates"
            );
        }

        #[test]
        fn as_center_returns_four_tiles_for_even_dimensions() {
            let rect = Rect::from_shape(Shape {
                width: 4,
                height: 4,
            });
            let center = rect.as_center(0);

            assert_eq!(
                center.len(),
                4,
                "Center for even-sized rectangle is four tiles"
            );

            let centers: Vec<_> = center.iter().map(|r| r.origin).collect();
            let expected_centers = vec![
                Coordinates { x: 1, y: 1 },
                Coordinates { x: 2, y: 1 },
                Coordinates { x: 1, y: 2 },
                Coordinates { x: 2, y: 2 },
            ];

            for c in expected_centers {
                assert!(centers.contains(&c), "Expected center tile at {:?}", c);
            }
        }
    }
}
