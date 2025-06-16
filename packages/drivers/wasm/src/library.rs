use rpgx::library::Library;
use std::any::Any;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Library)]
pub struct WasmLibrary {
    inner: Library<Box<dyn Any>>,
}

#[wasm_bindgen(js_class = Library)]
impl WasmLibrary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmLibrary {
        WasmLibrary {
            inner: Library::new(),
        }
    }

    #[wasm_bindgen]
    pub fn insert(&mut self, key: &str, value: JsValue) {
        // Safety: converting &str to &'static str by leaking it.
        // You should manage this carefully or use another approach for dynamic lifetimes.
        let static_key: &'static str = Box::leak(key.to_string().into_boxed_str());
        self.inner.insert(static_key, Box::new(value));
    }

    #[wasm_bindgen(js_name = getByKey)]
    pub fn get_by_key(&self, key: &str) -> JsValue {
        match self.inner.get_by_key(key) {
            Some(boxed) => {
                if let Some(s) = boxed.downcast_ref::<JsValue>() {
                    s.clone()
                } else {
                    JsValue::NULL
                }
            }
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(js_name = getById)]
    pub fn get_by_id(&self, id: u32) -> JsValue {
        match self.inner.get_by_id(id) {
            Some(boxed) => {
                if let Some(s) = boxed.downcast_ref::<JsValue>() {
                    s.clone()
                } else {
                    JsValue::NULL
                }
            }
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(js_name = getId)]
    pub fn get_id(&self, key: &str) -> Option<u32> {
        self.inner.get_id(key)
    }

    #[wasm_bindgen(js_name = getKey)]
    pub fn get_key(&self, id: u32) -> Option<String> {
        self.inner.get_key(id).map(|s| s.into())
    }
}
