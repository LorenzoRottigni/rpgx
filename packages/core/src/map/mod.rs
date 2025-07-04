use crate::{
    prelude::{Coordinates, Delta, Direction, Layer, Shape},
    traits::{Grid, Shaped, Shiftable},
};
use indexmap::IndexMap;

pub mod effect;
pub mod layer;
pub mod mask;
pub mod routing;

#[doc = include_str!("../../docs/map.md")]
/// Represents a game map with multiple layers, a name, and a spawn point.
#[derive(Clone)]
pub struct Map {
    /// Name identifier for the map
    pub name: String,
    /// Vector of layers stacked in this map
    pub layers: Vec<Layer>,
    /// Default spawn coordinates for pawns/players
    pub spawn: Coordinates,
}

impl Shaped for Map {
    /// Returns the bounding shape covering all layers in the map.
    fn get_shape(&self) -> Shape {
        let shapes: Vec<Shape> = self.layers.iter().map(|l| l.get_shape()).collect();
        Shape::bounding_shape(&shapes)
    }
}

impl Grid for Map {
    /// Checks if the map contains a tile at the specified coordinate.
    fn contains(&self, coord: &Coordinates) -> bool {
        self.layers.iter().any(|layer| layer.contains(coord))
    }
}

impl Map {
    /// Creates a new map with the given name, layers, and spawn location.
    ///
    /// # Notes
    /// The layers vector can be empty or contain any number of layers.
    pub fn new(name: String, layers: Vec<Layer>, spawn: Coordinates) -> Self {
        Self {
            name,
            layers,
            spawn,
        }
    }

    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.layers.iter().any(|layer| layer.is_blocking_at(target))
    }

    pub fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_actions_at(target))
            .collect()
    }

    /// Composes a new map by merging multiple maps at specified top-left offsets,
    /// adding additional layers, and setting spawn location.
    ///
    /// Each map in `maps` is merged at its specified `Coordinates`.
    /// Any additional `layers` are added on top.
    ///
    /// The `spawn` is the spawn point for the composed map.
    pub fn compose(
        name: String,
        maps: Vec<(Map, Coordinates)>,
        layers: Vec<Layer>,
        spawn: Coordinates,
    ) -> Self {
        let mut map = Map::new(name, layers, spawn);
        for (other_map, top_left) in maps.iter() {
            map.merge_at(other_map, *top_left, None);
        }
        map
    }

    /// Loads a new layer into the map, offsetting existing layers if needed to fit.
    ///
    /// If the new layer is larger than the current bounding shape, existing layers
    /// are offset by the necessary amount to make room.
    pub fn load_layer(&mut self, layer: Layer /* , offset: Coordinates */) {
        let current_shape = self.get_shape();
        let target_shape = layer.get_shape();

        let dx = (target_shape.width as i32) - (current_shape.width as i32) - 1;
        let dy = (target_shape.height as i32) - (current_shape.height as i32) - 1;

        let offset = Delta {
            dx: if dx > 0 { dx } else { 0 },
            dy: if dy > 0 { dy } else { 0 },
        };

        if offset.dx > 0 || offset.dy > 0 {
            for existing_layer in &mut self.layers {
                existing_layer.offset(offset);
            }
        }

        self.layers.push(layer);
    }

    /// Returns a map from layer name to the corresponding `Layer`.
    pub fn layers_by_name(&self) -> IndexMap<String, Layer> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.clone()))
            .collect()
    }

    /// Merges another map into this one at the specified top-left coordinate.
    ///
    /// Layers from `other` are offset by `top_left` and appended.
    /// Optionally updates the spawn coordinate.
    pub fn merge_at(&mut self, other: &Map, top_left: Coordinates, spawn: Option<Coordinates>) {
        for layer in &other.layers {
            let mut offset_layer = layer.clone();
            offset_layer.offset(top_left.to_delta());
            self.layers.push(offset_layer);
        }
        if let Some(new_spawn) = spawn {
            self.spawn = new_spawn;
        }
    }

    /// Duplicates this map in the given direction by merging a copy adjacent to itself.
    ///
    /// Optionally updates the spawn coordinate.
    pub fn duplicate_to_the(&mut self, direction: Direction, spawn: Option<Coordinates>) {
        let shape = self.get_shape();
        let top_left = match direction {
            Direction::Up | Direction::Down => Coordinates {
                x: 0,
                y: shape.height,
            },
            Direction::Left | Direction::Right => Coordinates {
                x: shape.width,
                y: 0,
            },
        };
        self.merge_at(&self.clone(), top_left, spawn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Effect, Layer, Mask, Rect};

    /// Build a simple map with a blocking layer at given coordinates.
    pub fn build_test_map(blocked: &[Coordinates]) -> Map {
        let mut masks = blocked
            .iter()
            .enumerate()
            .map(|(i, coord)| {
                let rect = Rect::from_xywh(coord.x, coord.y, 1, 1);
                Mask::new(
                    format!("block_{}", i),
                    vec![rect],
                    vec![Effect::Block(rect)],
                )
            })
            .collect::<Vec<_>>();

        // Add a non-blocking tile at (0,0) if not blocked
        if !blocked.iter().any(|c| *c == Coordinates::new(0, 0)) {
            let rect = Rect::from_xywh(0, 0, 1, 1);
            masks.push(Mask::new(
                "non_blocking_0_0".into(),
                vec![rect],
                vec![], // no blocking
            ));
        }

        let block_layer = Layer::new("blocking".into(), masks, 1);
        Map::new("test_map".into(), vec![block_layer], Coordinates::default())
    }

    #[test]
    fn test_map_new_and_shape() {
        let map = build_test_map(&[]);
        assert_eq!(map.name, "test_map");
    }

    // #[test]
    // fn test_move_allowed() {
    //     let blocked = vec![Coordinates::new(1, 1)];
    //     let map = build_test_map(&blocked);
    //
    //     assert!(map.move_allowed(Coordinates::new(0, 0))); // empty but tile missing? depends on layers
    //     assert!(!map.move_allowed(Coordinates::new(1, 1))); // blocked tile
    // }

    #[test]
    fn test_merge_and_layers_by_name() {
        let map1 = build_test_map(&[Coordinates::new(0, 0)]);
        let map2 = build_test_map(&[Coordinates::new(1, 1)]);
        let mut map = map1.clone();

        map.merge_at(&map2, Coordinates::new(5, 5), None);
        let layers = map.layers_by_name();
        assert!(layers.contains_key("blocking"));
        assert_eq!(map.layers.len(), 2);
    }

    #[test]
    fn test_duplicate_to_the() {
        let map = build_test_map(&[]);
        let mut dup = map.clone();
        dup.duplicate_to_the(Direction::Right, None);

        assert_eq!(dup.layers.len(), map.layers.len() * 2);
    }
}
