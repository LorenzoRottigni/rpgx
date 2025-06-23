use rpgx::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{
    prelude::{WasmCoordinates, WasmDelta, WasmEffect, WasmRect, WasmShape},
    traits::WasmWrapper,
};

#[wasm_bindgen(js_name = Mask)]
pub struct WasmMask {
    inner: Mask,
}

impl WasmWrapper<Mask> for WasmMask {
    /// Get a reference to the inner Mask (Rust struct)
    fn inner(&self) -> &Mask {
        &self.inner
    }

    /// Consume WasmMask and return the inner Mask
    fn into_inner(self) -> Mask {
        self.inner
    }

    /// Create WasmMask from inner Mask directly
    fn from_inner(inner: Mask) -> WasmMask {
        WasmMask { inner }
    }
}

#[wasm_bindgen(js_class = Mask)]
impl WasmMask {
    /// Create a new Mask from name, areas, and uniform effect
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, areas: Vec<WasmRect>, effects: Vec<WasmEffect>) -> WasmMask {
        let inner_areas = areas.into_iter().map(|r| r.into_inner()).collect();
        WasmMask {
            inner: Mask::new(
                name,
                inner_areas,
                effects.iter().map(|e| e.into_inner()).collect(),
            ),
        }
    }

    /// Get the mask's name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Offset all tiles and their effects by delta
    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(*delta.inner());
    }

    /// Get the bounding shape of all tiles
    #[wasm_bindgen(js_name = getShape)]
    pub fn get_shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.get_shape())
    }

    /// Returns true if any tile contains the coordinate
    #[wasm_bindgen]
    pub fn contains(&self, coord: &WasmCoordinates) -> bool {
        self.inner.contains(coord.inner())
    }
}
