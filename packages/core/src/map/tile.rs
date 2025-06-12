use crate::{
    common::delta::Delta,
    prelude::{Coordinates, Effect, Shape, SingleSelector},
};

#[doc = include_str!("../../docs/tile.md")]
/// Represents a single tile on the grid with unique identifier, spatial information, and effects applied.
///
/// See also:
/// - [`Effect`](crate::prelude::Effect)
/// - [`Coordinates`](crate::prelude::Coordinates)
/// - [`Shape`](crate::prelude::Shape)
/// - [`SingleSelector`](crate::prelude::SingleSelector)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    /// Effect applied to this tile (e.g., blocking, status changes).
    pub effect: Effect,

    /// Top-left coordinate where the tile begins.
    pub pointer: SingleSelector,

    /// Shape of the tile (its size on the grid).
    pub shape: Shape,
}

impl Tile {
    /// Creates a new tile from the given ID, effect, pointer, and shape.
    pub fn new(effect: Effect, pointer: SingleSelector, shape: Shape) -> Self {
        Self {
            effect,
            pointer,
            shape,
        }
    }

    /// Returns true if the given point lies within the tile's shape.
    ///
    /// Uses [`Shape`] and [`pointer`](Self::pointer) to compute bounds.
    pub fn contains(&self, point: Coordinates) -> bool {
        if point.x < self.pointer.x || point.y < self.pointer.y {
            return false;
        }
        let relative_point = Delta {
            dx: point.x as i32 - self.pointer.x as i32,
            dy: point.y as i32 - self.pointer.y as i32,
        };
        self.shape.delta_in_bounds(relative_point)
    }

    /// Returns true if the tile blocks at a specific coordinate.
    ///
    /// Blocking is defined by [`Effect::block`] and optional [`Effect::shrink`] region.
    pub fn is_blocking_at(&self, target: Coordinates) -> bool {
        if !self.effect.block {
            return false;
        }

        self.effect.shrink_contains(target) && self.contains(target)
    }

    /// Offsets the tile and any effect shrink bounds by the given delta.
    ///
    /// # Parameters
    ///
    /// * `delta` - The coordinate offset to apply to the tile's pointer and effect bounds.
    pub fn offset(&mut self, delta: Coordinates) {
        self.pointer += delta;
        if let Some((start, end)) = self.effect.shrink {
            self.effect.shrink = Some((start + delta, end + delta));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Effect, Shape, SingleSelector};

    fn basic_effect(block: bool, shrink: Option<(Coordinates, Coordinates)>) -> Effect {
        Effect {
            block,
            shrink,
            ..Default::default()
        }
    }

    #[test]
    fn test_contains_inside_and_outside() {
        let tile = Tile::new(
            basic_effect(false, None),
            SingleSelector { x: 10, y: 20 },
            Shape::from_rectangle(3, 3),
        );

        // Inside tile bounds
        assert!(tile.contains(Coordinates { x: 10, y: 20 }));
        assert!(tile.contains(Coordinates { x: 12, y: 22 }));

        // Outside tile bounds
        assert!(!tile.contains(Coordinates { x: 9, y: 20 }));
        assert!(!tile.contains(Coordinates { x: 13, y: 23 }));
    }

    #[test]
    fn test_is_blocking_at_without_block() {
        let tile = Tile::new(
            basic_effect(false, None),
            SingleSelector { x: 0, y: 0 },
            Shape::from_square(2),
        );
        assert!(!tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn test_is_blocking_at_with_block_no_shrink() {
        let tile = Tile::new(
            basic_effect(true, None),
            SingleSelector { x: 5, y: 5 },
            Shape::from_square(2),
        );

        // Inside tile and block enabled
        assert!(tile.is_blocking_at(Coordinates { x: 5, y: 5 }));
        assert!(tile.is_blocking_at(Coordinates { x: 6, y: 6 }));

        // Outside tile bounds
        assert!(!tile.is_blocking_at(Coordinates { x: 7, y: 7 }));
    }

    #[test]
    fn test_is_blocking_at_with_shrink() {
        // Effect shrink region from (6,6) to (7,7)
        let shrink_start = Coordinates { x: 6, y: 6 };
        let shrink_end = Coordinates { x: 7, y: 7 };

        let tile = Tile::new(
            basic_effect(true, Some((shrink_start, shrink_end))),
            SingleSelector { x: 5, y: 5 },
            Shape::from_square(3),
        );

        // Inside shrink region and tile bounds
        assert!(tile.is_blocking_at(Coordinates { x: 6, y: 6 }));
        assert!(tile.is_blocking_at(Coordinates { x: 7, y: 7 }));

        // Inside tile but outside shrink region
        assert!(!tile.is_blocking_at(Coordinates { x: 5, y: 5 }));

        // Outside tile
        assert!(!tile.is_blocking_at(Coordinates { x: 8, y: 8 }));
    }

    #[test]
    fn test_offset_moves_tile_and_shrink() {
        let shrink_start = Coordinates { x: 1, y: 1 };
        let shrink_end = Coordinates { x: 2, y: 2 };
        let mut tile = Tile::new(
            basic_effect(true, Some((shrink_start, shrink_end))),
            SingleSelector { x: 0, y: 0 },
            Shape::from_square(3),
        );

        tile.offset(Coordinates { x: 5, y: 5 });

        // Tile pointer updated
        assert_eq!(tile.pointer.x, 5);
        assert_eq!(tile.pointer.y, 5);

        // Shrink region updated by offset
        let (new_start, new_end) = tile.effect.shrink.expect("Shrink should be Some");
        assert_eq!(new_start, Coordinates { x: 6, y: 6 });
        assert_eq!(new_end, Coordinates { x: 7, y: 7 });
    }
}
