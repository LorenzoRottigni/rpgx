use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::{effect::Effect, selector::Selector};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Mask {
    name: String,
    effect: Effect,
    selector: Selector,
}

#[wasm_bindgen]
impl Mask {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, effect: Effect, selector: Selector) -> Mask {
        Mask {
            name,
            effect,
            selector,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> Effect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn selector(&self) -> Selector {
        self.selector.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }
}

impl Mask {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Mask.name must be a string"))?;
        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = Effect::from_js_value(&effect_js)?;
        let selector_js = Reflect::get(value, &JsValue::from_str("selector"))?;
        let selector = Selector::from_js_value(&selector_js)?;

        Ok(Mask {
            name,
            effect,
            selector,
        })
    }
}

impl Mask {
    pub fn from_native(mask: rpgx::prelude::Mask) -> Self {
        Mask {
            name: mask.name,
            effect: Effect::from_native(mask.effect),
            selector: Selector::from_native(mask.selector),
        }
    }

    pub fn to_native(&self) -> rpgx::prelude::Mask {
        rpgx::prelude::Mask {
            name: self.name.clone(),
            effect: self.effect.to_native(),
            selector: self.selector.to_native(),
        }
    }
}
