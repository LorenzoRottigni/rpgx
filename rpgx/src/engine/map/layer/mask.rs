use crate::prelude::{Coordinates, Shape, Selector, Effect, Tile};

/// A [`Mask`] defines a logical area on a [`super::grid::Grid`] or [`super::layer::Layer`] where specific [`Effect`]s are applied
/// based on a [`Selector`] pattern.
///
/// [`Mask`]s allow grouping or marking of [`super::tile::Tile`]s that match a certain spatial or logical pattern, enabling
/// batch application of effects such as collision blocks, visual overlays, triggers, or behaviors.
/// They are commonly used during map construction or dynamic modification to efficiently define areas
/// of interaction or decoration.
#[derive(Clone)]
pub struct Mask {
    pub name: String,
    pub selector: Selector,
    pub effect: Effect,
}

impl Mask {
    pub fn apply(&self, shape: Shape) -> Vec<Tile> {
        match self.selector {
            Selector::Single(pointer) => {
                if shape.in_bounds(pointer) {
                    vec![Tile {
                        id: 0,
                        pointer,
                        shape: Shape::from_square(1),
                        effect: self.effect,
                    }]
                } else {
                    vec![]
                }
            }

            Selector::Block((start, end)) => {
                let tiles: Vec<Tile> = shape
                    .coordinates_in_range(start, end)
                    .into_iter()
                    .map(|coord| Tile {
                        id: 0,
                        pointer: coord,
                        shape: Shape::from_square(1),
                        effect: self.effect,
                    })
                    .collect();

                if tiles.iter().all(|t| t.effect.group) {
                    if let Some(first_tile) = tiles.first() {
                        if let Some((top_left, bottom_right)) = Coordinates::bounding_box(
                            &tiles.iter().map(|t| t.pointer).collect::<Vec<_>>(),
                        ) {
                            return vec![Tile {
                                id: first_tile.id,
                                pointer: top_left,
                                shape: Shape::from_bounds(top_left, bottom_right),
                                effect: self.effect,
                            }];
                        }
                    }
                }

                tiles
            }

            Selector::Filter(filter_fn) => shape
                .filter_coordinates(filter_fn)
                .into_iter()
                .map(|coord| Tile {
                    id: 0,
                    pointer: coord,
                    shape: Shape::from_square(1),
                    effect: self.effect,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{SingleSelector};

    #[test]
    fn applies_effect_to_single_tile() {
        let mask = Mask {
            name: "TestMask".to_string(),
            selector: Selector::Single(SingleSelector { x: 1, y: 1 }),
            effect: Effect {
                action_id: Some(1),
                texture_id: None,
                block: false,
                group: false,
                shrink: None,
            },
        };

        let shape = Shape::from_square(3);
        let tiles = mask.apply(shape);

        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].pointer, SingleSelector { x: 1, y: 1 });
        assert_eq!(tiles[0].effect.action_id, Some(1));
    }

    #[test]
    fn applies_effect_to_block_of_tiles() {
        let mask = Mask {
            name: "TestMask".to_string(),
            selector: Selector::Block((SingleSelector { x: 1, y: 1 }, SingleSelector { x: 3, y: 3 })),
            effect: Effect {
                action_id: None,
                texture_id: Some(2),
                block: true,
                group: false,
                shrink: None,
            },
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        assert_eq!(tiles.len(), 9);
        for tile in &tiles {
            assert_eq!(tile.effect.texture_id, Some(2));
        }
    }

    #[test]
    fn does_not_apply_out_of_bounds_single_tile() {
        let mask = Mask {
            name: "OutOfBounds".to_string(),
            selector: Selector::Single(SingleSelector { x: 10, y: 10 }),
            effect: Effect::default(),
        };

        let shape = Shape::from_square(3);
        let tiles = mask.apply(shape);

        assert!(tiles.is_empty());
    }

    #[test]
    fn groups_block_tiles_into_single_tile_when_grouped() {
        let mask = Mask {
            name: "GroupedBlock".to_string(),
            selector: Selector::Block((SingleSelector { x: 1, y: 1 }, SingleSelector { x: 2, y: 2 })),
            effect: Effect {
                group: true,
                ..Default::default()
            },
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].pointer, SingleSelector { x: 1, y: 1 });
        assert_eq!(tiles[0].shape.width, 2);  // 1 to 2 → width 2
        assert_eq!(tiles[0].shape.height, 2); // 1 to 2 → height 2
    }

    #[test]
    fn applies_effect_using_filter_selector() {
        let mask = Mask {
            name: "FilterMask".to_string(),
            selector: Selector::Filter(|coord| coord.x % 2 == 0 && coord.y % 2 == 0),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        let shape = Shape::from_bounds(SingleSelector { x: 0, y: 0 }, SingleSelector { x: 3, y: 3 });
        let tiles = mask.apply(shape);

        assert!(tiles.iter().all(|tile| tile.pointer.x % 2 == 0 && tile.pointer.y % 2 == 0));
        assert!(tiles.iter().all(|tile| tile.effect.block));
    }

    #[test]
    fn handles_empty_block_range() {
        let mask = Mask {
            name: "InvalidBlock".to_string(),
            selector: Selector::Block((SingleSelector { x: 3, y: 3 }, SingleSelector { x: 1, y: 1 })),
            effect: Effect::default(),
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        assert!(tiles.is_empty());
    }
}