use crate::{prelude::WasmCoordinates, traits::WasmWrapper}; // Assuming you have a WasmTile wrapper
use rpgx::prelude::Pawn;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Pawn)]
pub struct WasmPawn {
    inner: Pawn,
}

impl WasmWrapper<Pawn> for WasmPawn {
    // Converts WasmPawn into the inner Pawn
    fn into_inner(self) -> Pawn {
        self.inner
    }

    // Creates WasmPawn from inner Pawn
    fn from_inner(inner: Pawn) -> WasmPawn {
        WasmPawn { inner }
    }

    // Access inner reference
    fn inner(&self) -> &Pawn {
        &self.inner
    }
}

#[wasm_bindgen(js_class = Pawn)]
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
