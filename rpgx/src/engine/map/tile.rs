use crate::common::{coordinates::Coordinates, shape::Shape};

use super::{effect::Effect, selector::SingleSelector};

/// Represents a single tile on the grid with unique identifier, spatial information, and effects applied.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub id: i32,
    pub effect: Effect,
    pub pointer: SingleSelector,
    pub shape: Shape,
}

impl Tile {
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

    pub fn generate_default_grid(shape: Shape, effect: Effect) -> Vec<Self> {
        let mut tiles = Vec::new();
        for y in 0..shape.height {
            for x in 0..shape.width {
                tiles.push(Tile {
                    id: x,
                    pointer: Coordinates { x, y },
                    shape: Shape::from_square(1),
                    effect,
                });
            }
        }
        tiles
    }

    pub fn offset(&mut self, delta: Coordinates) {
        self.pointer.x += delta.x;
        self.pointer.y += delta.y;
    }
}
