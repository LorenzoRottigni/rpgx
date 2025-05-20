use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Coordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl Coordinates {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let x = Reflect::get(value, &JsValue::from_str("x"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.x must be a number"))?
            as i32;
        let y = Reflect::get(value, &JsValue::from_str("y"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.y must be a number"))?
            as i32;
        Ok(Coordinates { x, y })
    }
}

impl Coordinates {
    pub fn to_native(&self) -> rpgx::common::coordinates::Coordinates {
        rpgx::common::coordinates::Coordinates {
            x: self.x,
            y: self.y,
        }
    }
}
