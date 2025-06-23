use rpgx::prelude::{Coordinates, Shape};
use wasm_bindgen::prelude::*;

use crate::traits::WasmWrapper;

#[wasm_bindgen(js_name = Shape)]
#[derive(Clone, Debug)]
pub struct WasmShape {
    inner: Shape,
}

impl WasmWrapper<Shape> for WasmShape {
    /// Get a reference to the inner Shape
    fn inner(&self) -> &Shape {
        &self.inner
    }

    /// Consume WasmShape and return the inner Shape
    fn into_inner(self) -> Shape {
        self.inner
    }

    /// Create WasmShape from inner Shape directly
    fn from_inner(inner: Shape) -> Self {
        WasmShape { inner }
    }
}

#[wasm_bindgen(js_class = Shape)]
impl WasmShape {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> WasmShape {
        WasmShape {
            inner: Shape::from_rectangle(width, height),
        }
    }

    #[wasm_bindgen(js_name = fromSquare)]
    pub fn from_square(side: u32) -> WasmShape {
        WasmShape {
            inner: Shape::from_square(side),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.inner.height
    }

    #[wasm_bindgen(js_name = area)]
    pub fn area(&self) -> u32 {
        self.inner.area()
    }

    /// Returns a copy of the shape offset by the given x/y
    #[wasm_bindgen(js_name = offsetBy)]
    pub fn offset_by(&self, x: u32, y: u32) -> WasmShape {
        let offset = Coordinates { x, y };
        WasmShape {
            inner: self.inner.offset_by(offset),
        }
    }
}
