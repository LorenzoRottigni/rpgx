use js_sys::{Array, Reflect};
use wasm_bindgen::prelude::*;

use crate::{mask::Mask, shape::Shape};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum LayerType {
    Default,
    Texture,
    Block,
    Action,
}

impl LayerType {
    fn to_native(&self) -> rpgx::prelude::LayerType {
        match self {
            LayerType::Default => rpgx::prelude::LayerType::Default,
            LayerType::Action => rpgx::prelude::LayerType::Action,
            LayerType::Texture => rpgx::prelude::LayerType::Texture,
            LayerType::Block => rpgx::prelude::LayerType::Block,
        }
    }
}

// Layer
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layer {
    name: String,
    kind: LayerType,
    shape: Shape,
    masks: Vec<Mask>,
}

#[wasm_bindgen]
impl Layer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        kind: LayerType,
        shape: Shape,
        masks: &JsValue,
    ) -> Result<Layer, JsValue> {
        let masks_array = masks
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Masks must be an Array"))?;
        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = Mask::from_js_value(&mask_js)?;
            masks_vec.push(mask);
        }
        Ok(Layer {
            name,
            kind,
            shape,
            masks: masks_vec,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> LayerType {
        self.kind
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> Vec<Mask> {
        self.masks.clone()
    }
}

impl Layer {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Layer.name must be a string"))?;

        let kind_js = Reflect::get(value, &JsValue::from_str("kind"))?;
        // For enums, you might pass numbers from JS or strings. Here, assume integer variant:
        let kind_num = kind_js
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Layer.kind must be a number"))?;
        let kind = match kind_num as u32 {
            0 => LayerType::Default,
            1 => LayerType::Texture,
            2 => LayerType::Block,
            3 => LayerType::Action,
            _ => return Err(JsValue::from_str("Invalid LayerType value")),
        };

        let shape_js = Reflect::get(value, &JsValue::from_str("shape"))?;
        let width = Reflect::get(&shape_js, &JsValue::from_str("width"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.width must be a number"))?
            as i32;
        let height = Reflect::get(&shape_js, &JsValue::from_str("height"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.height must be a number"))?
            as i32;
        let shape = Shape::new(width, height);

        let masks_js = Reflect::get(value, &JsValue::from_str("masks"))?;
        let masks_array = masks_js
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Layer.masks must be an Array"))?;
        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = Mask::from_js_value(&mask_js)?;
            masks_vec.push(mask);
        }

        Ok(Layer {
            name,
            kind,
            shape,
            masks: masks_vec,
        })
    }
}

impl Layer {
    pub fn to_native(&self) -> rpgx::prelude::Layer {
        rpgx::prelude::Layer::new(
            self.name.clone(),
            self.kind.to_native(),
            self.shape.to_native(),
            self.masks.iter().map(|m| m.to_native()).collect(),
        )
    }
}
