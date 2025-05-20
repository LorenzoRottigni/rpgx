use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::{coordinates::Coordinates, effect::Effect, shape::Shape};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Tile {
    id: i32,
    effect: Effect,
    pointer: Coordinates, // assuming SingleSelector is just Coordinates here; adjust if not
    shape: Shape,
}

#[wasm_bindgen]
impl Tile {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, effect: Effect, pointer: Coordinates, shape: Shape) -> Tile {
        Tile {
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
    pub fn effect(&self) -> Effect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> Coordinates {
        self.pointer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_pointer(&mut self, pointer: Coordinates) {
        self.pointer = pointer;
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }
}

impl Tile {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let id = Reflect::get(value, &JsValue::from_str("id"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Tile.id must be a number"))? as i32;

        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = Effect::from_js_value(&effect_js)?;

        let pointer_js = Reflect::get(value, &JsValue::from_str("pointer"))?;
        let pointer = Coordinates::from_js_value(&pointer_js)?;

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

        Ok(Tile {
            id,
            effect,
            pointer,
            shape,
        })
    }

    pub fn to_native(&self) -> rpgx::engine::map::tile::Tile {
        rpgx::engine::map::tile::Tile {
            id: self.id,
            effect: self.effect.to_native(),
            pointer: rpgx::common::coordinates::Coordinates {
                x: self.pointer.x(),
                y: self.pointer.y(),
            }, // adapt if SingleSelector is not Coordinates
            shape: self.shape.to_native(),
        }
    }
}
