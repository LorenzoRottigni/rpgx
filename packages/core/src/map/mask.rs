use crate::{
    map::grid::Grid,
    prelude::{Delta, Effect, Rect, Tile},
};

/// A [`Mask`] defines a logical area on a grid where specific [`Effect`]s are applied.
///
/// Masks group tiles that match spatial or logical patterns, enabling batch application of effects
/// such as collision blocks, visual overlays, triggers, or behaviors. They are commonly used during
/// map construction or dynamic modification to efficiently define areas of interaction or decoration.
#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,
    /// Grid containing tiles that form the mask's area and effects.
    pub grid: Grid,
}

impl Mask {
    /// Creates a new mask with a given name, rectangular areas, and uniform effect applied to all tiles.
    ///
    /// # Arguments
    /// * `name` - The name of the mask.
    /// * `areas` - Vector of `Rect` areas to create tiles from.
    /// * `effect` - Effect to apply to all tiles in the mask.
    ///
    /// # Returns
    /// * A new `Mask` instance with tiles covering the given areas.
    pub fn new(name: String, areas: Vec<Rect>, effect: Effect) -> Self {
        let tiles = areas
            .into_iter()
            .map(|area| Tile { area, effect })
            .collect();

        let grid = Grid::new(tiles);

        Self { name, grid }
    }

    /// Offsets all tiles in the mask and their effects by the specified delta.
    ///
    /// This moves each tile's area and updates the effect's internal rectangles accordingly.
    ///
    /// # Arguments
    /// * `delta` - The offset to apply.
    pub fn offset(&mut self, delta: Delta) {
        for tile in &mut self.grid.tiles {
            tile.area.offset(delta);
            tile.effect.offset(delta);
        }
    }
}

impl Effect {
    /// Offsets any internal rectangles of the effect (e.g., blocking region) by the given delta.
    ///
    /// # Arguments
    /// * `delta` - The offset to apply.
    pub fn offset(&mut self, delta: Delta) {
        if let Some(ref mut block_rect) = self.block {
            block_rect.offset(delta);
        }
        // Add other rect-like fields here if needed.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape, Tile};

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
        assert_eq!(mask.grid.tiles.len(), areas.len());
        for (tile, area) in mask.grid.tiles.iter().zip(areas.iter()) {
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

        let tile = &mask.grid.tiles[0];
        assert_eq!(tile.area.origin.x, 5);
        assert_eq!(tile.area.origin.y, 7);

        let block = tile.effect.block.expect("Effect block should be set");
        assert_eq!(block.origin.x, 6); // 1 + 5
        assert_eq!(block.origin.y, 8); // 1 + 7

        // Shape remains unchanged
        assert_eq!(block.shape.width, 2);
        assert_eq!(block.shape.height, 2);
    }
}
