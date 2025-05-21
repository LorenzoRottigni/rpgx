use js_sys::{Array, Reflect};
use wasm_bindgen::prelude::*;

use crate::{coordinates::Coordinates, mask::Mask, shape::Shape, tile::Tile};

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
    tiles: Vec<Tile>,
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
            .ok_or_else(|| JsValue::from_str("masks must be an Array"))?;

        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask = Mask::from_js_value(&masks_array.get(i))?;
            masks_vec.push(mask);
        }

        let native_layer = rpgx::engine::map::layer::Layer::new(
            name.clone(),
            kind.to_native(),
            shape.to_native(),
            masks_vec.iter().map(|m| m.to_native()).collect(),
        );

        Ok(Layer::from_native(native_layer))
    }

    #[wasm_bindgen(getter)]
    pub fn tiles(&self) -> Vec<Tile> {
        self.tiles.clone()
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

    #[wasm_bindgen]
    pub fn get_tile(&self, pointer: Coordinates) -> Option<Tile> {
        self.tiles
            .iter()
            .find(|tile| tile.pointer() == pointer)
            .cloned()
    }
}

impl Layer {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Layer.name must be a string"))?;

        let kind_js = Reflect::get(value, &JsValue::from_str("kind"))?;
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
        
        let mut masks = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = Mask::from_js_value(&mask_js)?;
            masks.push(mask);
        }

        let native_layer = rpgx::engine::map::layer::Layer::new(
            name.clone(),
            kind.to_native(),
            shape.to_native(),
            masks.iter().map(|m| m.to_native()).collect(),
        );

        Ok(Layer::from_native(native_layer))
    }
}

impl Layer {
    pub fn from_native(layer: rpgx::engine::map::layer::Layer) -> Self {
        Layer {
            name: layer.name,
            kind: match layer.kind {
                rpgx::prelude::LayerType::Default => LayerType::Default,
                rpgx::prelude::LayerType::Texture => LayerType::Texture,
                rpgx::prelude::LayerType::Block => LayerType::Block,
                rpgx::prelude::LayerType::Action => LayerType::Action,
            },
            shape: Shape::new(layer.shape.width, layer.shape.height),
            masks: layer.masks.into_iter().map(Mask::from_native).collect(),
            tiles: layer.tiles.into_iter().map(Tile::from_native).collect(),
        }
    }

    pub fn to_native(&self) -> rpgx::engine::map::layer::Layer {
        rpgx::engine::map::layer::Layer {
            name: self.name.clone(),
            kind: self.kind.to_native(),
            shape: self.shape.to_native(),
            tiles: self.tiles.iter().map(|t| t.to_native()).collect(),
            masks: self
                .masks
                .iter()
                .map(|m| m.to_native())
                .collect(),
        }
    }
}
