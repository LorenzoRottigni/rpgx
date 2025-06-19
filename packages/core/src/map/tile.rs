use std::fmt;

use crate::prelude::{Coordinates, Effect, Rect};
use crate::traits::{Grid, Spatial};

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

impl Spatial for Tile {
    fn contains(&self, target: &Coordinates) -> bool {
        self.area.contains(target)
    }
}

impl Grid for Tile {
    fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.area.contains(target)
            && self
                .effect
                .block
                .map_or(false, |block| block.contains(target))
    }

    fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        if self.contains(target) {
            self.effect.action_id.into_iter().collect()
        } else {
            vec![]
        }
    }

    fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect> {
        if self.contains(target) {
            vec![self.effect]
        } else {
            vec![]
        }
    }
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
    use crate::prelude::{Coordinates, Effect, Rect, Shape};

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
        assert!(tile.contains(&Coordinates { x: 1, y: 1 }));
        assert!(tile.contains(&Coordinates { x: 3, y: 3 }));
        assert!(!tile.contains(&Coordinates { x: 4, y: 4 }));
        assert!(!tile.contains(&Coordinates { x: 0, y: 0 }));
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
        assert!(tile.is_blocking_at(&Coordinates { x: 2, y: 2 }));
        assert!(tile.is_blocking_at(&Coordinates { x: 3, y: 3 }));

        // Inside tile area but outside block area
        assert!(!tile.is_blocking_at(&Coordinates { x: 1, y: 1 }));

        // Outside tile area
        assert!(!tile.is_blocking_at(&Coordinates { x: 5, y: 5 }));
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
