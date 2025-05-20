use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::layer::Layer;

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
}

impl Map {
    pub fn to_native(&self) -> rpgx::prelude::Map {
        rpgx::prelude::Map {
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_native()).collect(),
        }
    }
}
