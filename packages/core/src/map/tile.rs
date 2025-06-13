use crate::{
    common::{delta::Delta, rect::Rect},
    prelude::{Coordinates, Effect},
};

#[doc = include_str!("../../docs/tile.md")]
/// Represents a single tile on the grid with unique identifier, spatial information, and effects applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    /// Effect applied to this tile (e.g., blocking, status changes).
    pub effect: Effect,

    /// Area the tile covers on the grid.
    pub area: Rect,
}

impl Tile {
    /// Creates a new tile from the given effect and area.
    pub fn new(effect: Effect, area: Rect) -> Self {
        Self { effect, area }
    }

    /// Returns true if the given point lies within the tile's area.
    pub fn contains(&self, point: Coordinates) -> bool {
        self.area.contains(point)
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
    pub fn offset(&mut self, delta: Delta) {
        self.area.origin = self.area.origin.offseted(delta);

        if let Some((start, end)) = self.effect.shrink {
            self.effect.shrink = Some((start.offseted(delta), end.offseted(delta)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Shape;

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
            Rect {
                origin: Coordinates { x: 10, y: 20 },
                shape: Shape::from_rectangle(3, 3),
            },
        );

        assert!(tile.contains(Coordinates { x: 10, y: 20 }));
        assert!(tile.contains(Coordinates { x: 12, y: 22 }));
        assert!(!tile.contains(Coordinates { x: 9, y: 20 }));
        assert!(!tile.contains(Coordinates { x: 13, y: 23 }));
    }

    #[test]
    fn test_is_blocking_at_without_block() {
        let tile = Tile::new(
            basic_effect(false, None),
            Rect {
                origin: Coordinates { x: 0, y: 0 },
                shape: Shape::from_square(2),
            },
        );
        assert!(!tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn test_is_blocking_at_with_block_no_shrink() {
        let tile = Tile::new(
            basic_effect(true, None),
            Rect {
                origin: Coordinates { x: 5, y: 5 },
                shape: Shape::from_square(2),
            },
        );

        assert!(tile.is_blocking_at(Coordinates { x: 5, y: 5 }));
        assert!(tile.is_blocking_at(Coordinates { x: 6, y: 6 }));
        assert!(!tile.is_blocking_at(Coordinates { x: 7, y: 7 }));
    }

    #[test]
    fn test_is_blocking_at_with_shrink() {
        let shrink_start = Coordinates { x: 6, y: 6 };
        let shrink_end = Coordinates { x: 7, y: 7 };

        let tile = Tile::new(
            basic_effect(true, Some((shrink_start, shrink_end))),
            Rect {
                origin: Coordinates { x: 5, y: 5 },
                shape: Shape::from_square(3),
            },
        );

        assert!(tile.is_blocking_at(Coordinates { x: 6, y: 6 }));
        assert!(tile.is_blocking_at(Coordinates { x: 7, y: 7 }));
        assert!(!tile.is_blocking_at(Coordinates { x: 5, y: 5 }));
        assert!(!tile.is_blocking_at(Coordinates { x: 8, y: 8 }));
    }

    #[test]
    fn test_offset_moves_tile_and_shrink() {
        let shrink_start = Coordinates { x: 1, y: 1 };
        let shrink_end = Coordinates { x: 2, y: 2 };
        let mut tile = Tile::new(
            basic_effect(true, Some((shrink_start, shrink_end))),
            Rect {
                origin: Coordinates { x: 0, y: 0 },
                shape: Shape::from_square(3),
            },
        );

        tile.offset(Delta { dx: 5, dy: 5 });

        assert_eq!(tile.area.origin.x, 5);
        assert_eq!(tile.area.origin.y, 5);

        let (new_start, new_end) = tile.effect.shrink.expect("Shrink should be Some");
        assert_eq!(new_start, Coordinates { x: 6, y: 6 });
        assert_eq!(new_end, Coordinates { x: 7, y: 7 });
    }
}
