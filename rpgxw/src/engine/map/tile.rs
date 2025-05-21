use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmEffect, WasmShape};

/// Represents a tile in the RPGX engine, which can have various effects and properties.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmTile {
    id: i32,
    effect: WasmEffect,
    pointer: WasmCoordinates,
    shape: WasmShape,
}

impl WasmTile {
    /// Converts the `WasmTile` instance to a native RPGX tile.
    pub fn from_native(native_tile: rpgx::engine::map::tile::Tile) -> Self {
        Self {
            id: native_tile.id,
            effect: WasmEffect::from_native(native_tile.effect),
            pointer: WasmCoordinates::new(native_tile.pointer.x, native_tile.pointer.y),
            shape: WasmShape::new(native_tile.shape.width, native_tile.shape.height),
        }
    }

    /// Converts the `WasmTile` instance to a native RPGX tile.
    pub fn to_native(&self) -> rpgx::engine::map::tile::Tile {
        rpgx::engine::map::tile::Tile {
            id: self.id,
            effect: self.effect.to_native(),
            pointer: rpgx::common::coordinates::Coordinates {
                x: self.pointer.x(),
                y: self.pointer.y(),
            },
            shape: self.shape.to_native(),
        }
    }

    /// Creates a new `WasmTile` instance from a native RPGX tile.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let id = Reflect::get(value, &JsValue::from_str("id"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Tile.id must be a number"))? as i32;

        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = WasmEffect::from_js_value(&effect_js)?;

        let pointer_js = Reflect::get(value, &JsValue::from_str("pointer"))?;
        let pointer = WasmCoordinates::from_js_value(&pointer_js)?;

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

        Ok(Self {
            id,
            effect,
            pointer,
            shape,
        })
    }

    /// Converts the `WasmTile` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("id"), &JsValue::from_f64(self.id as f64)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("effect"), &self.effect.to_js_value()).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("pointer"), &self.pointer.to_js_value()).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("shape"), &self.shape.to_js_value()).unwrap();
        obj.into()
    }
}


#[wasm_bindgen]
impl WasmTile {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, effect: WasmEffect, pointer: WasmCoordinates, shape: WasmShape) -> Self {
        Self {
            id,
            effect,
            pointer,
            shape,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> WasmEffect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: WasmEffect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> WasmCoordinates {
        self.pointer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_pointer(&mut self, pointer: WasmCoordinates) {
        self.pointer = pointer;
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> WasmShape {
        self.shape.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_shape(&mut self, shape: WasmShape) {
        self.shape = shape;
    }

    #[wasm_bindgen]
    pub fn is_blocking_at(&self, target: WasmCoordinates) -> bool {
        if !self.effect.block() {
            return false;
        }

        if let Some(s) = self.effect().shrink() {
            // Shrink is interpreted as absolute bounds
            target.x() >= s.start().x() && target.x() <= s.end().x() && target.y() >= s.start().y() && target.y() <= s.end().y()
        } else {
            self.contains(target)
        }
    }

    #[wasm_bindgen]
    pub fn generate_default_grid(shape: WasmShape, effect: WasmEffect) -> Vec<WasmTile> {
        let mut tiles = Vec::new();
        for y in 0..shape.height() {
            for x in 0..shape.width() {
                tiles.push(WasmTile {
                    id: x,
                    pointer: WasmCoordinates::new(x, y),
                    shape: WasmShape::new(1, 1),
                    effect: effect.clone(),
                });
            }
        }
        tiles
    }

    #[wasm_bindgen]
    pub fn contains(&self, point: WasmCoordinates) -> bool {
        let start = self.pointer;
        let end = WasmCoordinates::new(start.x() + self.shape.width() - 1, start.y() + self.shape.height() - 1);

        point.x() >= start.x() && point.x() <= end.x() && point.y() >= start.y() && point.y() <= end.y()
    }
}

