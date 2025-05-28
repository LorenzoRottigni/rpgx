use js_sys::Function;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

enum Resource {
    Texture(String),
    Action(Closure<dyn Fn()>),
}

#[wasm_bindgen]
pub struct ResourceLibrary {
    data: HashMap<String, Resource>,
    key_ids: HashMap<String, i32>,
    id_keys: HashMap<i32, String>,
    next_id: i32,
}

#[wasm_bindgen]
impl ResourceLibrary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            key_ids: HashMap::new(),
            id_keys: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_key_id(&mut self, key: String) -> i32 {
        if let Some(id) = self.key_ids.get(&key) {
            *id
        } else {
            let id = self.next_id;
            self.key_ids.insert(key.clone(), id);
            self.id_keys.insert(id, key);
            self.next_id += 1;
            id
        }
    }

    pub fn insert_texture(&mut self, key: String, texture: String) {
        self.data.insert(key.clone(), Resource::Texture(texture));
        self.get_key_id(key);
    }

    pub fn insert_action(&mut self, key: String, action: &Function) {
        let action_cloned = action.clone();
        let closure = Closure::wrap(Box::new(move || {
            let _ = action_cloned.call0(&JsValue::NULL);
        }) as Box<dyn Fn()>);

        self.data.insert(key.clone(), Resource::Action(closure));
        self.get_key_id(key);
    }

    pub fn get_texture(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(Resource::Texture(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_texture_by_id(&self, id: i32) -> Option<String> {
        self.id_keys
            .get(&id)
            .and_then(|key| match self.data.get(key) {
                Some(Resource::Texture(s)) => Some(s.clone()),
                _ => None,
            })
    }

    pub fn get_action_by_id(&self, id: i32) -> JsValue {
        if let Some(key) = self.id_keys.get(&id) {
            if let Some(Resource::Action(closure)) = self.data.get(key) {
                return closure.as_ref().clone();
            }
        }
        JsValue::UNDEFINED
    }

    pub fn call_action(&self, key: String) {
        if let Some(Resource::Action(closure)) = self.data.get(&key) {
            let _ = closure
                .as_ref()
                .unchecked_ref::<Function>()
                .call0(&JsValue::NULL);
        }
    }

    pub fn call_action_by_id(&self, id: i32) {
        if let Some(key) = self.id_keys.get(&id) {
            if let Some(Resource::Action(closure)) = self.data.get(key) {
                let _ = closure
                    .as_ref()
                    .unchecked_ref::<Function>()
                    .call0(&JsValue::NULL);
            }
        }
    }

    pub fn get_key_by_id(&self, id: i32) -> Option<String> {
        self.id_keys.get(&id).cloned()
    }
}
