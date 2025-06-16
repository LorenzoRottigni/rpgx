use rpgx::prelude::Layer;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmDelta, WasmMask, WasmShape, WasmTile};

#[wasm_bindgen(js_name = Layer)]
pub struct WasmLayer {
    inner: Layer,
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

    /// Returns the first tile at the given coordinate or null if none.
    #[wasm_bindgen(js_name = getTileAt)]
    pub fn get_tile_at(&self, coord: &WasmCoordinates) -> Option<WasmTile> {
        self.inner
            .get_tile_at(*coord.inner())
            .map(WasmTile::from_inner)
    }

    /// Returns true if any tile blocks movement at the coordinate.
    #[wasm_bindgen(js_name = isBlockingAt)]
    pub fn is_blocking_at(&self, coord: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(coord.inner())
    }

    /// Returns shapes of all masks.
    #[wasm_bindgen(js_name = getShapes)]
    pub fn get_shapes(&self) -> Vec<WasmShape> {
        self.inner
            .get_shapes()
            .into_iter()
            .map(WasmShape::from_inner)
            .collect()
    }

    /// Returns overall bounding shape of the layer.
    #[wasm_bindgen(js_name = getShape)]
    pub fn get_shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.get_shape())
    }

    /// Returns all tiles flattened.
    #[wasm_bindgen]
    pub fn render(&self) -> Vec<WasmTile> {
        self.inner
            .render()
            .into_iter()
            .map(WasmTile::from_inner)
            .collect()
    }

    /// Offset all tiles by delta.
    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(*delta.inner());
    }
}

impl WasmLayer {
    /// Get a reference to the inner Layer
    pub fn inner(&self) -> &Layer {
        &self.inner
    }

    /// Consume WasmLayer and return the inner Layer
    pub fn into_inner(self) -> Layer {
        self.inner
    }

    /// Create WasmLayer from inner Layer directly
    pub fn from_inner(inner: Layer) -> WasmLayer {
        WasmLayer { inner }
    }
}
