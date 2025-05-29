use rpgx::prelude::{Coordinates, Direction, Shape};
use wasm_bindgen::prelude::*;

use crate::prelude::WasmCoordinates;

#[wasm_bindgen]
pub struct WasmShape {
    pub(crate) inner: Shape,
}

#[wasm_bindgen]
impl WasmShape {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> WasmShape {
        WasmShape {
            inner: Shape { width, height },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> i32 {
        self.inner.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> i32 {
        self.inner.height
    }

    #[wasm_bindgen(js_name = fromSquare)]
    pub fn from_square(side: i32) -> WasmShape {
        WasmShape {
            inner: Shape::from_square(side),
        }
    }

    #[wasm_bindgen(js_name = fromBounds)]
    pub fn from_bounds(start: &WasmCoordinates, end: &WasmCoordinates) -> WasmShape {
        WasmShape {
            inner: Shape::from_bounds(start.inner, end.inner),
        }
    }

    #[wasm_bindgen(js_name = inBounds)]
    pub fn in_bounds(&self, coord: &WasmCoordinates) -> bool {
        self.inner.in_bounds(coord.inner)
    }

    #[wasm_bindgen(js_name = offsetBy)]
    pub fn offset_by(&self, offset: &WasmCoordinates) -> WasmShape {
        WasmShape {
            inner: self.inner.offset_by(offset.inner),
        }
    }
}

impl WasmShape {
    pub fn from_inner(inner: Shape) -> WasmShape {
        WasmShape { inner }
    }

    pub(crate) fn into_inner(&self) -> Shape {
        self.inner.clone()
    }
}
