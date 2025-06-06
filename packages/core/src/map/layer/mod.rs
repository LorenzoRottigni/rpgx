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
    pub z: i32, // Z-index for rendering order
}

impl Layer {
    /// Constructs a new non-base layer by applying masks to the given shape.
    ///
    /// Panics if called with [`LayerType::Base`] (use [`Layer::base`] instead).
    pub fn new(name: String, kind: LayerType, shape: Shape, masks: Vec<Mask>, z: i32) -> Self {
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
    /// merge any tiles from [`LayerType::Texture`] layers into itself.
    pub fn base(layers: Vec<Self>) -> Self {
        let mut base_shape = Shape::default();

        for layer in &layers {
            base_shape = base_shape.union(layer.shape);
        }

        // Create a uniform grid of tiles across the entire shape
        let mut tiles = Vec::new();
        for y in 0..base_shape.height {
            for x in 0..base_shape.width {
                tiles.push(Tile {
                    id: x,
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

        let mut layers_by_z = layers.clone();
        layers_by_z.sort_by_key(|layer| layer.z);

        for layer in &layers_by_z {
            base_layer.positive_reshape(layer.shape);

            if layer.kind == LayerType::Texture {
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
        let mut tiles = vec![];
        for tile in self.tiles.iter() {
            if shape.in_bounds(tile.pointer) {
                tiles.push(*tile)
            }
        }
        self.tiles = tiles;

        if self.kind == LayerType::Base {
            let mut base_tiles = Vec::new();
            for y in 0..self.shape.height {
                for x in 0..self.shape.width {
                    let pointer = Coordinates { x, y };
                    let effect = if let Some(tile) = self.get_tile_at(pointer) {
                        tile.effect
                    } else {
                        Effect::default()
                    };
                    base_tiles.push(Tile {
                        id: x,
                        pointer: Coordinates { x, y },
                        shape: Shape::from_square(1),
                        effect,
                    });
                }
            }
            self.tiles = base_tiles;
        }
    }

    /// Expands the shape to include the provided shape (only grows, never shrinks).
    pub fn positive_reshape(&mut self, shape: Shape) {
        if shape.width > self.shape.width {
            self.shape.width = shape.width;
        }
        if shape.height > self.shape.height {
            self.shape.height = shape.height;
        }
        self.reshape(self.shape);
    }

    /// Finds the tile covering the given coordinates, accounting for both tile origin and shape.
    pub fn get_tile_at(&self, pointer: Coordinates) -> Option<Tile> {
        self.tiles
            .iter()
            .find(|tile| {
                let local = Coordinates {
                    x: pointer.x - tile.pointer.x,
                    y: pointer.y - tile.pointer.y,
                };
                tile.shape.in_bounds(local)
            })
            .cloned()
    }

    /// Retrieves all tiles within a rectangular block defined by two coordinates.
    pub fn get_block_at(&self, pointer: BlockSelector) -> Vec<Tile> {
        self.shape
            .coordinates_in_range(pointer.0, pointer.1)
            .into_iter()
            .filter_map(|coord| self.tiles.iter().find(|t| t.pointer == coord).cloned())
            .collect()
    }

    /// Checks if the tile at a given coordinate is blocking.
    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.is_blocking_at(*target))
    }

    /// Offsets all tiles in this layer by the given delta.
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
    fn get_block_returns_only_existing_tiles() {
        let mask = Mask {
            name: "PartialBlock".to_string(),
            selector: Selector::Single(SingleSelector { x: 1, y: 1 }),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let layer = Layer::new(
            "TestLayer".to_string(),
            LayerType::Action,
            Shape::from_square(3),
            vec![mask],
            1,
        );

        let block =
            layer.get_block_at((SingleSelector { x: 0, y: 0 }, SingleSelector { x: 2, y: 2 }));
        assert_eq!(block.len(), 1);
        assert_eq!(block[0].pointer, SingleSelector { x: 1, y: 1 });
    }

    #[test]
    fn detects_blocked_tile() {
        let mask = Mask {
            name: "Blocked".to_string(),
            selector: Selector::Single(SingleSelector { x: 0, y: 0 }),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let layer = Layer::new(
            "BlockLayer".to_string(),
            LayerType::Block,
            Shape::from_square(2),
            vec![mask],
            1,
        );

        assert!(layer.is_blocking_at(&Coordinates { x: 0, y: 0 }));
        assert!(!layer.is_blocking_at(&Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn offsets_tiles_and_shape() {
        let mask = Mask {
            name: "OffsetMask".to_string(),
            selector: Selector::Single(SingleSelector { x: 0, y: 0 }),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let original_shape = Shape::from_square(2);
        let mut layer = Layer::new(
            "OffsetLayer".to_string(),
            LayerType::Action,
            original_shape,
            vec![mask],
            1,
        );
        let offset = Coordinates { x: 2, y: 3 };

        layer.offset(offset);

        assert_eq!(layer.tiles[0].pointer, Coordinates { x: 2, y: 3 });
        assert_eq!(layer.shape.width, original_shape.width + 2);
        assert_eq!(layer.shape.height, original_shape.height + 3);
    }
}
