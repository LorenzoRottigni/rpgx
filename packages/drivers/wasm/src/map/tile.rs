use crate::prelude::{WasmCoordinates, WasmEffect, WasmShape};
use rpgx::prelude::Tile;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTile {
    inner: Tile,
}

#[wasm_bindgen]
impl WasmTile {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: u32,
        effect: &WasmEffect,
        pointer: &WasmCoordinates,
        shape: &WasmShape,
    ) -> WasmTile {
        WasmTile {
            inner: Tile::new(id, effect.into_inner(), pointer.inner, shape.inner),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.inner.id
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> WasmEffect {
        WasmEffect::from_effect(self.inner.effect)
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.inner.pointer,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> WasmShape {
        WasmShape {
            inner: self.inner.shape,
        }
    }

    /// Checks if this tile contains the given coordinates.
    pub fn contains(&self, point: &WasmCoordinates) -> bool {
        self.inner.contains(point.inner)
    }

    /// Returns true if the tile blocks at the given target coordinates.
    pub fn is_blocking_at(&self, target: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(target.inner)
    }

    /// Offsets the tile's pointer and shrink effect by the delta coordinates.
    pub fn offset(&mut self, delta: &WasmCoordinates) {
        self.inner.offset(delta.inner);
    }
}

impl WasmTile {
    pub(crate) fn from_inner(inner: Tile) -> Self {
        WasmTile { inner }
    }

    pub(crate) fn _into_inner(self) -> Tile {
        self.inner
    }
}
