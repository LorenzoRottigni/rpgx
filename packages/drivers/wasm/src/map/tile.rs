use rpgx::prelude::*;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmDelta, WasmEffect, WasmRect};

#[wasm_bindgen(js_name = Tile)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmTile {
    inner: Tile,
}

#[wasm_bindgen(js_class = Tile)]
impl WasmTile {
    /// Create a new tile from an effect and area
    #[wasm_bindgen(constructor)]
    pub fn new(effect: &WasmEffect, area: &WasmRect) -> WasmTile {
        WasmTile {
            inner: Tile::new(effect.inner().clone(), area.inner().clone()),
        }
    }

    /// Create a tile with default effect from area
    #[wasm_bindgen(js_name = fromArea)]
    pub fn from_area(area: &WasmRect) -> WasmTile {
        WasmTile {
            inner: Tile::from_area(area.inner().clone()),
        }
    }

    /// Get the effect of this tile
    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> WasmEffect {
        WasmEffect::from_inner(self.inner.effect.clone())
    }

    /// Set the effect of this tile
    #[wasm_bindgen(setter, js_name = setEffect)]
    pub fn set_effect(&mut self, effect: &WasmEffect) {
        self.inner.effect = effect.inner().clone();
    }

    /// Get the area covered by this tile
    #[wasm_bindgen(getter)]
    pub fn area(&self) -> WasmRect {
        WasmRect::from_inner(self.inner.area.clone())
    }

    /// Set the area covered by this tile
    #[wasm_bindgen(setter, js_name=setArea)]
    pub fn set_area(&mut self, area: &WasmRect) {
        self.inner.area = area.inner().clone();
    }

    /// Apply a new effect to this tile (overwrites existing)
    #[wasm_bindgen]
    pub fn apply(&mut self, effect: &WasmEffect) {
        self.inner.apply(effect.inner().clone());
    }

    /// Returns true if tile blocks movement or interaction at the given coordinate
    #[wasm_bindgen(js_name = isBlockingAt)]
    pub fn is_blocking_at(&self, target: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(target.inner().clone())
    }

    /// Offsets the tile’s area and effect’s blocking region by the given delta
    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(delta.inner().clone());
    }

    /// Translate the tile by a delta (alias for offset)
    #[wasm_bindgen]
    pub fn translate(&mut self, delta: &WasmDelta) {
        self.inner.translate(delta.inner().clone());
    }

    /// Returns true if the tile’s area contains the specified coordinate
    #[wasm_bindgen]
    pub fn contains(&self, coord: &WasmCoordinates) -> bool {
        self.inner.contains(coord.inner().clone())
    }

    /// Display the tile as a string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.inner)
    }
}

// Internal Rust API
impl WasmTile {
    pub fn from_inner(inner: Tile) -> Self {
        WasmTile { inner }
    }

    pub fn inner(&self) -> &Tile {
        &self.inner
    }

    pub fn into_inner(self) -> Tile {
        self.inner
    }
}
