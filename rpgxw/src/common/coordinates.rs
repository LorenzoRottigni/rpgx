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

    #[wasm_bindgen]
    pub fn bounding_box(coords: &JsValue) -> Result<js_sys::Array, JsValue> {
        let arr = js_sys::Array::from(coords);
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        for val in arr.iter() {
            let coord = WasmCoordinates::from_js_value(&val)?;
            xs.push(coord.x);
            ys.push(coord.y);
        }

        let min_x = xs.iter().min().cloned().unwrap_or(0);
        let min_y = ys.iter().min().cloned().unwrap_or(0);
        let max_x = xs.iter().max().cloned().unwrap_or(0);
        let max_y = ys.iter().max().cloned().unwrap_or(0);

        let min_coord = WasmCoordinates::new(min_x, min_y).to_js_value();
        let max_coord = WasmCoordinates::new(max_x, max_y).to_js_value();

        let result = js_sys::Array::new();
        result.push(&min_coord);
        result.push(&max_coord);

        Ok(result)
    }
}
