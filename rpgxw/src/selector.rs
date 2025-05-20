use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinates;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Selector {
    start: Coordinates,
    end: Coordinates,
}

#[wasm_bindgen]
impl Selector {
    #[wasm_bindgen(constructor)]
    pub fn new_single(coord: Coordinates) -> Selector {
        Selector {
            start: coord,
            end: coord,
        }
    }

    #[wasm_bindgen]
    pub fn new_block(start: Coordinates, end: Coordinates) -> Selector {
        Selector { start, end }
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Coordinates {
        self.start
    }

    #[wasm_bindgen(setter)]
    pub fn set_start(&mut self, start: Coordinates) {
        self.start = start;
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Coordinates {
        self.end
    }

    #[wasm_bindgen(setter)]
    pub fn set_end(&mut self, end: Coordinates) {
        self.end = end;
    }
}

impl Selector {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let start_js = Reflect::get(value, &JsValue::from_str("start"))?;
        let start = Coordinates::from_js_value(&start_js)?;
        let end_js = Reflect::get(value, &JsValue::from_str("end"))?;
        let end = Coordinates::from_js_value(&end_js)?;
        Ok(Selector { start, end })
    }
}

impl Selector {
    pub fn to_native(&self) -> rpgx::engine::map::selector::Selector {
        if self.start == self.end {
            rpgx::engine::map::selector::Selector::Single(self.start.to_native())
        } else {
            rpgx::engine::map::selector::Selector::Block((
                self.start.to_native(),
                self.end.to_native(),
            ))
        }
        // Filter variant can't be constructed from JS currently
    }
}
