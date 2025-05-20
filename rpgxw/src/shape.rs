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
}
