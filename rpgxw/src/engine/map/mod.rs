use js_sys::Array;
use wasm_bindgen::prelude::*;

pub mod selector;
pub mod tile;
pub mod effect;
pub mod layer;

use crate::prelude::{WasmCoordinates, WasmLayer};

/// Represents a map in the RPGX engine, which can contain multiple layers.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmMap {
    name: String,
    layers: Vec<WasmLayer>,
}

impl WasmMap {
    /// Creates a new `WasmMap` instance from a native RPGX map.
    pub fn from_native(map: rpgx::prelude::Map) -> Self {
        Self {
            name: map.name,
            layers: map.layers.into_iter().map(WasmLayer::from_native).collect(),
        }
    }

    /// Converts the `WasmMap` instance to a native RPGX map.
    pub fn to_native(&self) -> rpgx::prelude::Map {
        rpgx::prelude::Map {
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_native()).collect(),
        }
    }

    /// Creates a new `WasmMap` instance from a JavaScript object.
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

    /// Converts the `WasmMap` instance to a JavaScript object.
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
        let layers_array = layers
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Layers must be an Array"))?;
        let mut layers_vec = Vec::with_capacity(layers_array.length() as usize);
        for i in 0..layers_array.length() {
            let layer_js = layers_array.get(i);
            let layer = WasmLayer::from_js_value(&layer_js)?;
            layers_vec.push(layer);
        }
        Ok(Self {
            name,
            layers: layers_vec,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn layers(&self) -> Vec<WasmLayer> {
        self.layers.clone()
    }

    /// Gets the actions at a specific pointer in the map.
    #[wasm_bindgen]
    pub fn get_actions_at(&self, pointer: WasmCoordinates) -> Vec<JsValue> {
        let mut actions_ids = Vec::new();
        for layer in &self.layers {
            if let Some(tile) = layer.get_tile(pointer) {
                if let Some(action) = tile.effect().action_id() {
                    actions_ids.push(action);
                }
            }
        }
        actions_ids
            .into_iter()
            .map(|id| JsValue::from_f64(id as f64))
            .collect()
    }

}

