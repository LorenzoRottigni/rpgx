use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape, Tile};

#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,
    /// Tiles that define the mask area and their effects.
    pub tiles: Vec<Tile>,
}

impl Mask {
    /// Creates a new mask with a given name, rectangular areas, and uniform effect.
    pub fn new(name: String, areas: Vec<Rect>, effect: Effect) -> Self {
        let tiles = areas
            .into_iter()
            .map(|area| Tile { area, effect })
            .collect();

        Self { name, tiles }
    }

    /// Offsets all tiles in the mask and their effects by the specified delta.
    pub fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.area.offset(delta);
            tile.effect.offset(delta);
        }
    }

    /// Computes the bounding shape of all tiles.
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
    use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape};

    #[test]
    fn mask_new_creates_tiles() {
        let effect = Effect {
            block: Some(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
            ..Default::default()
        };
        let areas = vec![
            Rect::new(Coordinates::new(0, 0), Shape::new(1, 1)),
            Rect::new(Coordinates::new(2, 2), Shape::new(3, 3)),
        ];
        let mask = Mask::new("test_mask".to_string(), areas.clone(), effect);

        assert_eq!(mask.name, "test_mask");
        assert_eq!(mask.tiles.len(), areas.len());

        for (tile, area) in mask.tiles.iter().zip(areas.iter()) {
            assert_eq!(tile.area, *area);
            assert_eq!(tile.effect, effect);
        }
    }

    #[test]
    fn mask_offset_moves_tiles_and_effects() {
        let effect = Effect {
            block: Some(Rect::new(Coordinates::new(1, 1), Shape::new(2, 2))),
            ..Default::default()
        };
        let area = Rect::new(Coordinates::new(0, 0), Shape::new(3, 3));
        let mut mask = Mask::new("offset_mask".to_string(), vec![area], effect);

        let delta = Delta::new(5, 7);
        mask.offset(delta);

        let tile = &mask.tiles[0];
        assert_eq!(tile.area.origin.x, 5);
        assert_eq!(tile.area.origin.y, 7);

        let block = tile.effect.block.expect("Effect block should be set");
        assert_eq!(block.origin.x, 6); // 1 + 5
        assert_eq!(block.origin.y, 8); // 1 + 7

        assert_eq!(block.shape.width, 2);
        assert_eq!(block.shape.height, 2);
    }

    #[test]
    fn mask_get_shape_returns_correct_bounds() {
        let tiles = vec![
            Rect::new(Coordinates::new(1, 1), Shape::new(2, 2)),
            Rect::new(Coordinates::new(4, 3), Shape::new(3, 1)),
        ];
        let mask = Mask::new("shape_mask".to_string(), tiles.clone(), Effect::default());

        let shape = mask.get_shape();
        assert_eq!(shape.width, 7); // 4 + 3
        assert_eq!(shape.height, 4); // max(1+2, 3+1)
    }

    #[test]
    fn mask_contains_and_tile_at_work() {
        let tile = Rect::new(Coordinates::new(5, 5), Shape::new(2, 2));
        let mask = Mask::new("lookup_mask".to_string(), vec![tile], Effect::default());

        assert!(mask.contains(Coordinates::new(5, 5)));
        assert!(mask.contains(Coordinates::new(6, 6)));
        assert!(!mask.contains(Coordinates::new(7, 7)));

        let found = mask.tile_at(Coordinates::new(6, 6));
        assert!(found.is_some());
        assert_eq!(found.unwrap().area, tile);

        assert!(mask.tile_at(Coordinates::new(7, 7)).is_none());
    }
}
