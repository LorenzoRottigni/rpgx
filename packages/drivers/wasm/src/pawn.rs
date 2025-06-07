use crate::prelude::{WasmCoordinates, WasmTile}; // Assuming you have a WasmTile wrapper
use rpgx::prelude::Pawn;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPawn {
    inner: Pawn,
}

#[wasm_bindgen]
impl WasmPawn {
    #[wasm_bindgen(constructor)]
    pub fn new(pointer: WasmCoordinates, texture_id: u32) -> WasmPawn {
        WasmPawn {
            inner: Pawn {
                pointer: pointer.into_inner(),
                texture_id,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> WasmCoordinates {
        WasmCoordinates::from_inner(self.inner.pointer.clone())
    }

    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, pointer: WasmCoordinates) {
        self.inner.pointer = pointer.into_inner();
    }

    #[wasm_bindgen(getter, js_name = textureId)]
    pub fn texture_id(&self) -> u32 {
        self.inner.texture_id
    }

    #[wasm_bindgen(setter, js_name = textureId)]
    pub fn set_texture_id(&mut self, texture_id: u32) {
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
