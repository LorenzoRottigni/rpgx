use js_sys::Reflect;
use wasm_bindgen::prelude::*;

/// Represents a coordinate in the RPGX engine, which can be used to specify positions on the map.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmCoordinates {
    x: i32,
    y: i32,
}

impl WasmCoordinates {
    /// Creates a new `WasmCoordinates` instance from the given x and y coordinates.
    pub fn from_native(coords: rpgx::common::coordinates::Coordinates) -> Self {
        Self {
            x: coords.x,
            y: coords.y,
        }
    }

    /// Converts the `WasmCoordinates` instance to a native RPGX coordinates.
    pub fn to_native(&self) -> rpgx::common::coordinates::Coordinates {
        rpgx::common::coordinates::Coordinates {
            x: self.x,
            y: self.y,
        }
    }

    /// Creates a new `WasmCoordinates` instance from a JavaScript object.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let x = Reflect::get(value, &JsValue::from_str("x"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.x must be a number"))?
            as i32;
        let y = Reflect::get(value, &JsValue::from_str("y"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.y must be a number"))?
            as i32;
        Ok(Self { x, y })
    }

    /// Converts the `WasmCoordinates` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("x"), &JsValue::from_f64(self.x as f64)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("y"), &JsValue::from_f64(self.y as f64)).unwrap();
        obj.into()
    }
}


#[wasm_bindgen]
impl WasmCoordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
