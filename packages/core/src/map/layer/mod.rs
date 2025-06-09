use mask::Mask;

pub use crate::prelude::{BlockSelector, Coordinates, Effect, Shape, SingleSelector, Tile};

pub mod mask;

/// Represents the different roles a [`Layer`] can play in the [`Grid`] stack.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LayerType {
    /// The base of all layers, includes all tiles of a Shape which takes into account all layers shapes.
    Base,
    /// Any action has to be placed within this layer, or won't be triggered within RPGX flows.
    Action,
    /// This layer is merged into the base layer and modifies base tiles.
    Texture,
    /// A special layer that uses unstandard tile shapes (e.g., 4×5 or 3×3 instead of 1×1).
    Block,
}

#[doc = include_str!("../../../docs/layer.md")]
/// A visual or logical overlay on top of the base grid, used to apply effects
/// to specific [`Tile`]s based on spatial [`Mask`]s and [`Selector`]s.
///
/// Layers simulate stacking behavior along the Z-axis and allow grouped or
/// conditional tile modifications without altering the original base.
#[derive(Clone, Debug)]
pub struct Layer {
    /// The name of the layer (e.g., `"collision"`, `"visuals"`)
    pub name: String,
    /// The type of layer (e.g., `Base`, `Action`, `Texture`, etc.)
    pub kind: LayerType,
    /// All tiles currently active within the layer.
    pub tiles: Vec<Tile>,
    /// The shape (bounds) of the layer.
    pub shape: Shape,
    /// All masks that were used to generate the tiles.
    pub masks: Vec<Mask>,
    /// Z-index for rendering order
    pub z: u32,
}

impl Layer {
    /// Constructs a new non-base layer by applying masks to the given shape.
    ///
    /// # Panics
    /// Panics if called with [`LayerType::Base`] (use [`Layer::base`] instead).
    ///
    /// Applies each mask's selector and effect over the `shape` to generate tiles.
    pub fn new(name: String, kind: LayerType, shape: Shape, masks: Vec<Mask>, z: u32) -> Self {
        assert!(
            kind != LayerType::Base,
            "Use Layer::base instead of Layer::new for Base layers"
        );

        let tiles = masks.iter().flat_map(|mask| mask.apply(shape)).collect();
        Self {
            name,
            kind,
            tiles,
            shape,
            masks,
            z,
        }
    }

    /// Constructs the base layer from a group of other layers.
    ///
    /// This layer will have a unified shape that encompasses all others and may
    /// merge any tiles from [`LayerType::Texture`] layers into itself by copying their effects.
    ///
    /// The base layer's tiles cover the entire unified shape with 1×1 squares by default.
    pub fn base(layers: Vec<Self>) -> Self {
        let mut base_shape = Shape::default();

        // Compute the union of all layer shapes to find the base bounds
        for layer in &layers {
            base_shape = base_shape.union(layer.shape);
        }

        // Create a uniform grid of tiles across the entire base shape
        let mut tiles = Vec::new();
        for y in 0..base_shape.height {
            for x in 0..base_shape.width {
                tiles.push(Tile {
                    id: x as u32,
                    pointer: Coordinates { x, y },
                    shape: Shape::from_square(1),
                    effect: Effect::default(),
                });
            }
        }

        let mut base_layer = Self {
            name: "base".to_string(),
            kind: LayerType::Base,
            shape: base_shape,
            tiles,
            masks: vec![],
            z: 1,
        };

        // Sort layers by their Z to apply texture effects in order
        let mut layers_by_z = layers.clone();
        layers_by_z.sort_by_key(|layer| layer.z);

        for layer in &layers_by_z {
            base_layer.positive_reshape(layer.shape);

            if layer.kind == LayerType::Texture {
                // For each texture tile, update base tile effect if coordinates match
                'tileloop: for tile in &layer.tiles {
                    for base_tile in base_layer.tiles.iter_mut() {
                        if base_tile.pointer == tile.pointer {
                            base_tile.effect = tile.effect;
                            continue 'tileloop;
                        }
                    }
                }
            }
        }

        base_layer
    }
}

impl Layer {
    /// Reshapes the layer to the given bounds.
    ///
    /// This will discard any tiles outside the shape. For `Base` layers,
    /// tiles are regenerated to fill the new shape, preserving any previous effects.
    pub fn reshape(&mut self, shape: Shape) {
        self.shape = shape;

        // Retain only tiles inside the new shape
        let mut tiles = vec![];
        for tile in self.tiles.iter() {
            if shape.in_bounds(tile.pointer) {
                tiles.push(*tile)
            }
        }
        self.tiles = tiles;

        // For base layers, regenerate tiles to fill the new shape
        if self.kind == LayerType::Base {
            let mut base_tiles = Vec::new();
            for y in 0..self.shape.height {
                for x in 0..self.shape.width {
                    let pointer = Coordinates { x, y };
                    // Preserve effect from previous tile if present
                    let effect = if let Some(tile) = self.get_tile_at(pointer) {
                        tile.effect
                    } else {
                        Effect::default()
                    };
                    base_tiles.push(Tile {
                        id: x as u32,
                        pointer,
                        shape: Shape::from_square(1),
                        effect,
                    });
                }
            }
            self.tiles = base_tiles;
        }
    }

    /// Expands the shape to include the provided shape (only grows, never shrinks).
    ///
    /// Calls [`reshape`] internally if expansion occurs.
    pub fn positive_reshape(&mut self, shape: Shape) {
        let mut changed = false;
        if shape.width > self.shape.width {
            self.shape.width = shape.width;
            changed = true;
        }
        if shape.height > self.shape.height {
            self.shape.height = shape.height;
            changed = true;
        }
        if changed {
            self.reshape(self.shape);
        }
    }

    /// Finds the tile covering the given coordinates, accounting for both tile origin and shape.
    ///
    /// Supports tiles larger than 1×1 by checking if the coordinate lies within the tile's shape.
    pub fn get_tile_at(&self, pointer: Coordinates) -> Option<Tile> {
        self.tiles
            .iter()
            .find(|tile| {
                // Avoid underflow for subtraction by checking coordinate ordering
                if pointer.x >= tile.pointer.x && pointer.y >= tile.pointer.y {
                    let local = Coordinates {
                        x: pointer.x - tile.pointer.x,
                        y: pointer.y - tile.pointer.y,
                    };
                    tile.shape.in_bounds(local)
                } else {
                    false
                }
            })
            .cloned()
    }

    /// Retrieves all tiles within a rectangular block defined by two coordinates.
    ///
    /// Only returns tiles that exist exactly at those coordinates (tiles with larger shape ignored if not top-left).
    pub fn get_block_at(&self, pointer: BlockSelector) -> Vec<Tile> {
        self.shape
            .coordinates_in_range(pointer.0, pointer.1)
            .into_iter()
            .filter_map(|coord| self.tiles.iter().find(|t| t.pointer == coord).cloned())
            .collect()
    }

    /// Checks if any tile covering the given coordinate is blocking.
    ///
    /// This method considers tiles with larger shapes.
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.is_blocking_at(*target))
    }

    /// Offsets all tiles and the shape of the layer by the given delta.
    ///
    /// The shape dimensions increase by delta.x and delta.y (capped to zero minimum).
    pub fn offset(&mut self, delta: Coordinates) {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }

        self.shape.width = (self.shape.width + delta.x).max(0);
        self.shape.height = (self.shape.height + delta.y).max(0);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{Selector, SingleSelector};

    #[test]
    fn applies_multiple_masks_correctly() {
        let mask1 = Mask {
            name: "First".to_string(),
            selector: Selector::Single(SingleSelector { x: 0, y: 0 }),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        let mask2 = Mask {
            name: "Second".to_string(),
            selector: Selector::Single(SingleSelector { x: 1, y: 0 }),
            effect: Effect {
                action_id: Some(1),
                ..Default::default()
            },
        };

        let shape = Shape::from_square(2);
        let layer = Layer::new(
            "MultiMask".to_string(),
            LayerType::Action,
            shape,
            vec![mask1.clone(), mask2.clone()],
            1,
        );

        assert_eq!(layer.tiles.len(), 2);
        assert!(
            layer
                .get_tile_at(SingleSelector { x: 0, y: 0 })
                .unwrap()
                .effect
                .block
        );
        assert_eq!(
            layer
                .get_tile_at(SingleSelector { x: 1, y: 0 })
                .unwrap()
                .effect
                .action_id,
            Some(1)
        );
    }

    #[test]
    fn applies_mask_to_block_of_tiles() {
        let mask = Mask {
            name: "BlockMask".to_string(),
            selector: Selector::Block((Coordinates { x: 0, y: 0 }, Coordinates { x: 2, y: 2 })),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        let shape = Shape::from_square(3);
        let layer = Layer::new(
            "MaskedDefault".to_string(),
            LayerType::Texture,
            shape,
            vec![mask],
            1,
        );

        let block =
            layer.get_block_at((SingleSelector { x: 0, y: 0 }, SingleSelector { x: 2, y: 2 }));
        assert_eq!(block.len(), 9);
        for tile in block {
            assert!(tile.effect.block);
        }
    }

    #[test]
    fn empty_shape_produces_no_tiles() {
        let shape = Shape {
            width: 0,
            height: 0,
        };
        let layer = Layer::new(
            "EmptyShape".to_string(),
            LayerType::Texture,
            shape,
            vec![],
            1,
        );

        assert_eq!(layer.tiles.len(), 0);
    }

    #[test]
    fn returns_none_for_missing_tile() {
        let layer = Layer::new(
            "Empty".to_string(),
            LayerType::Texture,
            Shape::from_square(2),
            vec![],
            1,
        );

        let out_of_bounds = SingleSelector { x: 10, y: 10 };
        assert_eq!(layer.get_tile_at(out_of_bounds), None);
    }

    #[test]
    fn is_blocking_returns_true_for_blocking_tile() {
        let mask = Mask {
            name: "Blocking".to_string(),
            selector: Selector::Single(SingleSelector { x: 0, y: 0 }),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let layer = Layer::new(
            "BlockingLayer".to_string(),
            LayerType::Action,
            Shape::from_square(1),
            vec![mask],
            1,
        );

        assert!(layer.is_blocking_at(&Coordinates { x: 0, y: 0 }));
        assert!(!layer.is_blocking_at(&Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn offset_adjusts_all_tiles_and_shape() {
        let mut layer = Layer::new(
            "Offset".to_string(),
            LayerType::Texture,
            Shape::from_square(2),
            vec![],
            1,
        );
        let original_tiles = layer.tiles.clone();

        layer.offset(Coordinates { x: 1, y: 1 });
        for (original, offset_tile) in original_tiles.iter().zip(layer.tiles.iter()) {
            assert_eq!(
                offset_tile.pointer,
                Coordinates {
                    x: original.pointer.x + 1,
                    y: original.pointer.y + 1
                }
            );
        }
        assert_eq!(layer.shape.width, 3);
        assert_eq!(layer.shape.height, 3);
    }

    #[test]
    fn base_layer_combines_and_applies_textures() {
        let base_layer = Layer::base(vec![
            Layer::new(
                "Layer1".to_string(),
                LayerType::Texture,
                Shape::from_square(2),
                vec![Mask {
                    name: "BlockMask".to_string(),
                    selector: Selector::Single(SingleSelector { x: 0, y: 0 }),
                    effect: Effect {
                        block: true,
                        ..Default::default()
                    },
                }],
                2,
            ),
            Layer::new(
                "Layer2".to_string(),
                LayerType::Texture,
                Shape::from_square(2),
                vec![Mask {
                    name: "ActionMask".to_string(),
                    selector: Selector::Single(SingleSelector { x: 1, y: 1 }),
                    effect: Effect {
                        action_id: Some(5),
                        ..Default::default()
                    },
                }],
                3,
            ),
        ]);

        assert_eq!(base_layer.kind, LayerType::Base);
        assert_eq!(base_layer.shape.width, 2);
        assert_eq!(base_layer.shape.height, 2);

        // Base layer tiles are 4 total for 2x2 shape
        assert_eq!(base_layer.tiles.len(), 4);

        // Check blocking effect applied from texture layer
        let tile_0_0 = base_layer.get_tile_at(Coordinates { x: 0, y: 0 }).unwrap();
        assert!(tile_0_0.effect.block);

        // Check action effect applied from other texture layer
        let tile_1_1 = base_layer.get_tile_at(Coordinates { x: 1, y: 1 }).unwrap();
        assert_eq!(tile_1_1.effect.action_id, Some(5));
    }

    #[test]
    fn reshape_discards_tiles_outside_shape() {
        let mut layer = Layer::new(
            "Reshape".to_string(),
            LayerType::Texture,
            Shape::from_square(3),
            vec![Mask {
                name: "BlockMask".to_string(),
                selector: Selector::Single(SingleSelector { x: 2, y: 2 }),
                effect: Effect {
                    block: true,
                    ..Default::default()
                },
            }],
            1,
        );

        layer.reshape(Shape {
            width: 2,
            height: 2,
        });

        // Tile at (2,2) should be discarded
        assert_eq!(layer.get_tile_at(Coordinates { x: 2, y: 2 }), None);
    }

    #[test]
    fn positive_reshape_expands_shape_and_regenerates_tiles() {
        let mut layer = Layer::new(
            "PositiveReshape".to_string(),
            LayerType::Texture,
            Shape::from_square(2),
            vec![],
            1,
        );

        let old_shape = layer.shape;
        layer.positive_reshape(Shape {
            width: 3,
            height: 4,
        });

        assert!(layer.shape.width >= old_shape.width);
        assert!(layer.shape.height >= old_shape.height);
        assert_eq!(layer.shape.width, 3);
        assert_eq!(layer.shape.height, 4);
    }
}
