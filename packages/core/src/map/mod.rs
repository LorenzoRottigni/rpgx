use crate::{
    prelude::{Coordinates, Delta, Direction, Effect, Layer, Rect, Tile},
    traits::{Bounded, Grid, Renderable, Spatial},
};
use indexmap::IndexMap;

pub mod effect;
pub mod layer;
pub mod mask;
pub mod routing;
pub mod tile;

/// Represents a game map with multiple layers, a name, and a spawn point.
#[derive(Clone)]
pub struct Map {
    /// Name identifier for the map
    pub name: String,
    /// Vector of layers stacked in this map
    pub layers: Vec<Layer>,
    /// Default spawn coordinates for pawns/players
    pub spawn: Coordinates,
    /// Offset applied at rendering time to all layers in the map.
    pub offset: Delta,
}

impl Renderable for Map {
    fn render(&self) -> Vec<Tile> {
        let mut layers = self.layers.clone();

        // Sort layers by z-index (ascending; adjust to descending if needed)
        layers.sort_by_key(|layer| layer.z);

        layers
            .into_iter()
            .flat_map(|mut layer| {
                layer.offset = layer.offset + self.offset;
                layer.render()
            })
            .collect()
    }
}

impl Bounded for Map {
    fn get_bounding_rect(&self) -> Rect {
        let rects: Vec<Rect> = self
            .layers
            .iter()
            .map(|layer| layer.get_bounding_rect())
            .collect();
        Rect::bounding_rect(&rects)
    }
}

impl Spatial for Map {
    fn contains(&self, target: &Coordinates) -> bool {
        self.layers.iter().any(|layer| layer.contains(target))
    }
}

impl Grid for Map {
    fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_effects_at(target))
            .collect()
    }

    fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_actions_at(target))
            .collect()
    }

    fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.layers.iter().any(|layer| layer.is_blocking_at(target))
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
            offset: Delta::default(),
        }
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
    /// If the new layer is larger than the current bounding rect, existing layers
    /// are offset by the necessary amount to make room.
    pub fn load_layer(&mut self, layer: Layer) {
        let current_shape = self.get_bounding_rect().shape;
        let target_shape = layer.get_bounding_rect().shape;

        // Compute delta with -1 to avoid excessive shifting
        let dx = (target_shape.width as i32) - (current_shape.width as i32) - 1;
        let dy = (target_shape.height as i32) - (current_shape.height as i32) - 1;

        let offset = Delta {
            dx: if dx > 0 { dx } else { 0 },
            dy: if dy > 0 { dy } else { 0 },
        };

        if offset.dx > 0 || offset.dy > 0 {
            for existing_layer in &mut self.layers {
                existing_layer.offset = existing_layer.offset + offset;
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
        for mut layer in other.layers.clone() {
            layer.offset = layer.offset + top_left.to_delta();
            self.layers.push(layer.clone());
            // let mut offset_layer = layer.clone();
            // offset_layer.offset(top_left.to_delta());
            // self.layers.push(offset_layer);
        }
        if let Some(new_spawn) = spawn {
            self.spawn = new_spawn;
        }
    }

    /// Duplicates this map in the given direction by merging a copy adjacent to itself.
    ///
    /// Optionally updates the spawn coordinate.
    pub fn duplicate_to_the(&mut self, direction: Direction, spawn: Option<Coordinates>) {
        let bounding_rect = self.get_bounding_rect();
        let top_left = match direction {
            Direction::Up => Coordinates {
                x: bounding_rect.origin.x,
                y: bounding_rect
                    .origin
                    .y
                    .saturating_sub(bounding_rect.shape.height),
            },
            Direction::Down => Coordinates {
                x: bounding_rect.origin.x,
                y: bounding_rect.origin.y + bounding_rect.shape.height,
            },
            Direction::Left => Coordinates {
                x: bounding_rect
                    .origin
                    .x
                    .saturating_sub(bounding_rect.shape.width),
                y: bounding_rect.origin.y,
            },
            Direction::Right => Coordinates {
                x: bounding_rect.origin.x + bounding_rect.shape.width,
                y: bounding_rect.origin.y,
            },
        };
        self.merge_at(&self.clone(), top_left, spawn);
    }

    /// Returns true if movement onto the specified coordinate is allowed.
    ///
    /// A coordinate is allowed if at least one layer has a tile there,
    /// and no layer is blocking at that coordinate.
    pub fn move_allowed(&self, target: Coordinates) -> bool {
        self.layers.iter().any(|layer| layer.contains(&target))
            && self
                .layers
                .iter()
                .all(|layer| !layer.is_blocking_at(&target))
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
                    Effect {
                        block: Some(rect),
                        ..Default::default()
                    },
                )
            })
            .collect::<Vec<_>>();

        // Add a non-blocking tile at (0,0) if not blocked
        if !blocked.iter().any(|c| *c == Coordinates::new(0, 0)) {
            let rect = Rect::from_xywh(0, 0, 1, 1);
            masks.push(Mask::new(
                "non_blocking_0_0".into(),
                vec![rect],
                Effect::default(), // no blocking
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

    #[test]
    fn test_move_allowed() {
        let blocked = vec![Coordinates::new(1, 1)];
        let map = build_test_map(&blocked);

        assert!(map.move_allowed(Coordinates::new(0, 0)));
        assert!(!map.move_allowed(Coordinates::new(1, 1)));
    }

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
