use std::collections::HashMap;

#[derive(Clone)]
pub enum Resource {
    Texture(String),
    Action(fn()),
}

#[derive(Default, Clone)]
pub struct ResourceLibrary {
    data: HashMap<&'static str, Resource>,
    key_ids: HashMap<&'static str, i32>,
    id_keys: HashMap<i32, &'static str>,
    next_id: i32,
}

impl ResourceLibrary {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            key_ids: HashMap::new(),
            id_keys: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn insert_texture(&mut self, key: &'static str, texture: String) {
        self.data.insert(key, Resource::Texture(texture));
        self.assign_id(key);
    }

    pub fn insert_action(&mut self, key: &'static str, action: fn()) {
        self.data.insert(key, Resource::Action(action));
        self.assign_id(key);
    }

    fn assign_id(&mut self, key: &'static str) {
        if !self.key_ids.contains_key(key) {
            let id = self.next_id;
            self.key_ids.insert(key, id);
            self.id_keys.insert(id, key);
            self.next_id += 1;
        }
    }

    pub fn get_texture(&self, key: &str) -> Option<&String> {
        match self.data.get(key) {
            Some(Resource::Texture(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_texture_by_id(&self, id: i32) -> Option<&String> {
        self.id_keys.get(&id).and_then(|&key| {
            match self.data.get(key) {
                Some(Resource::Texture(s)) => Some(s),
                _ => None,
            }
        })
    }

    pub fn get_action(&self, key: &str) -> Option<fn()> {
        match self.data.get(key) {
            Some(Resource::Action(f)) => Some(*f),
            _ => None,
        }
    }

    pub fn get_action_by_id(&self, id: i32) -> Option<fn()> {
        self.id_keys.get(&id).and_then(|&key| {
            match self.data.get(key) {
                Some(Resource::Action(f)) => Some(*f),
                _ => None,
            }
        })
    }

    pub fn get_key_id(&self, key: &'static str) -> i32 {
        *self.key_ids
            .get(key)
            .expect(format!("Key ID not found. Insert the resource before requesting its ID. {}", key).as_str())
    }
}
