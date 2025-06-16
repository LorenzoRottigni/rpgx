use crate::prelude::{WasmDelta, WasmShape};
use rpgx::prelude::Coordinates;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmCoordinates {
    inner: Coordinates,
}

#[wasm_bindgen]
impl WasmCoordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> WasmCoordinates {
        WasmCoordinates {
            inner: Coordinates::new(x, y),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.inner.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.inner.y
    }

    #[wasm_bindgen]
    pub fn is_origin(&self) -> bool {
        self.inner.is_origin()
    }

    #[wasm_bindgen]
    pub fn is_aligned_with(&self, other: &WasmCoordinates) -> bool {
        self.inner.is_aligned_with(other.inner)
    }

    #[wasm_bindgen]
    pub fn is_within(&self, origin: &WasmCoordinates, shape: &WasmShape) -> bool {
        self.inner.is_within(origin.inner, *shape.inner())
    }

    #[wasm_bindgen]
    pub fn offseted(&self, delta: &WasmDelta) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner.offseted(*delta.inner()),
        }
    }

    #[wasm_bindgen]
    pub fn try_offseted(&self, delta: &WasmDelta) -> Option<WasmCoordinates> {
        self.inner
            .try_offseted(*delta.inner())
            .map(|c| WasmCoordinates { inner: c })
    }

    #[wasm_bindgen]
    pub fn to_delta(&self) -> WasmDelta {
        WasmDelta::from_inner(self.inner.to_delta())
    }

    #[wasm_bindgen]
    pub fn add_coordinates(&self, other: &WasmCoordinates) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner + other.inner,
        }
    }

    #[wasm_bindgen]
    pub fn sub_coordinates(&self, other: &WasmCoordinates) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner - other.inner,
        }
    }

    #[wasm_bindgen]
    pub fn add_shape(&self, shape: &WasmShape) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner + *shape.inner(),
        }
    }

    #[wasm_bindgen]
    pub fn sub_shape(&self, shape: &WasmShape) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner - *shape.inner(),
        }
    }

    #[wasm_bindgen]
    pub fn add_delta(&self, delta: &WasmDelta) -> Option<WasmCoordinates> {
        (self.inner + *delta.inner()).map(|c| WasmCoordinates { inner: c })
    }

    #[wasm_bindgen]
    pub fn sub_delta(&self, delta: &WasmDelta) -> Option<WasmCoordinates> {
        (self.inner - *delta.inner()).map(|c| WasmCoordinates { inner: c })
    }

    #[wasm_bindgen(js_name = boundingBox)]
    pub fn bounding_box(coords: Vec<WasmCoordinates>) -> Option<js_sys::Array> {
        let inner: Vec<Coordinates> = coords.into_iter().map(|c| c.inner).collect();
        Coordinates::bounding_box(&inner).map(|(min, max)| {
            let array = js_sys::Array::new();
            array.push(&WasmCoordinates { inner: min }.into());
            array.push(&WasmCoordinates { inner: max }.into());
            array
        })
    }
}

// Internal Rust API
impl WasmCoordinates {
    pub fn from_inner(inner: Coordinates) -> Self {
        WasmCoordinates { inner }
    }

    pub fn inner(&self) -> &Coordinates {
        &self.inner
    }

    pub fn into_inner(self) -> Coordinates {
        self.inner
    }
}
