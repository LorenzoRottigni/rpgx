use crate::prelude::{Coordinates, Delta, Shape, Tile};

/// Represents a grid of tiles arranged in a 2D space.
///
/// The grid keeps track of the tiles it contains, each with their own spatial properties.
///
/// The shape is dynamically computed from the tile positions.
///
/// # Note
/// - Offsetting the grid moves all tiles.
/// - `get_shape` computes the bounding rectangle containing all tiles.
#[derive(Clone, Debug)]
pub struct Grid {
    /// Collection of tiles in the grid.
    pub tiles: Vec<Tile>,
}

impl Grid {
    /// Creates a new grid from the given tiles.
    ///
    /// If `tiles` is empty, the shape is considered (0, 0).
    pub fn new(tiles: Vec<Tile>) -> Self {
        Self { tiles }
    }

    /// Offsets all tiles in the grid by the given delta.
    pub fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }
    }

    /// Dynamically computes the bounding shape of the grid.
    ///
    /// Returns a `Shape` that represents the minimum rectangle covering all tiles.
    pub fn get_shape(&self) -> Shape {
        if self.tiles.is_empty() {
            return Shape::default();
        }

        let (max_x, max_y) = self.tiles.iter().fold((0, 0), |(mx, my), tile| {
            let x_end = tile.area.origin.x + tile.area.shape.width;
            let y_end = tile.area.origin.y + tile.area.shape.height;
            (mx.max(x_end), my.max(y_end))
        });

        Shape::from_rectangle(max_x, max_y)
    }

    /// Returns true if any tile contains the given coordinate.
    pub fn contains(&self, coord: Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.contains(coord))
    }

    /// Returns the tile at the specified coordinate, if any.
    pub fn tile_at(&self, coord: Coordinates) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.contains(coord))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Effect, Rect};

    #[test]
    fn test_grid_new_empty() {
        let grid = Grid::new(Vec::new());
        let shape = grid.get_shape();
        assert_eq!(shape.width, 0);
        assert_eq!(shape.height, 0);
        assert!(grid.tiles.is_empty());
    }

    #[test]
    fn test_grid_new_with_tiles() {
        let tile1 = Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 1, y: 1 }, Shape::from_rectangle(2, 3)),
        );
        let tile2 = Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 3, y: 2 }, Shape::from_rectangle(4, 1)),
        );
        let grid = Grid::new(vec![tile1.clone(), tile2.clone()]);

        let shape = grid.get_shape();

        assert_eq!(shape.width, 7); // 3 (x) + 4 (w) = 7
        assert_eq!(shape.height, 4); // 2 (y) + 1 (h) = 3 -> max(1+3,2+1) = 4
        assert_eq!(grid.tiles.len(), 2);
    }

    #[test]
    fn test_grid_offset() {
        let mut grid = Grid::new(vec![Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 1, y: 1 }, Shape::from_rectangle(2, 2)),
        )]);

        let shape = grid.get_shape();
        assert_eq!(shape.width, 3);
        assert_eq!(shape.height, 3);

        grid.offset(Delta::new(2, 3));
        let tile = &grid.tiles[0];

        assert_eq!(tile.area.origin.x, 3);
        assert_eq!(tile.area.origin.y, 4);

        let shape_after = grid.get_shape();
        assert_eq!(shape_after.width, 5); // 3 + 2
        assert_eq!(shape_after.height, 6); // 3 + 3
    }

    #[test]
    fn test_grid_contains_and_tile_at() {
        let tile = Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 5, y: 5 }, Shape::from_rectangle(2, 2)),
        );
        let grid = Grid::new(vec![tile.clone()]);

        assert!(grid.contains(Coordinates { x: 5, y: 5 }));
        assert!(grid.contains(Coordinates { x: 6, y: 6 }));
        assert!(!grid.contains(Coordinates { x: 7, y: 7 }));

        assert_eq!(grid.tile_at(Coordinates { x: 5, y: 5 }), Some(&tile));
        assert_eq!(grid.tile_at(Coordinates { x: 7, y: 7 }), None);
    }
}
