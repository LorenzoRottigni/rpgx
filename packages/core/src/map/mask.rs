use crate::{
    prelude::{Coordinates, Delta, Effect, Rect, Shape},
    traits::{Grid, Shaped, Shiftable},
};

impl Mask {
    /// Creates a new mask with a given name, rectangular areas, and uniform effect.
    pub fn new(name: String, tiles: Vec<Rect>, effect: Effect) -> Self {
        Self {
            name,
            tiles,
            effect,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,
    /// Tiles that define the mask area and their effects.
    pub tiles: Vec<Rect>,
    pub effect: Effect,
}

impl Shaped for Mask {
    /// Computes the bounding shape of all tiles.
    fn get_shape(&self) -> Shape {
        if self.tiles.is_empty() {
            return Shape::default();
        }

        let (max_x, max_y) = self.tiles.iter().fold((0, 0), |(mx, my), tile| {
            let x_end = tile.origin.x + tile.shape.width;
            let y_end = tile.origin.y + tile.shape.height;
            (mx.max(x_end), my.max(y_end))
        });

        Shape::from_rectangle(max_x, max_y)
    }
}

impl Shiftable for Mask {
    /// Offsets the mask and all its tiles by the specified delta.
    fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.offset(delta);
            self.effect.offset(delta);
        }
    }

    /// Returns a new mask with the offset applied.
    fn translate(&self, delta: Delta) -> Self {
        let mut new_mask = self.clone();
        new_mask.offset(delta);
        new_mask
    }
}

impl Mask {
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        if let Some(block) = self.effect.block {
            self.contains(target) && block.contains(target)
        } else {
            false
        }
    }
}

impl Grid for Mask {
    /// Checks if the mask contains the specified coordinate.
    fn contains(&self, coord: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.contains(coord))
    }

    //  /// Returns all tiles at the specified coordinate.
    //  fn get_tiles_at(&self, pointer: Coordinates) -> Vec<Rect> {
    //      self.tiles
    //          .iter()
    //          .filter(|tile| tile.contains(pointer))
    //          .cloned()
    //          .collect()
    //  }
    //
    //  /// Checks if any tile blocks movement at the specified coordinate.

    //
    //  /// Checks if movement is allowed at the specified coordinate.
    //  fn move_allowed(&self, target: Coordinates) -> bool {
    //      self.contains(target) && !self.is_blocking_at(&target)
    //  }
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
            assert_eq!(*tile, *area);
            assert_eq!(mask.effect, effect);
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
        assert_eq!(tile.origin.x, 5);
        assert_eq!(tile.origin.y, 7);

        let block = mask.effect.block.expect("Effect block should be set");
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
}
