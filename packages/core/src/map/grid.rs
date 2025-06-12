use crate::prelude::{Coordinates, Shape, Tile};

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

    pub fn offset(&mut self, delta: Coordinates) {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }
        self.shape.width = (self.shape.width + delta.x).max(0);
        self.shape.height = (self.shape.height + delta.y).max(0);
    }
}
