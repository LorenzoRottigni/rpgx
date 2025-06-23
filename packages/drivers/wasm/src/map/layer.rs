use rpgx::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{
    prelude::{WasmCoordinates, WasmDelta, WasmMask, WasmShape},
    traits::WasmWrapper,
};

#[wasm_bindgen(js_name = Layer)]
pub struct WasmLayer {
    inner: Layer,
}

impl WasmWrapper<Layer> for WasmLayer {
    /// Get a reference to the inner Layer
    fn inner(&self) -> &Layer {
        &self.inner
    }

    /// Consume WasmLayer and return the inner Layer
    fn into_inner(self) -> Layer {
        self.inner
    }

    /// Create WasmLayer from inner Layer directly
    fn from_inner(inner: Layer) -> WasmLayer {
        WasmLayer { inner }
    }
}

#[wasm_bindgen(js_class = Layer)]
impl WasmLayer {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, masks: Vec<WasmMask>, z: u32) -> WasmLayer {
        let inner_masks = masks.into_iter().map(|m| m.into_inner()).collect();
        WasmLayer {
            inner: Layer::new(name, inner_masks, z),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> u32 {
        self.inner.z
    }

    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> Vec<WasmMask> {
        self.inner
            .masks
            .iter()
            .cloned()
            .map(WasmMask::from_inner)
            .collect()
    }

    /// Returns true if any tile blocks movement at the coordinate.
    #[wasm_bindgen(js_name = isBlockingAt)]
    pub fn is_blocking_at(&self, coord: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(coord.inner())
    }

    /// Returns overall bounding shape of the layer.
    #[wasm_bindgen(js_name = getShape)]
    pub fn get_shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.get_shape())
    }

    /// Offset all tiles by delta.
    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(*delta.inner());
    }
}
