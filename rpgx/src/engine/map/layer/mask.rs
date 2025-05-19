use crate::{
        common::coordinates::Coordinates,
        common::shape::Shape,
        engine::map::selector::Selector,
        prelude::{Effect, Tile},
};

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
                        effect: self.effect.clone(),
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
                        effect: self.effect.clone(),
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
                                effect: self.effect.clone(),
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
                    effect: self.effect.clone(),
                })
                .collect(),
        }
    }
}
