use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmEffect, WasmSelector};

/// Represents a mask that can be applied to a tile or an UI element.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmMask {
    name: String,
    effect: WasmEffect,
    selector: WasmSelector,
}

impl WasmMask {
    /// Creates a `WasmMask` from a native `rpgx::prelude::Mask`.
    pub fn from_native(mask: rpgx::prelude::Mask) -> Self {
        Self {
            name: mask.name,
            effect: WasmEffect::from_native(mask.effect),
            selector: WasmSelector::from_native(mask.selector),
        }
    }

    /// Converts the `WasmMask` instance to a native `rpgx::prelude::Mask`.
    pub fn to_native(&self) -> rpgx::prelude::Mask {
        rpgx::prelude::Mask {
            name: self.name.clone(),
            effect: self.effect.to_native(),
            selector: self.selector.to_native(),
        }
    }
    /// Creates a `WasmMask` from a JavaScript object.

    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Mask.name must be a string"))?;
        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = WasmEffect::from_js_value(&effect_js)?;
        let selector_js = Reflect::get(value, &JsValue::from_str("selector"))?;
        let selector = WasmSelector::from_js_value(&selector_js)?;

        Ok(Self {
            name,
            effect,
            selector,
        })
    }

    /// Converts the `WasmMask` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from(self.name.clone())).unwrap();
        Reflect::set(&obj, &JsValue::from_str("effect"), &self.effect.to_js_value()).unwrap();
        Reflect::set(&obj, &JsValue::from_str("selector"), &self.selector.to_js_value()).unwrap();
        obj.into()
    }
}


#[wasm_bindgen]
impl WasmMask {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, effect: WasmEffect, selector: WasmSelector) -> Self {
        Self {
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
    pub fn effect(&self) -> WasmEffect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: WasmEffect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn selector(&self) -> WasmSelector {
        self.selector.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_selector(&mut self, selector: WasmSelector) {
        self.selector = selector;
    }
}

