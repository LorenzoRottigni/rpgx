use crate::{
    prelude::{Coordinates, Delta, Effect, Mask, Rect, Tile},
    traits::{Bounded, Grid, Renderable, Spatial},
};

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
    /// Offset applied at rendering time to all masks in the layer
    pub offset: Delta,
}

impl Renderable for Layer {
    fn render(&self) -> Vec<Tile> {
        self.masks
            .iter()
            .flat_map(|mask| {
                let mut mask = mask.clone();
                mask.offset = mask.offset + self.offset;
                mask.render()
            })
            .collect()
    }
}

impl Bounded for Layer {
    fn get_bounding_rect(&self) -> Rect {
        if self.masks.is_empty() {
            return Rect::default();
        }

        // Collect all mask bounds, translated by self.offset
        let translated_bounds: Vec<Rect> = self
            .masks
            .iter()
            .map(|mask| {
                let mut bounds = mask.get_bounding_rect();
                bounds.origin = (bounds.origin + self.offset).unwrap();
                bounds
            })
            .collect();

        // Compute the bounding rect of all translated bounds
        Rect::bounding_rect(&translated_bounds)
    }
}

impl Spatial for Layer {
    fn contains(&self, target: &Coordinates) -> bool {
        self.masks.iter().any(|mask| mask.contains(target))
    }
}

impl Grid for Layer {
    fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        self.masks
            .iter()
            .flat_map(|mask| mask.get_actions_at(target))
            .collect()
    }

    fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.masks.iter().any(|mask| mask.is_blocking_at(target))
    }

    fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect> {
        self.masks
            .iter()
            .flat_map(|mask| mask.get_effects_at(target))
            .collect()
    }
}

impl Layer {
    /// Creates a new layer with a name, masks, and z-index.
    pub fn new(name: String, masks: Vec<Mask>, z: u32) -> Self {
        Self {
            name,
            masks,
            z,
            offset: Delta::default(),
        }
    }

    pub fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect> {
        self.masks
            .iter()
            .filter_map(|mask| {
                if mask.contains(target) {
                    Some(mask.effect)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if any tile in the layer blocks movement at the given coordinate.
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.masks.iter().any(|mask| mask.is_blocking_at(target))
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
        let shape = layer.get_bounding_rect().shape;
        assert_eq!(shape.width, 2);
        assert_eq!(shape.height, 2);
    }

    #[test]
    fn test_render_tiles() {
        let layer = simple_layer();
        let tiles = layer.render();
        assert_eq!(tiles.len(), 1);
    }
}
