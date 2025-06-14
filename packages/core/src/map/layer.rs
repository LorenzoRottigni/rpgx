use crate::prelude::{Coordinates, Delta, Mask, Shape, Tile};

#[doc = include_str!("../../docs/layer.md")]
/// A `Layer` is a logical or visual overlay composed of [`Mask`]s that apply [`Effect`]s to specific tiles.
///
/// Layers allow grouped application of tile-based modifications (e.g. collision, decoration, triggers)
/// without altering the original base grid. Layers are rendered or processed in Z-order, determined by `z`.
#[derive(Clone, Debug)]
pub struct Layer {
    /// The name of the layer (e.g., `"collision"`, `"visuals"`)
    pub name: String,
    /// A list of masks (effect areas) applied in this layer
    pub masks: Vec<Mask>,
    /// Z-index for stacking and rendering order
    pub z: u32,
}

impl Layer {
    /// Creates a new layer with a name, masks, and z-index.
    pub fn new(name: String, masks: Vec<Mask>, z: u32) -> Self {
        Self { name, masks, z }
    }

    /// Returns the first tile that contains the given coordinates.
    ///
    /// Checks all tiles in all masks. Supports tiles with shape larger than 1×1.
    pub fn get_tile_at(&self, pointer: Coordinates) -> Option<Tile> {
        self.masks
            .iter()
            .flat_map(|mask| mask.grid.tiles.iter().cloned())
            .find(|tile| {
                if pointer.x >= tile.area.origin.x && pointer.y >= tile.area.origin.y {
                    let local = Coordinates {
                        x: pointer.x - tile.area.origin.x,
                        y: pointer.y - tile.area.origin.y,
                    };
                    tile.area.shape.in_bounds(local)
                } else {
                    false
                }
            })
    }

    /// Checks if any tile in the layer blocks movement at the given coordinate.
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.masks.iter().any(|mask| {
            mask.grid
                .tiles
                .iter()
                .any(|tile| tile.is_blocking_at(*target))
        })
    }

    /// Returns the individual shapes of all masks in the layer.
    pub fn get_shapes(&self) -> Vec<Shape> {
        self.masks.iter().map(|mask| mask.grid.shape).collect()
    }

    /// Returns the overall bounding shape of all masks.
    pub fn get_shape(&self) -> Shape {
        Shape::bounding_shape(&self.get_shapes())
    }

    /// Returns all tiles in the layer, flattened.
    pub fn render(&self) -> Vec<Tile> {
        self.masks
            .iter()
            .flat_map(|mask| mask.grid.tiles.iter().cloned())
            .collect()
    }

    /// Offsets all tiles in the layer by the given delta.
    pub fn offset(&mut self, delta: Delta) {
        for mask in &mut self.masks {
            mask.offset(delta);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Effect, Rect};

    fn dummy_tile(x: u32, y: u32, w: u32, h: u32) -> Tile {
        Tile::new(
            Effect {
                block: Some(Rect::from_xywh(x, y, w, h)),
                ..Default::default()
            },
            Rect::from_xywh(x, y, w, h),
        )
    }

    fn simple_layer() -> Layer {
        let tile = dummy_tile(0, 0, 2, 2);
        let mask = Mask::new("test".into(), vec![tile.area], tile.effect);
        Layer::new("layer".into(), vec![mask], 0)
    }

    #[test]
    fn test_get_tile_at_hit() {
        let layer = simple_layer();
        assert!(layer.get_tile_at(Coordinates::new(1, 1)).is_some());
    }

    #[test]
    fn test_get_tile_at_miss() {
        let layer = simple_layer();
        assert!(layer.get_tile_at(Coordinates::new(3, 3)).is_none());
    }

    #[test]
    fn test_is_blocking_at_true() {
        let layer = simple_layer();
        assert!(layer.is_blocking_at(&Coordinates::new(1, 1)));
    }

    #[test]
    fn test_is_blocking_at_false() {
        let layer = simple_layer();
        assert!(!layer.is_blocking_at(&Coordinates::new(5, 5)));
    }

    #[test]
    fn test_get_shapes_and_shape() {
        let layer = simple_layer();
        let shape = layer.get_shape();
        assert_eq!(shape.width, 2);
        assert_eq!(shape.height, 2);
    }

    #[test]
    fn test_offset_layer() {
        let mut layer = simple_layer();
        layer.offset(Delta::new(2, 3));
        let shape = layer.get_shape();
        assert!(shape.width >= 2);
        assert!(shape.height >= 2);
        assert!(layer.get_tile_at(Coordinates::new(2, 3)).is_some());
    }

    #[test]
    fn test_render_tiles() {
        let layer = simple_layer();
        let tiles = layer.render();
        assert_eq!(tiles.len(), 1);
    }
}
