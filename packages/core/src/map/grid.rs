use crate::prelude::{Delta, Shape, Tile};

/// Represents a grid of tiles arranged in a 2D space.
///
/// The grid keeps track of its bounding shape (width × height),
/// and the tiles it contains, each with their own spatial properties.
///
/// The grid shape is automatically computed from tiles on construction.
///
/// # Note
/// - Offsetting the grid moves all tiles and updates the grid shape accordingly.
/// - Shape always covers the bounding rectangle containing all tiles.
#[derive(Clone, Debug)]
pub struct Grid {
    /// Collection of tiles in the grid.
    pub tiles: Vec<Tile>,

    /// Bounding shape of the grid (width and height).
    pub shape: Shape,
}

impl Grid {
    /// Creates a new grid from the given tiles.
    ///
    /// The grid shape is computed as the minimal bounding rectangle that
    /// contains all tile areas.
    ///
    /// If tiles is empty, shape defaults to zero width and height.
    pub fn new(tiles: Vec<Tile>) -> Self {
        let shape = if tiles.is_empty() {
            Shape::default()
        } else {
            let (max_x, max_y) = tiles.iter().fold((0, 0), |(mx, my), tile| {
                let x_end = tile.area.origin.x + tile.area.shape.width;
                let y_end = tile.area.origin.y + tile.area.shape.height;
                (mx.max(x_end), my.max(y_end))
            });

            Shape::from_rectangle(max_x, max_y)
        };

        Self { tiles, shape }
    }

    /// Offsets the entire grid and its tiles by the given delta.
    ///
    /// Moves all tiles by `delta` and adjusts the grid's shape accordingly,
    /// saturating at zero to prevent underflow.
    pub fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }

        // Adjust width safely
        if delta.dx < 0 {
            self.shape.width = self.shape.width.saturating_sub((-delta.dx) as u32);
        } else {
            self.shape.width = self.shape.width.saturating_add(delta.dx as u32);
        }

        // Adjust height safely
        if delta.dy < 0 {
            self.shape.height = self.shape.height.saturating_sub((-delta.dy) as u32);
        } else {
            self.shape.height = self.shape.height.saturating_add(delta.dy as u32);
        }
    }

    /// Returns true if any tile contains the given coordinate.
    ///
    /// Useful for quick containment checks within the grid.
    pub fn contains(&self, coord: crate::prelude::Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.contains(coord))
    }

    /// Returns the tile at the specified coordinate, if any.
    pub fn tile_at(&self, coord: crate::prelude::Coordinates) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.contains(coord))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Effect, Rect};

    #[test]
    fn test_grid_new_empty() {
        let grid = Grid::new(Vec::new());
        assert_eq!(grid.shape.width, 0);
        assert_eq!(grid.shape.height, 0);
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
        let grid = Grid::new(vec![tile1, tile2]);

        // Bounding box should cover max x=7 (3+4) and max y=3 (2+1)
        assert_eq!(grid.shape.width, 7);
        assert_eq!(grid.shape.height, 4);
        assert_eq!(grid.tiles.len(), 2);
    }

    #[test]
    fn test_grid_offset() {
        let mut grid = Grid::new(vec![Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 1, y: 1 }, Shape::from_rectangle(2, 2)),
        )]);

        // initial shape width = 1 + 2 = 3
        // initial shape height = 1 + 2 = 3
        assert_eq!(grid.shape.width, 3);
        assert_eq!(grid.shape.height, 3);

        grid.offset(Delta::new(2, 3));
        let tile = &grid.tiles[0];

        // Tile should be offset by (2, 3)
        assert_eq!(tile.area.origin.x, 3);
        assert_eq!(tile.area.origin.y, 4);

        // Grid shape should be adjusted accordingly (width + 2, height + 3)
        assert_eq!(grid.shape.width, 5); // 3 + 2
        assert_eq!(grid.shape.height, 6); // 3 + 3
    }

    #[test]
    fn test_grid_contains_and_tile_at() {
        let tile = Tile::new(
            Effect::default(),
            Rect::new(Coordinates { x: 5, y: 5 }, Shape::from_rectangle(2, 2)),
        );
        let grid = Grid::new(vec![tile]);

        // Point inside tile
        assert!(grid.contains(Coordinates { x: 5, y: 5 }));
        assert!(grid.contains(Coordinates { x: 6, y: 6 }));

        // Point outside tile
        assert!(!grid.contains(Coordinates { x: 7, y: 7 }));

        // tile_at returns Some for inside coords, None otherwise
        assert_eq!(grid.tile_at(Coordinates { x: 5, y: 5 }), Some(&tile));
        assert_eq!(grid.tile_at(Coordinates { x: 7, y: 7 }), None);
    }
}
