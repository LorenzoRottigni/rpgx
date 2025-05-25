use mask::Mask;

pub use crate::prelude::{BlockSelector, Coordinates, Effect, Shape, SingleSelector, Tile};

pub mod mask;

#[derive(Clone, Copy, PartialEq)]
pub enum LayerType {
    Base,
    Action,
    Texture,
    Block,
}

/// A visual or logical overlay on top of the base [`Grid`], used to apply effects
/// on specific [`Tile`]s based on spatial [`Mask`] and [`Selector`].
/// [`Layer`]s simulate stacking behavior along the Z-axis and allow grouped or
/// conditional [`Tile`] modifications without altering the original [`Grid`].
#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub kind: LayerType,
    pub tiles: Vec<Tile>,
    pub shape: Shape,
    pub masks: Vec<Mask>,
}

impl Layer {
    pub fn new(name: String, kind: LayerType, shape: Shape, masks: Vec<Mask>) -> Self {
        match kind {
            LayerType::Base => Layer::base(name, shape, masks),
            _ => {
                let tiles = masks.iter().flat_map(|mask| mask.apply(shape)).collect();
                Self {
                    name,
                    kind,
                    shape,
                    tiles,
                    masks,
                }
            }
        }
    }

    pub fn base(name: impl Into<String>, shape: Shape, masks: Vec<Mask>) -> Self {
        let mut tiles = Vec::new();
        for y in 0..shape.height {
            for x in 0..shape.width {
                tiles.push(Tile {
                    id: x,
                    pointer: Coordinates { x, y },
                    shape: Shape::from_square(1),
                    effect: Effect::default(),
                });
            }
        }

        for mask in &masks {
            for tile in mask.apply(shape) {
                if let Some(t) = tiles.iter_mut().find(|t| t.pointer == tile.pointer) {
                    t.effect = tile.effect.clone(); // or merge if needed
                }
            }
        }

        Self {
            name: name.into(),
            kind: LayerType::Base,
            shape,
            tiles,
            masks,
        }
    }

    /// Retrieve a [`Tile`] from within the [`Grid`]
    pub fn get_tile(&self, pointer: SingleSelector) -> Option<Tile> {
        self.tiles
            .iter()
            .find(|tile| tile.pointer == pointer)
            .cloned()
    }

    /// Retrieve a block of [`Tile`]s from within the [`Grid`]
    pub fn get_block(&self, pointer: BlockSelector) -> Vec<Tile> {
        self.shape
            .coordinates_in_range(pointer.0, pointer.1)
            .into_iter()
            .filter_map(|coord| self.tiles.iter().find(|t| t.pointer == coord).cloned())
            .collect()
    }

    pub fn is_tile_blocked(&self, target: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.is_blocking_at(*target))
    }

    pub fn offset_tiles(mut self, delta: Coordinates) -> Self {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }
        self.shape.width += delta.x;
        self.shape.height += delta.y;
        self
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
        );

        assert_eq!(layer.tiles.len(), 2);
        assert!(
            layer
                .get_tile(SingleSelector { x: 0, y: 0 })
                .unwrap()
                .effect
                .block
        );
        assert_eq!(
            layer
                .get_tile(SingleSelector { x: 1, y: 0 })
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
        );

        let block = layer.get_block((SingleSelector { x: 0, y: 0 }, SingleSelector { x: 2, y: 2 }));
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
        let layer = Layer::new("EmptyShape".to_string(), LayerType::Base, shape, vec![]);

        assert_eq!(layer.tiles.len(), 0);
    }

    #[test]
    fn creates_default_layer_without_masks() {
        let shape = Shape::from_square(2);
        let layer = Layer::new("BaseLayer".to_string(), LayerType::Base, shape, vec![]);

        assert_eq!(layer.tiles.len(), 4); // 2x2
        for tile in &layer.tiles {
            assert_eq!(tile.effect, Effect::default());
        }
    }

    #[test]
    fn returns_none_for_missing_tile() {
        let layer = Layer::new(
            "Empty".to_string(),
            LayerType::Base,
            Shape::from_square(2),
            vec![],
        );

        let out_of_bounds = SingleSelector { x: 10, y: 10 };
        assert_eq!(layer.get_tile(out_of_bounds), None);
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
        );

        let block = layer.get_block((SingleSelector { x: 0, y: 0 }, SingleSelector { x: 2, y: 2 }));
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
        );

        assert!(layer.is_tile_blocked(&Coordinates { x: 0, y: 0 }));
        assert!(!layer.is_tile_blocked(&Coordinates { x: 1, y: 1 }));
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
        let layer = Layer::new(
            "OffsetLayer".to_string(),
            LayerType::Action,
            original_shape,
            vec![mask],
        );
        let offset = Coordinates { x: 2, y: 3 };

        let offset_layer = layer.offset_tiles(offset);

        assert_eq!(offset_layer.tiles[0].pointer, Coordinates { x: 2, y: 3 });
        assert_eq!(offset_layer.shape.width, original_shape.width + 2);
        assert_eq!(offset_layer.shape.height, original_shape.height + 3);
    }
}
