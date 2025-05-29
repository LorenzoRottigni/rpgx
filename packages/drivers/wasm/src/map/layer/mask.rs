use crate::prelude::{WasmEffect, WasmSelector, WasmShape, WasmTile};
use rpgx::prelude::{Coordinates, Effect, Mask, Selector, Shape, Tile};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMask {
    inner: Mask,
}

#[wasm_bindgen]
impl WasmMask {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, selector: WasmSelector, effect: WasmEffect) -> WasmMask {
        WasmMask {
            inner: Mask::new(name, selector.into_inner(), effect.into_inner()),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.inner.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn selector(&self) -> WasmSelector {
        WasmSelector::from_inner(self.inner.selector)
    }

    #[wasm_bindgen(setter)]
    pub fn set_selector(&mut self, selector: WasmSelector) {
        self.inner.selector = selector.into_inner();
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> WasmEffect {
        WasmEffect::from_effect(self.inner.effect)
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: WasmEffect) {
        self.inner.effect = effect.into_inner();
    }

    /// Applies the mask to the given shape, returning an array of Tiles.
    #[wasm_bindgen]
    pub fn apply(&self, shape: &WasmShape) -> js_sys::Array {
        let tiles = self.inner.apply(shape.into_inner());
        let arr = js_sys::Array::new_with_length(tiles.len() as u32);

        for (i, tile) in tiles.into_iter().enumerate() {
            arr.set(i as u32, WasmTile::from_inner(tile).into());
        }

        arr
    }
}

// Helper methods to convert between wasm wrappers and inner Rust structs
impl WasmMask {
    pub(crate) fn from_inner(inner: Mask) -> Self {
        Self { inner }
    }

    pub(crate) fn into_inner(self) -> Mask {
        self.inner
    }

    pub fn as_inner(&self) -> &Mask {
        &self.inner
    }
}
