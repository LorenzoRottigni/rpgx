use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Shape {
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl Shape {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Shape {
        Shape { width, height }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

impl Shape {
    pub fn to_native(&self) -> rpgx::common::shape::Shape {
        rpgx::common::shape::Shape {
            width: self.width,
            height: self.height,
        }
    }

    pub fn from_native(shape: rpgx::common::shape::Shape) -> Self {
        Shape {
            width: shape.width,
            height: shape.height,
        }
    }

    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let width = Reflect::get(value, &JsValue::from_str("width"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.width must be a number"))?
            as i32;
        let height = Reflect::get(value, &JsValue::from_str("height"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.height must be a number"))?
            as i32;
        Ok(Shape { width, height })
    }

    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("width"), &JsValue::from_f64(self.width as f64)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("height"), &JsValue::from_f64(self.height as f64)).unwrap();
        obj.into()
    }
}
