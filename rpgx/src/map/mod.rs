pub mod effect;
pub mod layer;
pub mod routing;
pub mod selector;
pub mod tile;

use crate::prelude::{Coordinates, Layer, LayerType, Tile};

use indexmap::IndexMap;

#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn new(name: String, layers: Vec<Layer>) -> Self {
        Self { name, layers }
    }

    /// Add another map's layers, offsetting them into this map's grid layout
    pub fn expand_at(&mut self, other: &Map, top_left: Coordinates) {
        let mut layers_by_name: IndexMap<String, Layer> = self
            .layers
            .clone()
            .into_iter()
            .map(|layer| (layer.name.clone(), layer))
            .collect();

        for layer in &other.layers {
            let mut offset_layer = layer.clone().offset_tiles(top_left);

            // 🔁 Offset shrink values
            for tile in &mut offset_layer.tiles {
                if let Some((start, end)) = tile.effect.shrink {
                    tile.effect.shrink = Some((start + top_left, end + top_left));
                }
            }

            layers_by_name
                .entry(layer.name.clone())
                .and_modify(|existing| {
                    existing.tiles.extend(offset_layer.tiles.clone());
                    existing.shape.expand_to_include(top_left, layer.shape);
                })
                .or_insert(offset_layer);
        }

        self.layers = layers_by_name.into_values().collect();
    }

    /// Determine if a [`Tile`] is blocked in any layer
    pub fn is_tile_blocked(&self, target: Coordinates) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.is_tile_blocked(&target))
    }

    /// Get the first base layer (Default type)
    pub fn get_base_layer(&self) -> Option<Layer> {
        self.layers
            .iter()
            .find(|layer| layer.kind == LayerType::Default)
            .cloned()
    }

    /// Get all base layers
    pub fn get_base_layers(&self) -> Vec<Layer> {
        self.layers
            .iter()
            .filter(|layer| layer.kind == LayerType::Default)
            .cloned()
            .collect()
    }

    /// Retrieve a [`Tile`] from the base layer using a coordinate
    pub fn get_base_tile(&self, pointer: Coordinates) -> Option<Tile> {
        self.get_base_layer()?.get_tile(pointer)
    }

    pub fn get_actions_at(&self, pointer: Coordinates) -> Vec<i32> {
        let mut actions = vec![];

        for layer in &self.layers {
            if let Some(tile) = layer.get_tile(pointer) {
                if let Some(action) = tile.effect.action_id {
                    actions.push(action);
                }
            }
        }

        actions
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{Effect, Shape};

    fn dummy_tile(x: i32, y: i32) -> Tile {
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
        }
    }

    #[test]
    fn creates_map_with_layers() {
        let tile = dummy_tile(0, 0);
        let layer = dummy_layer(
            "base",
            LayerType::Default,
            vec![tile],
            Shape::from_square(1),
        );
        let map = Map::new("TestMap".to_string(), vec![layer.clone()]);

        assert_eq!(map.name, "TestMap");
        assert_eq!(map.layers.len(), 1);
        assert_eq!(map.get_base_layer().unwrap().name, "base");
    }

    #[test]
    fn gets_tile_from_base_layer() {
        let tile = dummy_tile(1, 2);
        let layer = dummy_layer(
            "base",
            LayerType::Default,
            vec![tile.clone()],
            Shape::from_square(3),
        );
        let map = Map::new("TileMap".to_string(), vec![layer]);

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
        let map = Map::new("BlockMap".to_string(), vec![blocking_layer]);

        assert!(map.is_tile_blocked(Coordinates { x: 0, y: 0 }));
        assert!(!map.is_tile_blocked(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn retrieves_all_base_layers() {
        let tile = dummy_tile(0, 0);
        let base_layer = dummy_layer(
            "base1",
            LayerType::Default,
            vec![tile.clone()],
            Shape::from_square(1),
        );
        let other_layer = dummy_layer(
            "logic",
            LayerType::Action,
            vec![tile],
            Shape::from_square(1),
        );
        let map = Map::new(
            "LayerMap".to_string(),
            vec![base_layer.clone(), other_layer],
        );

        let bases = map.get_base_layers();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].name, "base1");
    }

    #[test]
    fn expands_map_with_offset_layer() {
        let tile = dummy_tile(0, 0);
        let shape = Shape::from_square(1);
        let mut base_map = Map::new(
            "Base".to_string(),
            vec![dummy_layer(
                "base",
                LayerType::Default,
                vec![tile.clone()],
                shape,
            )],
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
        let offset_layer = dummy_layer("base", LayerType::Default, vec![offset_tile], shape);

        let overlay_map = Map::new("Overlay".to_string(), vec![offset_layer]);

        base_map.expand_at(&overlay_map, Coordinates { x: 2, y: 3 });

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
        let map = Map::new("ActionMap".to_string(), vec![action_layer]);

        let actions = map.get_actions_at(Coordinates { x: 1, y: 1 });
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], 99);
    }
}
