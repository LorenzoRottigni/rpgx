use rpgx::prelude::Delta;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Delta)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmDelta {
    inner: Delta,
}

#[wasm_bindgen(js_class = Delta)]
impl WasmDelta {
    #[wasm_bindgen(constructor)]
    pub fn new(dx: i32, dy: i32) -> WasmDelta {
        WasmDelta {
            inner: Delta::new(dx, dy),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> i32 {
        self.inner.dx
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> i32 {
        self.inner.dy
    }

    #[wasm_bindgen]
    pub fn zero() -> WasmDelta {
        WasmDelta {
            inner: Delta::zero(),
        }
    }

    #[wasm_bindgen]
    pub fn invert(&self) -> WasmDelta {
        WasmDelta {
            inner: self.inner.invert(),
        }
    }

    #[wasm_bindgen(js_name = isZero)]
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    #[wasm_bindgen]
    pub fn manhattan(&self) -> u32 {
        self.inner.manhattan()
    }

    #[wasm_bindgen(js_name = isAxisAligned)]
    pub fn is_axis_aligned(&self) -> bool {
        self.inner.is_axis_aligned()
    }

    #[wasm_bindgen(js_name = isDiagonal)]
    pub fn is_diagonal(&self) -> bool {
        self.inner.is_diagonal()
    }

    #[wasm_bindgen]
    pub fn add(&self, other: &WasmDelta) -> WasmDelta {
        WasmDelta {
            inner: self.inner + other.inner,
        }
    }

    #[wasm_bindgen]
    pub fn sub(&self, other: &WasmDelta) -> WasmDelta {
        WasmDelta {
            inner: self.inner - other.inner,
        }
    }

    #[wasm_bindgen]
    pub fn neg(&self) -> WasmDelta {
        WasmDelta { inner: -self.inner }
    }
}

// Internal Rust API
impl WasmDelta {
    pub fn from_inner(inner: Delta) -> Self {
        WasmDelta { inner }
    }

    pub fn inner(&self) -> &Delta {
        &self.inner
    }

    pub fn into_inner(self) -> Delta {
        self.inner
    }
}
