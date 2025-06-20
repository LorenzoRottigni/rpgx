use crate::prelude::{Effect, Mask, Rect, Shape};

pub enum RenderTarget {
    Many,
    Single,
    Odds,
    Evens,
    Perimeter(u32, u32),
    Bisector(u32, u32),
    Center(u32, u32),
    Rhombus(u32),
    Circle,
}

pub trait Renderable {
    fn into_many(&self) -> Vec<Rect>;
    fn into_single(&self) -> Vec<Rect>;
    fn into_odds(&self) -> Vec<Rect>;
    fn into_evens(&self) -> Vec<Rect>;
    fn into_perimeter(&self, offset: u32, size: u32) -> Vec<Rect>;
    fn into_bisector(&self, offset: u32, size: u32) -> Vec<Rect>;
    fn into_center(&self, offset: u32, size: u32) -> Vec<Rect>;
    fn into_rhombus(&self, dial: u32) -> Vec<Rect>;
    fn into_circle(&self) -> Vec<Rect>;
}

pub struct Builder {}

impl Builder {
    pub fn render(rect: Rect, target: RenderTarget) -> Vec<Rect> {
        match target {
            RenderTarget::Many => rect.into_many(),
            RenderTarget::Single => rect.into_single(),
            RenderTarget::Odds => rect.into_odds(),
            RenderTarget::Evens => rect.into_evens(),
            RenderTarget::Perimeter(offset, size) => rect.into_perimeter(offset, size),
            RenderTarget::Bisector(offset, size) => rect.into_bisector(offset, size),
            RenderTarget::Center(offset, size) => rect.into_center(offset, size),
            RenderTarget::Rhombus(dial) => rect.into_rhombus(dial),
            RenderTarget::Circle => rect.into_circle(),
        }
    }
}

impl Rect {
    fn with_effect(&self, effect: Effect) -> Mask {
        Mask::new("default".to_string(), self.into_many(), effect)
    }
}

impl Renderable for Rect {
    /// Splits this `Rect` into multiple 1×1 `Rect`s, one per tile.
    ///
    /// This is useful for applying per-tile logic or reconstruction.
    ///
    fn into_many(&self) -> Vec<Self> {
        self.iter()
            .map(|coord| Rect {
                origin: coord,
                shape: Shape::from_square(1),
            })
            .collect()
    }

    fn into_single(&self) -> Vec<Self> {
        vec![*self]
    }

    /// Returns the perimeter tiles of the rect as 1×1 `Rect`s offset inward by `offset`, with `size` thickness.
    ///
    /// Tiles are returned clockwise starting from the top-left corner of the outermost perimeter band.
    ///
    /// # Arguments
    /// * `offset` - Distance from the edge before the perimeter starts.
    /// * `size` - Thickness of the perimeter band.
    ///
    /// # Panics
    /// Panics if `offset + size` exceeds half the width or height.
    ///
    fn into_perimeter(&self, offset: u32, size: u32) -> Vec<Self> {
        let mut perimeter = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            return perimeter;
        }

        let max_size = std::cmp::min(
            (self.shape.width / 2).saturating_sub(offset),
            (self.shape.height / 2).saturating_sub(offset),
        );

        let size = size.min(max_size);

        for s in 0..size {
            let left = self.origin.x + offset + s;
            let right = self.origin.x + self.shape.width - 1 - offset - s;
            let top = self.origin.y + offset + s;
            let bottom = self.origin.y + self.shape.height - 1 - offset - s;

            // Top edge
            for x in left..=right {
                perimeter.push(Rect::from_xywh(x, top, 1, 1));
            }
            // Right edge
            for y in (top + 1)..bottom {
                perimeter.push(Rect::from_xywh(right, y, 1, 1));
            }
            // Bottom edge
            if bottom > top {
                for x in (left..=right).rev() {
                    perimeter.push(Rect::from_xywh(x, bottom, 1, 1));
                }
            }
            // Left edge
            if right > left {
                for y in ((top + 1)..bottom).rev() {
                    perimeter.push(Rect::from_xywh(left, y, 1, 1));
                }
            }
        }

        perimeter
    }

    /// Returns a vertical or horizontal bisector band of 1×1 `Rect`s offset inward by `offset`, with `size` thickness.
    ///
    /// If the inner width is greater than or equal to height, returns a vertical band; otherwise, horizontal.
    ///
    /// # Arguments
    /// * `offset` - Distance from the edge before bisecting.
    /// * `size` - Thickness of the bisector (number of rows or columns).
    fn into_bisector(&self, offset: u32, size: u32) -> Vec<Self> {
        let mut bisector = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            return bisector;
        }

        let left = self.origin.x + offset;
        let top = self.origin.y + offset;
        let width = self.shape.width - 2 * offset;
        let height = self.shape.height - 2 * offset;

        if width >= height {
            // Vertical strip
            let center_x = left + width / 2;
            let half = (size / 2) as i32;

            for dx in -half..=(half + (size % 2 == 0) as i32 - 1) {
                let x = center_x as i32 + dx;
                if x >= left as i32 && x < (left + width) as i32 {
                    for y in top..top + height {
                        bisector.push(Rect::from_xywh(x as u32, y, 1, 1));
                    }
                }
            }
        } else {
            // Horizontal strip
            let center_y = top + height / 2;
            let half = (size / 2) as i32;

            for dy in -half..=(half + (size % 2 == 0) as i32 - 1) {
                let y = center_y as i32 + dy;
                if y >= top as i32 && y < (top + height) as i32 {
                    for x in left..left + width {
                        bisector.push(Rect::from_xywh(x, y as u32, 1, 1));
                    }
                }
            }
        }

        bisector
    }

    /// Returns a center block of `size × size` 1×1 `Rect`s offset inward by `offset`.
    ///
    /// # Arguments
    /// * `offset` - Distance from the edge before selecting center.
    /// * `size` - Width and height of the center selection.
    ///
    /// # Notes
    /// If the rect is too small to fit the center block after offset, returns empty.
    fn into_center(&self, offset: u32, size: u32) -> Vec<Self> {
        let mut center = Vec::new();

        if self.shape.width <= 2 * offset || self.shape.height <= 2 * offset {
            return center;
        }

        let inner_width = self.shape.width - 2 * offset;
        let inner_height = self.shape.height - 2 * offset;

        if inner_width < size || inner_height < size {
            return center;
        }

        let left = self.origin.x + offset + (inner_width - size) / 2;
        let top = self.origin.y + offset + (inner_height - size) / 2;

        for x in left..left + size {
            for y in top..top + size {
                center.push(Rect::from_xywh(x, y, 1, 1));
            }
        }

        center
    }

    /// Returns tiles approximating a rhombus (diamond-shaped) area around the center,
    /// including all tiles within `dial` distance (Manhattan distance) from the center tile(s).
    ///
    /// The circle is clamped to the rectangle bounds.
    fn into_rhombus(&self, dial: u32) -> Vec<Self> {
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
        // We'll just consider center points as in into_center:
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

    fn into_circle(&self) -> Vec<Rect> {
        let center = self.center();
        let radius_x = self.shape.width as f32 / 2.0;
        let radius_y = self.shape.height as f32 / 2.0;

        self.iter()
            .filter(|coord| {
                // Normalize coordinates relative to center
                let dx = (coord.x as f32 + 0.5) - (center.x as f32 + 0.5);
                let dy = (coord.y as f32 + 0.5) - (center.y as f32 + 0.5);

                // Use ellipse equation (dx/rx)^2 + (dy/ry)^2 <= 1
                (dx * dx) / (radius_x * radius_x) + (dy * dy) / (radius_y * radius_y) <= 1.0
            })
            .map(|coord| Rect::new(coord, Shape::from_square(1)))
            .collect()
    }

    /// Returns all 1×1 tiles within the `Rect` that are located on odd (x + y) sum positions.
    ///
    /// Useful for checkerboard or diagonal pattern logic.
    fn into_odds(&self) -> Vec<Rect> {
        self.iter()
            .filter(|coord| coord.x % 2 == 1 && coord.y % 2 == 1)
            .map(|coord| Rect::new(coord, Shape::from_square(1)))
            .collect()
    }

    /// Returns all 1×1 tiles within the `Rect` that are located on even (x + y) sum positions.
    ///
    /// Useful for checkerboard or diagonal pattern logic.
    fn into_evens(&self) -> Vec<Rect> {
        self.iter()
            .filter(|coord| coord.x % 2 == 0 && coord.y % 2 == 0)
            .map(|coord| Rect::new(coord, Shape::from_square(1)))
            .collect()
    }
}
