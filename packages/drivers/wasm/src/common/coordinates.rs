use rpgx::prelude::Coordinates;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCoordinates {
    pub(crate) inner: Coordinates,
}

#[wasm_bindgen]
impl WasmCoordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> WasmCoordinates {
        WasmCoordinates {
            inner: Coordinates { x, y },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.inner.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.inner.y
    }

    #[wasm_bindgen(js_name = add)]
    pub fn add(&self, other: &WasmCoordinates) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner + other.inner,
        }
    }
}

impl WasmCoordinates {
    pub fn from_inner(inner: Coordinates) -> WasmCoordinates {
        WasmCoordinates { inner }
    }

    pub fn into_inner(self) -> Coordinates {
        self.inner
    }
}
