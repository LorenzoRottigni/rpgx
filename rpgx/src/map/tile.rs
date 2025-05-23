use crate::prelude::{Coordinates, Effect, Shape, SingleSelector};

/// Represents a single tile on the grid with unique identifier, spatial information, and effects applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    pub id: i32,
    pub effect: Effect,
    pub pointer: SingleSelector,
    pub shape: Shape,
}

impl Tile {
    pub fn new(id: i32, effect: Effect, pointer: SingleSelector, shape: Shape) -> Self {
        Self {
            id,
            effect,
            pointer,
            shape
        }
    }

    pub fn contains(&self, point: Coordinates) -> bool {
        let start = self.pointer;
        let end = Coordinates {
            x: start.x + self.shape.width - 1,
            y: start.y + self.shape.height - 1,
        };

        point.x >= start.x && point.x <= end.x && point.y >= start.y && point.y <= end.y
    }

    pub fn is_blocking_at(&self, target: Coordinates) -> bool {
        if !self.effect.block {
            return false;
        }

        if let Some((start, end)) = self.effect.shrink {
            // Shrink is interpreted as absolute bounds
            target.x >= start.x && target.x <= end.x && target.y >= start.y && target.y <= end.y
        } else {
            self.contains(target)
        }
    }

    pub fn offset(&mut self, delta: Coordinates) {
        self.pointer.x += delta.x;
        self.pointer.y += delta.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_its_coordinates() {
        let tile = Tile::new(
            0,
            Effect::default(),
            Coordinates { x: 0, y: 0 },
            Shape::from_square(3),
        );

        assert!(tile.contains(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn doesnt_contain_out_of_bounds_coordinates() {
        let tile = Tile::new(
            0,
            Effect::default(),
            Coordinates { x: 0, y: 0 },
            Shape::from_square(2),
            
        );
        assert!(!tile.contains(Coordinates { x: 3, y: 3 }));
    }

    #[test]
    fn is_blocking_when_required() {
        let effect = Effect {
            block: true,
            shrink: None,
            ..Default::default()
        };
        let tile = Tile::new(0, effect,Coordinates { x: 0, y: 0 }, Shape::from_square(2));
        assert!(tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn is_not_blocking_by_default() {
        let effect = Effect {
            block: false,
            shrink: None,
            ..Default::default()
        };
        let tile = Tile::new(0, effect, Coordinates { x: 0, y: 0 }, Shape::from_square(2));
        assert!(!tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn is_blocking_when_within_shrink_bounds() {
        let effect = Effect {
            block: true,
            shrink: Some((Coordinates { x: 1, y: 1 }, Coordinates { x: 2, y: 2 })),
            ..Default::default()
        };
        let tile = Tile::new(0, effect, Coordinates { x: 0, y: 0 }, Shape::from_square(4));
        assert!(tile.is_blocking_at(Coordinates { x: 2, y: 2 }));
        assert!(!tile.is_blocking_at(Coordinates { x: 0, y: 0 }));
    }

    #[test]
    fn offset_modifies_pointer() {
        let mut tile = Tile::new(
            0,
            Effect::default(),
            Coordinates { x: 2, y: 3 },
            Shape::from_square(1),
        );
        tile.offset(Coordinates { x: 1, y: 2 });
        assert_eq!(tile.pointer, Coordinates { x: 3, y: 5 });
    }
}
