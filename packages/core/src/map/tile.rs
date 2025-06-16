use std::fmt;

use crate::prelude::{Coordinates, Delta, Effect, Rect};

#[doc = include_str!("../../docs/tile.md")]
/// Represents a single tile on the grid with an associated effect and
/// a rectangular area that it covers.
///
/// The tile’s area defines the bounds on the grid (inclusive at top-left,
/// exclusive at bottom-right).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    /// Effect applied to this tile (e.g., blocking, status changes).
    pub effect: Effect,

    /// Rectangular area covered by this tile on the grid.
    pub area: Rect,
}

impl Tile {
    /// Creates a new `Tile` with the given effect and area.
    ///
    /// # Parameters
    /// - `effect`: The effect to apply to this tile.
    /// - `area`: The rectangular grid area this tile occupies.
    pub fn new(effect: Effect, area: Rect) -> Self {
        Self { effect, area }
    }

    pub fn from_area(area: Rect) -> Self {
        Self {
            effect: Effect::default(),
            area,
        }
    }

    pub fn apply(&mut self, effect: Effect) {
        self.effect = effect
    }

    /// Returns `true` if the tile blocks movement or interaction at the
    /// specified coordinate.
    ///
    /// Blocking is defined by the presence of a blocking region inside the
    /// tile's effect (`effect.block`), and the coordinate must be inside
    /// both the tile's area and the blocking rectangle.
    ///
    /// # Parameters
    /// - `target`: The coordinate to check.
    pub fn is_blocking_at(&self, target: Coordinates) -> bool {
        if let Some(block_area) = self.effect.block {
            self.area.contains(target) && block_area.contains(target)
        } else {
            false
        }
    }

    /// Offsets the tile's area and any blocking region within the effect
    /// by the given delta.
    ///
    /// The offset clamps the origin of both rectangles to zero minimum.
    ///
    /// # Parameters
    /// - `delta`: The delta to offset by.
    pub fn offset(&mut self, delta: Delta) {
        self.area.offset(delta);

        if let Some(block_area) = &mut self.effect.block {
            block_area.offset(delta);
        }
    }

    /// Translates the tile by the given delta (alias for [`offset`]).
    ///
    /// # Parameters
    /// - `delta`: The delta to translate by.
    pub fn translate(&mut self, delta: Delta) {
        self.offset(delta)
    }

    /// Returns `true` if the tile's area contains the specified coordinate.
    ///
    /// # Parameters
    /// - `coord`: The coordinate to check.
    pub fn contains(&self, coord: Coordinates) -> bool {
        self.area.contains(coord)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tile(effect: {:?}, area: x={} y={} w={} h={})",
            self.effect,
            self.area.origin.x,
            self.area.origin.y,
            self.area.shape.width,
            self.area.shape.height
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape};

    fn effect_with_block(origin: Coordinates, shape: Shape) -> Effect {
        Effect {
            block: Some(Rect::new(origin, shape)),
            ..Default::default()
        }
    }

    #[test]
    fn contains_returns_true_inside_and_false_outside() {
        let tile = Tile::new(
            Effect::default(),
            Rect::new(
                Coordinates { x: 1, y: 1 },
                Shape {
                    width: 3,
                    height: 3,
                },
            ),
        );
        // inside bounds, inclusive top-left, exclusive bottom-right
        assert!(tile.contains(Coordinates { x: 1, y: 1 }));
        assert!(tile.contains(Coordinates { x: 3, y: 3 }));
        assert!(!tile.contains(Coordinates { x: 4, y: 4 }));
        assert!(!tile.contains(Coordinates { x: 0, y: 0 }));
    }

    #[test]
    fn is_blocking_at_respects_block_area_and_tile_area() {
        let effect = effect_with_block(
            Coordinates { x: 2, y: 2 },
            Shape {
                width: 2,
                height: 2,
            },
        );
        let tile = Tile::new(
            effect,
            Rect::new(
                Coordinates { x: 1, y: 1 },
                Shape {
                    width: 4,
                    height: 4,
                },
            ),
        );

        // Inside both tile area and block area
        assert!(tile.is_blocking_at(Coordinates { x: 2, y: 2 }));
        assert!(tile.is_blocking_at(Coordinates { x: 3, y: 3 }));

        // Inside tile area but outside block area
        assert!(!tile.is_blocking_at(Coordinates { x: 1, y: 1 }));

        // Outside tile area
        assert!(!tile.is_blocking_at(Coordinates { x: 5, y: 5 }));
    }

    #[test]
    fn offset_and_translate_shift_area_and_block() {
        let mut tile = Tile::new(
            effect_with_block(
                Coordinates { x: 0, y: 0 },
                Shape {
                    width: 2,
                    height: 2,
                },
            ),
            Rect::new(
                Coordinates { x: 1, y: 1 },
                Shape {
                    width: 3,
                    height: 3,
                },
            ),
        );

        tile.offset(Delta { dx: 2, dy: 3 });
        assert_eq!(tile.area.origin, Coordinates { x: 3, y: 4 });
        assert_eq!(
            tile.effect.block.unwrap().origin,
            Coordinates { x: 2, y: 3 }
        );

        tile.translate(Delta { dx: -1, dy: -1 });
        assert_eq!(tile.area.origin, Coordinates { x: 2, y: 3 });
        assert_eq!(
            tile.effect.block.unwrap().origin,
            Coordinates { x: 1, y: 2 }
        );

        // Offset clamps to zero
        tile.offset(Delta { dx: -10, dy: -10 });
        assert_eq!(tile.area.origin, Coordinates { x: 0, y: 0 });
        assert_eq!(
            tile.effect.block.unwrap().origin,
            Coordinates { x: 0, y: 0 }
        );
    }

    #[test]
    fn display_outputs_correct_format() {
        let effect = Effect::default();
        let tile = Tile::new(
            effect,
            Rect::new(
                Coordinates { x: 5, y: 6 },
                Shape {
                    width: 7,
                    height: 8,
                },
            ),
        );
        let display = format!("{}", tile);
        assert!(display.contains("effect:"));
        assert!(display.contains("area: x=5 y=6 w=7 h=8"));
    }
}
