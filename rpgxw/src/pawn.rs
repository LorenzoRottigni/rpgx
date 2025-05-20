use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::tile::Tile;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pawn {
    tile: Tile,
    texture: String,
}

#[wasm_bindgen]
impl Pawn {
    #[wasm_bindgen(constructor)]
    pub fn new(tile: Tile, texture: String) -> Pawn {
        Pawn { tile, texture }
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
    pub fn texture(&self) -> String {
        self.texture.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture(&mut self, texture: String) {
        self.texture = texture;
    }
}

impl Pawn {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let tile_js = Reflect::get(value, &JsValue::from_str("tile"))?;
        let tile = Tile::from_js_value(&tile_js)?;

        let texture = Reflect::get(value, &JsValue::from_str("texture"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Pawn.texture must be a string"))?;

        Ok(Pawn { tile, texture })
    }

    pub fn from_native(pawn: rpgx::prelude::Pawn) -> Self {
        Pawn {
            tile: Tile::from_native(pawn.tile),
            texture: pawn.texture,
        }
    }

    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("tile"), &self.tile.to_js_value()).unwrap();
        Reflect::set(&obj, &JsValue::from_str("texture"), &JsValue::from_str(&self.texture)).unwrap();
        obj.into()
    }

    pub fn to_native(&self) -> rpgx::prelude::Pawn {
        rpgx::prelude::Pawn {
            tile: self.tile.to_native(),
            texture: self.texture.clone(),
        }
    }
}
