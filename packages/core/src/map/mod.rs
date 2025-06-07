use crate::{
    pawn::Pawn,
    prelude::{Coordinates, Direction, Layer, LayerType, Tile},
};
use indexmap::IndexMap;
use layer::{Effect, Shape, SingleSelector};

pub mod effect;
pub mod layer;
pub mod routing;
pub mod selector;
pub mod tile;

/// Game map containing multiple layers.
#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub layers: Vec<Layer>,
    pub spawn: Coordinates,
}

impl Map {
    /// Creates a new map, adding a base layer if none exists.
    pub fn new(name: String, mut layers: Vec<Layer>, spawn: Coordinates) -> Self {
        if !layers.iter().any(|layer| layer.kind == LayerType::Base) {
            layers.push(Layer::base(layers.clone()));
        }
        Self {
            name,
            layers,
            spawn,
        }
    }

    /// Composes a map from multiple maps and layers.
    pub fn compose(
        name: String,
        maps: Vec<(Map, SingleSelector)>,
        layers: Vec<Layer>,
        spawn: Coordinates,
    ) -> Self {
        let mut map = Map::new(name, layers, spawn);
        for (other_map, top_left) in maps.iter() {
            map.merge_at(other_map, *top_left, None);
        }
        map
    }
}

impl Map {
    /// Adds a layer, reshaping base if present or creating one if missing.
    pub fn load_layer(&mut self, layer: Layer /* , offset: Coordinates */) {
        if let Some(base_layer) = self.get_base_layer() {
            let offset = Coordinates {
                x: if layer.shape.width > base_layer.shape.width {
                    layer.shape.width - base_layer.shape.width - 1
                } else {
                    0
                },
                y: if layer.shape.height > base_layer.shape.height {
                    layer.shape.height - base_layer.shape.height - 1
                } else {
                    0
                },
            };
            // Offset the tiles in the incoming layer
            for existing_layer in &mut self.layers {
                existing_layer.offset(offset);
            }

            // Add the updated layer
            self.layers.push(layer);

            // Remove old base layer
            self.layers.retain(|l| l.kind != LayerType::Base);

            // Regenerate base layer from all non-base layers
            let base_layer = Layer::base(self.layers.clone());

            self.layers.push(base_layer);
        } else {
            self.layers.push(layer);
            let base_layer = Layer::base(self.layers.clone());
            self.layers.push(base_layer);
        }
    }

    /// Returns a map of layer name to layer.
    pub fn layers_by_name(&self) -> IndexMap<String, Layer> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.clone()))
            .collect()
    }

    /// Merges another map into this one at the specified top-left coordinates.
    pub fn merge_at(&mut self, other: &Map, top_left: Coordinates, spawn: Option<Coordinates>) {
        let mut layers_by_name = self.layers_by_name();

        for layer in &other.layers {
            let mut offset_layer = layer.clone();
            offset_layer.offset(top_left);

            layers_by_name
                .entry(layer.name.clone())
                .and_modify(|existing| {
                    existing.tiles.extend(&offset_layer.tiles);
                    existing.shape.expand_to_include(top_left, layer.shape);
                })
                .or_insert(offset_layer);
        }

        self.layers = layers_by_name.into_values().collect();

        self.spawn = spawn.unwrap_or(self.spawn);
    }

    /// Duplicates the map in the specified direction, expanding it.
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

    /// Returns `true` if any layer blocks the tile at `target`.
    pub fn is_blocking_at(&self, target: Coordinates) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.is_blocking_at(&target))
    }

    /// Returns the shape of the map, which is the shape of the base layer.
    pub fn get_shape(&self) -> Shape {
        if let Some(base_layer) = self.get_base_layer() {
            base_layer.shape
        } else {
            Shape::default()
        }
    }

    /// Returns the first base layer, if any.
    pub fn get_base_layer(&self) -> Option<Layer> {
        self.layers
            .iter()
            .find(|l| l.kind == LayerType::Base)
            .cloned()
    }

    /// Return map layers of the specified type.
    pub fn get_layers_of_type(&self, kind: LayerType) -> Vec<Layer> {
        self.layers
            .iter()
            .filter(|l| l.kind == kind)
            .cloned()
            .collect()
    }

    /// Returns the tile at `pointer` in the base layer, if present.
    pub fn get_base_tile(&self, pointer: Coordinates) -> Option<Tile> {
        self.get_base_layer()?.get_tile_at(pointer)
    }

    /// Return stacked tiles from all layers at the specified pointer.
    pub fn get_tiles_at(&self, pointer: Coordinates) -> Vec<Tile> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer))
            .collect()
    }

    /// Returns all effects present at `pointer` across all layers.
    pub fn get_effects_at(&self, pointer: Coordinates) -> Vec<Effect> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer).map(|tile| tile.effect))
            .collect()
    }

    /// Returns all action IDs present at `pointer` across action layers.
    pub fn get_actions_at(&self, pointer: Coordinates) -> Vec<u32> {
        self.get_layers_of_type(LayerType::Action)
            .into_iter()
            .flat_map(|layer| {
                layer
                    .get_tile_at(pointer)
                    .and_then(|tile| tile.effect.action_id)
            })
            .collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{Effect, Shape};

    fn dummy_tile(x: u32, y: u32) -> Tile {
        Tile {
            id: 1,
            pointer: Coordinates { x, y },
            shape: Shape::from_square(1),
            effect: Effect::default(),
        }
    }

    fn dummy_layer(name: &str, kind: LayerType, tiles: Vec<Tile>, shape: Shape) -> Layer {
        Layer {
            name: name.to_string(),
            kind,
            tiles,
            shape,
            masks: vec![],
            z: 1,
        }
    }

    #[test]
    fn creates_map_with_layers() {
        let tile = dummy_tile(0, 0);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(1));
        let map = Map::new(
            "TestMap".to_string(),
            vec![layer.clone()],
            Coordinates::default(),
        );

        assert_eq!(map.name, "TestMap");
        assert_eq!(map.layers.len(), 1);
        assert_eq!(map.get_base_layer().unwrap().name, "base");
    }

    #[test]
    fn gets_tile_from_base_layer() {
        let tile = dummy_tile(1, 2);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(3));
        let map = Map::new("TileMap".to_string(), vec![layer], Coordinates::default());

        let result = map.get_base_tile(Coordinates { x: 1, y: 2 });
        assert!(result.is_some());
        assert_eq!(result.unwrap().pointer, Coordinates { x: 1, y: 2 });
    }

    #[test]
    fn detects_blocked_tile_across_layers() {
        let blocked_tile = Tile {
            id: 2,
            pointer: Coordinates { x: 0, y: 0 },
            shape: Shape::from_square(1),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let blocking_layer = dummy_layer(
            "block",
            LayerType::Block,
            vec![blocked_tile],
            Shape::from_square(1),
        );
        let map = Map::new(
            "BlockMap".to_string(),
            vec![blocking_layer],
            Coordinates::default(),
        );

        assert!(map.is_blocking_at(Coordinates { x: 0, y: 0 }));
        assert!(!map.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn expands_map_with_offset_layer() {
        let tile = dummy_tile(0, 0);
        let shape = Shape::from_square(1);
        let mut base_map = Map::new(
            "Base".to_string(),
            vec![dummy_layer("base", LayerType::Base, vec![tile], shape)],
            Coordinates::default(),
        );

        let offset_tile = Tile {
            pointer: Coordinates { x: 0, y: 0 },
            id: 10,
            shape: Shape::from_square(1),
            effect: Effect {
                action_id: Some(42),
                ..Default::default()
            },
        };
        let offset_layer = dummy_layer("base", LayerType::Base, vec![offset_tile], shape);

        let overlay_map = Map::new(
            "Overlay".to_string(),
            vec![offset_layer],
            Coordinates::default(),
        );

        base_map.merge_at(&overlay_map, Coordinates { x: 2, y: 3 }, None);

        let tile = base_map.get_base_tile(Coordinates { x: 2, y: 3 });
        assert!(tile.is_some());
        assert_eq!(tile.unwrap().effect.action_id, Some(42));
    }

    #[test]
    fn gets_actions_at_position() {
        let tile = Tile {
            pointer: Coordinates { x: 1, y: 1 },
            id: 5,
            shape: Shape::from_square(1),
            effect: Effect {
                action_id: Some(99),
                ..Default::default()
            },
        };
        let action_layer = dummy_layer(
            "action",
            LayerType::Action,
            vec![tile],
            Shape::from_square(2),
        );
        let map = Map::new(
            "ActionMap".to_string(),
            vec![action_layer],
            Coordinates::default(),
        );

        let actions = map.get_actions_at(Coordinates { x: 1, y: 1 });
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], 99);
    }
}
