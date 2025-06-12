use crate::{
    map::grid::Grid,
    prelude::{Coordinates, Effect, Selector, Shape, Tile},
};

#[doc = include_str!("../../../docs/mask.md")]
/// A [`Mask`] defines a logical area on a [`super::grid::Grid`] or [`super::layer::Layer`] where specific [`Effect`]s are applied
/// based on a [`Selector`] pattern.
///
/// [`Mask`]s allow grouping or marking of [`super::tile::Tile`]s that match a certain spatial or logical pattern, enabling
/// batch application of effects such as collision blocks, visual overlays, triggers, or behaviors.
/// They are commonly used during map construction or dynamic modification to efficiently define areas
/// of interaction or decoration.
#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,

    /// The selector pattern defining which coordinates this mask applies to.
    pub selector: Selector,

    /// The effect to apply to all tiles covered by this mask.
    pub effect: Effect,

    pub grid: Grid,
}

impl Mask {
    /// Create a new mask with the given name, selector, and effect.
    pub fn new(name: String, selector: Selector, effect: Effect) -> Self {
        let shape = selector.get_shape();
        let tiles = match &selector {
            Selector::Single(pointer) => {
                // If the selector is a single coordinate and is inside the shape,
                // create one tile with the effect at that coordinate.
                if shape.in_bounds(*pointer) {
                    vec![Tile {
                        pointer: *pointer,
                        shape: Shape::from_square(1), // single tile shape
                        effect: effect,
                    }]
                } else {
                    vec![]
                }
            }

            Selector::Block((start, end)) => {
                // For a block selector, get all coordinates in the rectangular range
                // and create tiles for each coordinate.

                let tiles: Vec<Tile> = shape
                    .coordinates_in_range(*start, *end)
                    .into_iter()
                    .map(|coord| Tile {
                        pointer: coord,
                        shape: Shape::from_square(1),
                        effect: effect,
                    })
                    .collect();

                // Optimization: if all tiles have the 'group' flag in the effect,
                // merge them into a single tile with a bounding box shape covering all tiles.
                if tiles.iter().all(|t| t.effect.group) {
                    if let Some((top_left, bottom_right)) = Coordinates::bounding_box(
                        &tiles.iter().map(|t| t.pointer).collect::<Vec<_>>(),
                    ) {
                        return Self {
                            name,
                            selector: selector.clone(),
                            effect,
                            grid: Grid {
                                shape: selector.get_shape(),
                                tiles: vec![Tile {
                                    pointer: top_left,
                                    shape: Shape::from_bounds(top_left, bottom_right),
                                    effect,
                                }],
                            },
                        };
                    }
                }

                tiles
            }

            Selector::Sparse(pointers) => pointers
                .into_iter()
                .map(|pointer| Tile {
                    pointer: *pointer,
                    shape: Shape::from_square(1),
                    effect: effect,
                })
                .collect(),
            // Selector::Filter(filter_fn) => {
            //     // For a filter selector, filter coordinates inside the shape using the provided function
            //     // and create tiles for each matching coordinate.
            //
            //     shape
            //         .filter_coordinates(filter_fn)
            //         .into_iter()
            //         .map(|coord| Tile {
            //             pointer: coord,
            //             shape: Shape::from_square(1),
            //             effect: self.effect,
            //         })
            //         .collect()
            // }
        };
        let grid = Grid {
            shape: selector.get_shape(),
            tiles,
        };
        Self {
            name,
            selector,
            effect,
            grid,
        }
    }

    pub fn offset(&mut self, delta: Coordinates) {
        self.grid.offset(delta);
    }

    /*
    /// Apply this mask's effect to the tiles within the given shape bounds.
    ///
    /// Returns a vector of [`Tile`]s representing the effect applied to coordinates
    /// matching the mask's selector within the shape.
     pub fn apply(&self, _shape: Shape) -> Vec<Tile> {
        let shape = &self.selector.get_shape();
        match &self.selector {
            Selector::Single(pointer) => {
                // If the selector is a single coordinate and is inside the shape,
                // create one tile with the effect at that coordinate.
                if shape.in_bounds(*pointer) {
                    vec![Tile {
                        pointer: *pointer,
                        shape: Shape::from_square(1), // single tile shape
                        effect: self.effect,
                    }]
                } else {
                    vec![]
                }
            }

            Selector::Block((start, end)) => {
                // For a block selector, get all coordinates in the rectangular range
                // and create tiles for each coordinate.

                let tiles: Vec<Tile> = shape
                    .coordinates_in_range(*start, *end)
                    .into_iter()
                    .map(|coord| Tile {
                        pointer: coord,
                        shape: Shape::from_square(1),
                        effect: self.effect,
                    })
                    .collect();

                // Optimization: if all tiles have the 'group' flag in the effect,
                // merge them into a single tile with a bounding box shape covering all tiles.
                if tiles.iter().all(|t| t.effect.group) {
                    if let Some(first_tile) = tiles.first() {
                        if let Some((top_left, bottom_right)) = Coordinates::bounding_box(
                            &tiles.iter().map(|t| t.pointer).collect::<Vec<_>>(),
                        ) {
                            return vec![Tile {
                                pointer: top_left,
                                shape: Shape::from_bounds(top_left, bottom_right),
                                effect: self.effect,
                            }];
                        }
                    }
                }

                tiles
            }

            Selector::Sparse(pointers) => pointers
                .into_iter()
                .map(|pointer| Tile {
                    pointer: *pointer,
                    shape: Shape::from_square(1),
                    effect: self.effect,
                })
                .collect(),
            // Selector::Filter(filter_fn) => {
            //     // For a filter selector, filter coordinates inside the shape using the provided function
            //     // and create tiles for each matching coordinate.
            //
            //     shape
            //         .filter_coordinates(filter_fn)
            //         .into_iter()
            //         .map(|coord| Tile {
            //             pointer: coord,
            //             shape: Shape::from_square(1),
            //             effect: self.effect,
            //         })
            //         .collect()
            // }
        }
    } */
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::SingleSelector;

    #[test]
    fn applies_effect_to_single_tile() {
        let mask = Mask::new(
            "TestMask".to_string(),
            Selector::Single(SingleSelector { x: 1, y: 1 }),
            Effect {
                action_id: Some(1),
                ..Default::default()
            },
        );

        assert_eq!(mask.grid.tiles.len(), 1);
        assert_eq!(mask.grid.tiles[0].pointer, SingleSelector { x: 1, y: 1 });
        assert_eq!(mask.grid.tiles[0].effect.action_id, Some(1));
    }

    /* #[test]
    fn applies_effect_to_block_of_tiles() {
        let mask = Mask {
            name: "TestMask".to_string(),
            selector: Selector::Block((
                SingleSelector { x: 1, y: 1 },
                SingleSelector { x: 3, y: 3 },
            )),
            effect: Effect {
                texture_id: Some(2),
                block: true,
                ..Default::default()
            },
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        // 3x3 block = 9 tiles
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

        // Coordinate is outside shape bounds, so no tiles should be returned.
        assert!(tiles.is_empty());
    }

    #[test]
    fn groups_block_tiles_into_single_tile_when_grouped() {
        let mask = Mask {
            name: "GroupedBlock".to_string(),
            selector: Selector::Block((
                SingleSelector { x: 1, y: 1 },
                SingleSelector { x: 2, y: 2 },
            )),
            effect: Effect {
                group: true,
                ..Default::default()
            },
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        // Should merge 4 tiles into 1 tile with bounding box shape
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].pointer, SingleSelector { x: 1, y: 1 });
        assert_eq!(tiles[0].shape.width, 2);
        assert_eq!(tiles[0].shape.height, 2);
    }

    /* #[test]
    fn applies_effect_using_filter_selector() {
        let mask = Mask {
            name: "FilterMask".to_string(),
            selector: Selector::Filter(|coord, _s| coord.x % 2 == 0 && coord.y % 2 == 0),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };

        let shape =
            Shape::from_bounds(SingleSelector { x: 0, y: 0 }, SingleSelector { x: 3, y: 3 });
        let tiles = mask.apply(shape);

        // Only even x and y coordinates should be included
        assert!(
            tiles
                .iter()
                .all(|tile| tile.pointer.x % 2 == 0 && tile.pointer.y % 2 == 0)
        );
        assert!(tiles.iter().all(|tile| tile.effect.block));
    } */

    #[test]
    fn handles_empty_block_range() {
        let mask = Mask {
            name: "InvalidBlock".to_string(),
            selector: Selector::Block((
                SingleSelector { x: 3, y: 3 },
                SingleSelector { x: 1, y: 1 },
            )),
            effect: Effect::default(),
        };

        let shape = Shape::from_square(5);
        let tiles = mask.apply(shape);

        // Invalid range (start > end) should produce no tiles
        assert!(tiles.is_empty());
    } */
}
