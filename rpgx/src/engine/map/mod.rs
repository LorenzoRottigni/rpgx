pub mod effect;
pub mod layer;
pub mod routing;
pub mod selector;
pub mod tile;

use crate::prelude::{Coordinates,Tile};

use indexmap::IndexMap;
use layer::{Layer, LayerType};


#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub layers: Vec<Layer>
}

impl Map {
    pub fn new(name: String, layers: Vec<Layer>) -> Self {
        Self {
            name,
            layers
        }
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
