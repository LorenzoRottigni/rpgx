use js_sys::{Array, Reflect};
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmMask, WasmShape, WasmTile};
use rpgx::prelude::Layer;
pub mod mask;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum WasmLayerType {
    Base,
    Texture,
    Block,
    Action,
}

impl WasmLayerType {
    fn to_native(&self) -> rpgx::prelude::LayerType {
        match self {
            WasmLayerType::Base => rpgx::prelude::LayerType::Base,
            WasmLayerType::Action => rpgx::prelude::LayerType::Action,
            WasmLayerType::Texture => rpgx::prelude::LayerType::Texture,
            WasmLayerType::Block => rpgx::prelude::LayerType::Block,
        }
    }
}

/// Represents a layer in the RPGX engine, which can contain tiles and masks.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmLayer {
    name: String,
    kind: WasmLayerType,
    shape: WasmShape,
    masks: Vec<WasmMask>,
    tiles: Vec<WasmTile>,
}

impl WasmLayer {
    /// Creates a new `WasmLayer` instance from a native RPGX layer.
    pub fn from_native(layer: Layer) -> Self {
        Self {
            name: layer.name,
            kind: match layer.kind {
                rpgx::prelude::LayerType::Base => WasmLayerType::Base,
                rpgx::prelude::LayerType::Texture => WasmLayerType::Texture,
                rpgx::prelude::LayerType::Block => WasmLayerType::Block,
                rpgx::prelude::LayerType::Action => WasmLayerType::Action,
            },
            shape: WasmShape::new(layer.shape.width, layer.shape.height),
            masks: layer.masks.into_iter().map(WasmMask::from_native).collect(),
            tiles: layer.tiles.into_iter().map(WasmTile::from_native).collect(),
        }
    }

    /// Converts the `WasmLayer` instance to a native RPGX layer.
    pub fn to_native(&self) -> Layer {
        Layer {
            name: self.name.clone(),
            kind: self.kind.to_native(),
            shape: self.shape.to_native(),
            tiles: self.tiles.iter().map(|t| t.to_native()).collect(),
            masks: self.masks.iter().map(|m| m.to_native()).collect(),
        }
    }

    /// Creates a new `WasmLayer` instance from a JavaScript object.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Layer.name must be a string"))?;

        let kind_js = Reflect::get(value, &JsValue::from_str("kind"))?;
        let kind_num = kind_js
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Layer.kind must be a number"))?;
        let kind = match kind_num as u32 {
            0 => WasmLayerType::Base,
            1 => WasmLayerType::Texture,
            2 => WasmLayerType::Block,
            3 => WasmLayerType::Action,
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
        let shape = WasmShape::new(width, height);

        let masks_js = Reflect::get(value, &JsValue::from_str("masks"))?;
        let masks_array = masks_js
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("Layer.masks must be an Array"))?;

        let mut masks = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = WasmMask::from_js_value(&mask_js)?;
            masks.push(mask);
        }

        let native_layer = Layer::new(
            name.clone(),
            kind.to_native(),
            shape.to_native(),
            masks.iter().map(|m| m.to_native()).collect(),
        );

        Ok(Self::from_native(native_layer))
    }

    /// Converts the `WasmLayer` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &JsValue::from(self.name.clone()),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("kind"),
            &JsValue::from(self.kind as u32),
        )
        .unwrap();
        Reflect::set(&obj, &JsValue::from_str("shape"), &self.shape.to_js_value()).unwrap();

        let masks_array = Array::new();
        for mask in &self.masks {
            masks_array.push(&mask.to_js_value());
        }
        Reflect::set(&obj, &JsValue::from_str("masks"), &masks_array).unwrap();

        obj.into()
    }
}

#[wasm_bindgen]
impl WasmLayer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        kind: WasmLayerType,
        shape: WasmShape,
        masks: &JsValue,
    ) -> Result<Self, JsValue> {
        let masks_array = masks
            .dyn_ref::<Array>()
            .ok_or_else(|| JsValue::from_str("masks must be an Array"))?;

        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask = WasmMask::from_js_value(&masks_array.get(i))?;
            masks_vec.push(mask);
        }

        let native_layer = Layer::new(
            name.clone(),
            kind.to_native(),
            shape.to_native(),
            masks_vec.iter().map(|m| m.to_native()).collect(),
        );

        Ok(Self::from_native(native_layer))
    }

    #[wasm_bindgen(getter)]
    pub fn tiles(&self) -> Vec<WasmTile> {
        self.tiles.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> WasmLayerType {
        self.kind
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> WasmShape {
        self.shape.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> Vec<WasmMask> {
        self.masks.clone()
    }

    #[wasm_bindgen]
    pub fn get_tile(&self, pointer: WasmCoordinates) -> Option<WasmTile> {
        self.tiles
            .iter()
            .find(|tile| tile.pointer() == pointer)
            .cloned()
    }

    #[wasm_bindgen]
    pub fn get_block(&self, start: WasmCoordinates, end: WasmCoordinates) -> Vec<WasmTile> {
        self.shape
            .coordinates_in_range(start, end)
            .into_iter()
            .filter_map(|coord| self.tiles.iter().find(|t| t.pointer() == coord).cloned())
            .collect()
    }

    #[wasm_bindgen]
    pub fn is_tile_blocked(&self, target: WasmCoordinates) -> bool {
        self.tiles.iter().any(|tile| tile.is_blocking_at(target))
    }
}
