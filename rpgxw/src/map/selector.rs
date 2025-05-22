use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use rpgx::prelude::{Selector};

use crate::prelude::WasmCoordinates;

/// Represents a selector in the RPGX engine, which can be a single coordinate or a block of coordinates.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmSelector {
    start: WasmCoordinates,
    end: WasmCoordinates,
}

impl WasmSelector {
    /// Creates a new `WasmSelector` instance from the given start and end coordinates.
    pub fn from_native(selector: Selector) -> Self {
        match selector {
            Selector::Single(coord) => Self {
                start: WasmCoordinates::from_native(coord),
                end: WasmCoordinates::from_native(coord),
            },
            Selector::Block((start, end)) => Self {
                start: WasmCoordinates::from_native(start),
                end: WasmCoordinates::from_native(end),
            },
            _ => panic!("Filter variant can't be constructed from JS currently"),
        }
    }

    /// Converts the `WasmSelector` instance to a native RPGX selector.
    pub fn to_native(&self) -> Selector {
        if self.start == self.end {
            Selector::Single(self.start.to_native())
        } else {
            Selector::Block((
                self.start.to_native(),
                self.end.to_native(),
            ))
        }
        // Filter variant can't be constructed from JS currently
    }

    /// Creates a new `WasmSelector` instance from a JavaScript object.
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let start_js = Reflect::get(value, &JsValue::from_str("start"))?;
        let start = WasmCoordinates::from_js_value(&start_js)?;
        let end_js = Reflect::get(value, &JsValue::from_str("end"))?;
        let end = WasmCoordinates::from_js_value(&end_js)?;
        Ok(Self { start, end })
    }

    /// Converts the `WasmSelector` instance to a JavaScript object.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &JsValue::from_str("start"), &self.start.to_js_value()).unwrap();
        Reflect::set(&obj, &JsValue::from_str("end"), &self.end.to_js_value()).unwrap();
        obj.into()
    }
}


#[wasm_bindgen]
impl WasmSelector {
    #[wasm_bindgen(constructor)]
    pub fn new_single(coord: WasmCoordinates) -> Self {
        Self {
            start: coord,
            end: coord,
        }
    }

    #[wasm_bindgen]
    pub fn new_block(start: WasmCoordinates, end: WasmCoordinates) -> Self {
        Self { start, end }
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> WasmCoordinates {
        self.start
    }

    #[wasm_bindgen(setter)]
    pub fn set_start(&mut self, start: WasmCoordinates) {
        self.start = start;
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> WasmCoordinates {
        self.end
    }

    #[wasm_bindgen(setter)]
    pub fn set_end(&mut self, end: WasmCoordinates) {
        self.end = end;
    }
}
