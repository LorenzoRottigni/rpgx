use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::prelude::WasmTile;

/// Represents a pawn in the RPGX engine, which can be a character or an object on the map.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmPawn {
    tile: WasmTile,
    texture_id: i32,
}

impl WasmPawn {
    /// Creates a new `WasmPawn` instance from a native RPGX pawn.
    pub fn from_native(pawn: rpgx::prelude::Pawn) -> Self {
        Self {
            tile: WasmTile::from_native(pawn.tile),
            texture_id: pawn.texture_id,
        }
    }

    /// Converts the `WasmPawn` instance to a native RPGX pawn.
    pub fn to_native(&self) -> rpgx::prelude::Pawn {
        rpgx::prelude::Pawn {
            tile: self.tile.to_native(),
            texture_id: self.texture_id.clone(),
        }
    }
    /// Creates a new `WasmPawn` instance from a JavaScript object.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let tile_js = Reflect::get(value, &JsValue::from_str("tile"))?;
        let tile = WasmTile::from_js_value(&tile_js)?;

        let texture_id = Reflect::get(value, &JsValue::from_str("texture_id"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Pawn.texture_id must be a number"))? as i32;

        Ok(Self { tile, texture_id })
    }

    /// Converts the `WasmPawn` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("tile"), &self.tile.to_js_value()).unwrap();
        Reflect::set(&obj, &JsValue::from_str("texture_id"), &JsValue::from(self.texture_id)).unwrap();
        obj.into()
    }
}


#[wasm_bindgen]
impl WasmPawn {
    #[wasm_bindgen(constructor)]
    pub fn new(tile: WasmTile, texture_id: i32) -> Self {
        Self { tile, texture_id }
    }

    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> WasmTile {
        self.tile.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, tile: WasmTile) {
        self.tile = tile;
    }

    #[wasm_bindgen(getter)]
    pub fn texture_id(&self) -> i32 {
        self.texture_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture_id(&mut self, texture_id: i32) {
        self.texture_id = texture_id;
    }
}

