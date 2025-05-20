use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Effect {
    texture: Option<String>,
    block: bool,
    group: bool,
}

impl Effect {
    pub fn to_native(&self) -> rpgx::prelude::Effect {
        rpgx::prelude::Effect {
            texture: self.texture.clone(),
            block: self.block,
            group: self.group,
            ..Default::default()
        }
    }
}

#[wasm_bindgen]
impl Effect {
    #[wasm_bindgen(constructor)]
    pub fn new(texture: Option<String>, block: bool, group: bool) -> Effect {
        Effect {
            texture,
            block,
            group,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn texture(&self) -> Option<String> {
        self.texture.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture(&mut self, texture: Option<String>) {
        self.texture = texture;
    }

    #[wasm_bindgen(getter)]
    pub fn block(&self) -> bool {
        self.block
    }

    #[wasm_bindgen(setter)]
    pub fn set_block(&mut self, block: bool) {
        self.block = block;
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> bool {
        self.group
    }

    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, group: bool) {
        self.group = group;
    }
}

impl Effect {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let texture = Reflect::get(value, &JsValue::from_str("texture"))?.as_string();
        let block = Reflect::get(value, &JsValue::from_str("block"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.block must be a bool"))?;
        let group = Reflect::get(value, &JsValue::from_str("group"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.group must be a bool"))?;

        Ok(Effect {
            texture,
            block,
            group,
        })
    }
}
