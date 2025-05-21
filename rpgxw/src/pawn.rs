use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::tile::Tile;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pawn {
    tile: Tile,
    texture_id: i32,
}

#[wasm_bindgen]
impl Pawn {
    #[wasm_bindgen(constructor)]
    pub fn new(tile: Tile, texture_id: i32) -> Pawn {
        Pawn { tile, texture_id }
    }

    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> Tile {
        self.tile.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, tile: Tile) {
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

impl Pawn {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
    let tile_js = Reflect::get(value, &JsValue::from_str("tile"))?;
    let tile = Tile::from_js_value(&tile_js)?;

    let texture_id = Reflect::get(value, &JsValue::from_str("texture_id"))?
        .as_f64()
        .ok_or_else(|| JsValue::from_str("Pawn.texture_id must be a number"))? as i32;

    Ok(Pawn { tile, texture_id })
}

    pub fn from_native(pawn: rpgx::prelude::Pawn) -> Self {
        Pawn {
            tile: Tile::from_native(pawn.tile),
            texture_id: pawn.texture_id,
        }
    }

    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("tile"), &self.tile.to_js_value()).unwrap();
        Reflect::set(&obj, &JsValue::from_str("texture_id"), &JsValue::from(self.texture_id)).unwrap();
        obj.into()
    }

    pub fn to_native(&self) -> rpgx::prelude::Pawn {
        rpgx::prelude::Pawn {
            tile: self.tile.to_native(),
            texture_id: self.texture_id.clone(),
        }
    }
}
