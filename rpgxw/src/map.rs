use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::{coordinates::Coordinates, layer::Layer};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Map {
    name: String,
    layers: Vec<Layer>,
}

#[wasm_bindgen]
impl Map {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, layers: &JsValue) -> Result<Map, JsValue> {
        let layers_array = layers
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Layers must be an Array"))?;
        let mut layers_vec = Vec::with_capacity(layers_array.length() as usize);
        for i in 0..layers_array.length() {
            let layer_js = layers_array.get(i);
            let layer = Layer::from_js_value(&layer_js)?;
            layers_vec.push(layer);
        }
        Ok(Map {
            name,
            layers: layers_vec,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn layers(&self) -> Vec<Layer> {
        self.layers.clone()
    }

    #[wasm_bindgen]
    pub fn get_actions_at(&self, pointer: Coordinates) -> Vec<JsValue> {
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

impl Map {
    pub fn from_native(map: rpgx::prelude::Map) -> Self {
        Map {
            name: map.name,
            layers: map.layers.into_iter().map(Layer::from_native).collect(),
        }
    }
}

impl Map {
    pub fn to_native(&self) -> rpgx::prelude::Map {
        rpgx::prelude::Map {
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_native()).collect(),
        }
    }
}
