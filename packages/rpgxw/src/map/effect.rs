use js_sys::Reflect;
use rpgx::prelude::Selector;
use wasm_bindgen::prelude::*;

use crate::prelude::WasmSelector;

/// Visual and interactive properties applied to a [`super::tile::Tile`] or an UI element
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmEffect {
    texture_id: Option<i32>,
    action_id: Option<i32>,
    block: bool,
    group: bool,
    shrink: Option<WasmSelector>,
}

impl WasmEffect {
    /// Creates a new `WasmEffect` instance from a native RPGX effect.
    pub fn from_native(effect: rpgx::prelude::Effect) -> Self {
        Self {
            texture_id: effect.texture_id,
            action_id: effect.action_id,
            block: effect.block,
            group: effect.group,
            shrink: effect
                .shrink
                .map(|s| WasmSelector::from_native(Selector::Block(s))),
        }
    }

    /// Converts the `WasmEffect` instance to a native RPGX effect.
    pub fn to_native(&self) -> rpgx::prelude::Effect {
        rpgx::prelude::Effect {
            texture_id: self.texture_id,
            action_id: self.action_id,
            block: self.block,
            group: self.group,
            ..Default::default()
        }
    }

    /// Creates a new `WasmEffect` instance from a JavaScript object.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let texture_id: Option<i32> = Reflect::get(value, &JsValue::from_str("texture_id"))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|n| n as i32);
        let action_id: Option<i32> = Reflect::get(value, &JsValue::from_str("action_id"))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|n| n as i32);
        let block = Reflect::get(value, &JsValue::from_str("block"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.block must be a bool"))?;
        let group = Reflect::get(value, &JsValue::from_str("group"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.group must be a bool"))?;
        let shrink_js = Reflect::get(value, &JsValue::from_str("shrink"))?;

        let shrink = if shrink_js.is_null() || shrink_js.is_undefined() {
            None
        } else if shrink_js.is_object() {
            Some(WasmSelector::from_js_value(&shrink_js)?)
        } else {
            return Err(JsValue::from_str(
                "Effect.shrink must be an object, null or undefined",
            ));
        };

        Ok(Self {
            texture_id,
            action_id,
            block,
            group,
            shrink,
        })
    }

    /// Converts the `WasmEffect` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let texture_jsvalue = match &self.texture_id {
            Some(s) => JsValue::from_f64(*s as f64),
            None => JsValue::NULL,
        };
        let action_jsvalue = match &self.action_id {
            Some(s) => JsValue::from_f64(*s as f64),
            None => JsValue::NULL,
        };
        Reflect::set(&obj, &JsValue::from_str("texture_id"), &texture_jsvalue).unwrap();
        Reflect::set(&obj, &JsValue::from_str("action_id"), &action_jsvalue).unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("block"),
            &JsValue::from(self.block),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("group"),
            &JsValue::from(self.group),
        )
        .unwrap();
        if let Some(shrink) = &self.shrink {
            Reflect::set(&obj, &JsValue::from_str("shrink"), &shrink.to_js_value()).unwrap();
        } else {
            Reflect::set(&obj, &JsValue::from_str("shrink"), &JsValue::NULL).unwrap();
        }
        obj.into()
    }
}

#[wasm_bindgen]
impl WasmEffect {
    #[wasm_bindgen(constructor)]
    pub fn new(
        texture_id: Option<i32>,
        action_id: Option<i32>,
        block: bool,
        group: bool,
        shrink: Option<WasmSelector>,
    ) -> Self {
        Self {
            texture_id,
            action_id,
            block,
            group,
            shrink,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn texture_id(&self) -> Option<i32> {
        self.texture_id
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture_id(&mut self, texture_id: Option<i32>) {
        self.texture_id = texture_id;
    }

    #[wasm_bindgen(getter)]
    pub fn action_id(&self) -> Option<i32> {
        self.action_id
    }

    #[wasm_bindgen(setter)]
    pub fn set_action_id(&mut self, action_id: Option<i32>) {
        self.action_id = action_id;
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

    #[wasm_bindgen(getter)]
    pub fn shrink(&self) -> Option<WasmSelector> {
        self.shrink.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_shrink(&mut self, shrink: Option<WasmSelector>) {
        self.shrink = shrink;
    }
}
