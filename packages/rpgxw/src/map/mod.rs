use js_sys::Array;
use wasm_bindgen::prelude::*;

pub mod tile;
pub mod effect;
pub mod layer;
pub mod selector;

use crate::prelude::{WasmCoordinates, WasmLayer, WasmEffect, WasmTile};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmMap {
    name: String,
    layers: Vec<WasmLayer>,
}

impl WasmMap {
    pub fn from_native(map: rpgx::prelude::Map) -> Self {
        Self {
            name: map.name,
            layers: map.layers.into_iter().map(WasmLayer::from_native).collect(),
        }
    }

    pub fn to_native(&self) -> rpgx::prelude::Map {
        rpgx::prelude::Map {
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_native()).collect(),
        }
    }

    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = js_sys::Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Map.name must be a string"))?;
        let layers_value = js_sys::Reflect::get(value, &JsValue::from_str("layers"))?;
        let layers_js = layers_value
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Map.layers must be an Array"))?;
        let mut layers_vec = Vec::with_capacity(layers_js.length() as usize);
        for i in 0..layers_js.length() {
            let layer_js = layers_js.get(i);
            let layer = WasmLayer::from_js_value(&layer_js)?;
            layers_vec.push(layer);
        }
        Ok(Self { name, layers: layers_vec })
    }

    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from(self.name.clone())).unwrap();

        let layers_array = Array::new();
        for layer in &self.layers {
            layers_array.push(&layer.to_js_value());
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("layers"), &layers_array).unwrap();

        obj.into()
    }
}

#[wasm_bindgen]
impl WasmMap {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, layers: &JsValue) -> Result<Self, JsValue> {
        // Convert JsValue to Vec<WasmLayer>
        let layers_array = layers
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Layers must be an Array"))?;

        let mut wasm_layers = Vec::with_capacity(layers_array.length() as usize);
        for i in 0..layers_array.length() {
            let layer_js = layers_array.get(i);
            let layer = WasmLayer::from_js_value(&layer_js)?;
            wasm_layers.push(layer);
        }

        // Convert to native layers
        let native_layers = wasm_layers.iter().map(|l| l.to_native()).collect();

        // Use native logic (inserts base layer, validates, etc.)
        let native_map = rpgx::prelude::Map::new(name, native_layers);

        // Convert back to WasmMap (which ensures full consistency)
        Ok(Self::from_native(native_map))
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn layers(&self) -> Vec<WasmLayer> {
        self.layers.clone()
    }

    /// Merge another WasmMap into this one at a given top-left coordinate.
    #[wasm_bindgen]
    pub fn merge_at(&mut self, other: &WasmMap, top_left: WasmCoordinates) {
        let mut native_self = self.to_native();
        let native_other = other.to_native();
        native_self.merge_at(&native_other, top_left.to_native());
        // Update self from native after merging
        *self = WasmMap::from_native(native_self);
    }

    /// Load a new layer into the map, with reshaping base layer if needed.
    #[wasm_bindgen]
    pub fn load_layer(&mut self, layer: WasmLayer) {
        let mut native_map = self.to_native();
        native_map.load_layer(layer.to_native());
        *self = WasmMap::from_native(native_map);
    }

    /// Duplicate the map to the specified direction.
    #[wasm_bindgen]
    pub fn duplicate_to_the(&mut self, direction: i32) {
        // Assuming direction is passed as an integer matching your Direction enum
        let dir = match direction {
            0 => rpgx::prelude::Direction::Up,
            1 => rpgx::prelude::Direction::Down,
            2 => rpgx::prelude::Direction::Left,
            3 => rpgx::prelude::Direction::Right,
            _ => rpgx::prelude::Direction::Right, // default fallback
        };
        let mut native_map = self.to_native();
        native_map.duplicate_to_the(dir);
        *self = WasmMap::from_native(native_map);
    }

    /// Check if any layer blocks the tile at `target`.
    #[wasm_bindgen]
    pub fn is_blocking_at(&self, target: WasmCoordinates) -> bool {
        let native_map = self.to_native();
        native_map.is_blocking_at(target.to_native())
    }

    /// Get the base layer if any.
    #[wasm_bindgen]
    pub fn get_base_layer(&self) -> Option<WasmLayer> {
        self.to_native().get_base_layer().map(WasmLayer::from_native)
    }

    /// Get the tile at pointer in the base layer.
    #[wasm_bindgen]
    pub fn get_base_tile_at(&self, pointer: WasmCoordinates) -> Option<WasmTile> {
        self.to_native()
            .get_base_layer()
            .and_then(|layer| layer.get_tile_at(pointer.to_native()))
            .map(WasmTile::from_native)
    }

    /// Get stacked tiles from all layers at pointer.
    #[wasm_bindgen]
    pub fn get_tiles_at(&self, pointer: WasmCoordinates) -> Vec<WasmTile> {
        let native_map = self.to_native();
        native_map
            .layers
            .iter()
            .filter_map(|layer| layer.get_tile_at(pointer.to_native()))
            .map(WasmTile::from_native)
            .collect()
    }

    /// Get all effects at pointer across all layers.
    #[wasm_bindgen]
    pub fn get_effects_at(&self, pointer: WasmCoordinates) -> Vec<WasmEffect> {
        let native_map = self.to_native();
        native_map
            .layers
            .iter()
            .filter_map(|layer| layer.get_tile_at(pointer.to_native()))
            .map(|tile| WasmEffect::from_native(tile.effect))
            .collect()
    }

    /// Gets the actions at a specific pointer in the map.
    #[wasm_bindgen]
    pub fn get_actions_at(&self, pointer: WasmCoordinates) -> Vec<JsValue> {
        let native_map = self.to_native();
        let actions: Vec<i32> = native_map
            .get_layers_of_type(rpgx::prelude::LayerType::Action)
            .into_iter()
            .filter_map(|layer| {
                layer
                    .get_tile_at(pointer.to_native())
                    .and_then(|tile| tile.effect.action_id)
            })
            .collect();

        actions
            .into_iter()
            .map(|id| JsValue::from_f64(id as f64))
            .collect()
    }
}
