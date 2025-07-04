use crate::{
    prelude::{Coordinates, Delta, Mask, Shape},
    traits::{Grid, Shaped, Shiftable},
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
}

impl Layer {
    /// Creates a new layer with a name, masks, and z-index.
    pub fn new(name: String, masks: Vec<Mask>, z: u32) -> Self {
        Self { name, masks, z }
    }
}

impl Shaped for Layer {
    /// Returns the overall bounding shape of all masks.
    fn get_shape(&self) -> Shape {
        let shapes: Vec<Shape> = self.masks.iter().map(|mask| mask.get_shape()).collect();
        Shape::bounding_shape(&shapes)
    }
}

impl Shiftable for Layer {
    /// Offsets all tiles in the layer by the given delta.
    fn offset(&mut self, delta: Delta) {
        for mask in &mut self.masks {
            mask.offset(delta);
        }
    }

    fn translate(&self, delta: Delta) -> Self {
        let mut new_layer = self.clone();
        new_layer.offset(delta);
        new_layer
    }
}

impl Grid for Layer {
    /// Checks if the layer contains a tile at the specified coordinate.
    fn contains(&self, coord: &Coordinates) -> bool {
        self.masks.iter().any(|mask| mask.contains(coord))
    }
}

impl Layer {
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.masks.iter().any(|mask| mask.is_blocking_at(target))
    }

    pub fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        self.masks
            .iter()
            .flat_map(|mask| {
                if mask.contains(target) {
                    mask.get_actions()
                } else {
                    vec![]
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Rect;

    fn simple_layer() -> Layer {
        let mask = Mask::new(
            "test".into(),
            vec![Rect::from_shape(Shape::new(2, 2))],
            vec![],
        );
        Layer::new("layer".into(), vec![mask], 0)
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
    }
}
