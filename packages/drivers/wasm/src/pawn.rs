use crate::prelude::WasmTile; // Assuming you have a WasmTile wrapper
use rpgx::prelude::{Pawn, Tile};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPawn {
    inner: Pawn,
}

#[wasm_bindgen]
impl WasmPawn {
    #[wasm_bindgen(constructor)]
    pub fn new(tile: WasmTile, texture_id: i32) -> WasmPawn {
        WasmPawn {
            inner: Pawn {
                tile: tile.into_inner(),
                texture_id,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> WasmTile {
        WasmTile::from_inner(self.inner.tile.clone())
    }

    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, tile: WasmTile) {
        self.inner.tile = tile.into_inner();
    }

    #[wasm_bindgen(getter, js_name = textureId)]
    pub fn texture_id(&self) -> i32 {
        self.inner.texture_id
    }

    #[wasm_bindgen(setter, js_name = textureId)]
    pub fn set_texture_id(&mut self, texture_id: i32) {
        self.inner.texture_id = texture_id;
    }
}

impl WasmPawn {
    // Converts WasmPawn into the inner Pawn
    pub fn into_inner(self) -> Pawn {
        self.inner
    }

    // Creates WasmPawn from inner Pawn
    pub fn from_inner(inner: Pawn) -> WasmPawn {
        WasmPawn { inner }
    }

    // Access inner reference
    pub fn inner(&self) -> &Pawn {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Pawn {
        &mut self.inner
    }
}
