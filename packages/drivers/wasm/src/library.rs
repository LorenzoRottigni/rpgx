use std::{any::Any, collections::HashMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLibrary {
    inner: Library<Box<dyn Any>>,
}

#[wasm_bindgen]
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

    #[wasm_bindgen]
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

    #[wasm_bindgen]
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

    #[wasm_bindgen]
    pub fn get_id(&self, key: &str) -> Option<u32> {
        self.inner.get_id(key)
    }

    #[wasm_bindgen]
    pub fn get_key(&self, id: u32) -> Option<String> {
        self.inner.get_key(id).map(|s| s.to_string())
    }
}

// Original Library type
pub struct Library<V> {
    data: HashMap<&'static str, V>,
    key_to_id: HashMap<&'static str, u32>,
    id_to_key: HashMap<u32, &'static str>,
    next_id: u32,
}

impl<V> Library<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            key_to_id: HashMap::new(),
            id_to_key: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, key: &'static str, value: V) {
        if !self.key_to_id.contains_key(&key) {
            let id = self.next_id;
            self.key_to_id.insert(key, id);
            self.id_to_key.insert(id, key);
            self.next_id += 1;
        }
        self.data.insert(key, value);
    }

    pub fn get_by_key(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_by_id(&self, id: u32) -> Option<&V> {
        self.id_to_key.get(&id).and_then(|key| self.data.get(key))
    }

    pub fn get_id(&self, key: &str) -> Option<u32> {
        self.key_to_id.get(key).copied()
    }

    pub fn get_key(&self, id: u32) -> Option<&'static str> {
        self.id_to_key.get(&id).copied()
    }
}
