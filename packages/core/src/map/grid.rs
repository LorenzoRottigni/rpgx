use crate::{
    common::delta::Delta,
    prelude::{Shape, Tile},
};

#[derive(Clone, Debug)]
pub struct Grid {
    pub tiles: Vec<Tile>,
    pub shape: Shape,
}

impl Grid {
    pub fn new(tiles: Vec<Tile>) -> Self {
        let shape = if tiles.is_empty() {
            Shape::default()
        } else {
            let (max_x, max_y) = tiles.iter().fold((0, 0), |(mx, my), tile| {
                (mx.max(tile.pointer.x), my.max(tile.pointer.y))
            });

            // Add 1 because shape is width/height, and coordinates are 0-indexed
            Shape::from_rectangle(max_x + 1, max_y + 1)
        };

        Self { tiles, shape }
    }

    pub fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }

        // Apply delta safely, preventing underflow
        if delta.dx < 0 {
            self.shape.width = self.shape.width.saturating_sub((-delta.dx) as u32);
        } else {
            self.shape.width = self.shape.width.saturating_add(delta.dx as u32);
        }

        if delta.dy < 0 {
            self.shape.height = self.shape.height.saturating_sub((-delta.dy) as u32);
        } else {
            self.shape.height = self.shape.height.saturating_add(delta.dy as u32);
        }
    }
}
