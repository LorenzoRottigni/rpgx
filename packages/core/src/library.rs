use std::collections::HashMap;

pub struct Library<V> {
    data: HashMap<&'static str, V>,
    key_to_id: HashMap<&'static str, i32>,
    id_to_key: HashMap<i32, &'static str>,
    next_id: i32,
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

    // Insert a key-value pair and assign a unique ID if key is new
    pub fn insert(&mut self, key: &'static str, value: V) {
        if !self.key_to_id.contains_key(&key) {
            let id = self.next_id;
            self.key_to_id.insert(key, id);
            self.id_to_key.insert(id, key);
            self.next_id += 1;
        }
        self.data.insert(key, value);
    }

    // Get value by key
    pub fn get_by_key(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }

    // Get value by id
    pub fn get_by_id(&self, id: i32) -> Option<&V> {
        self.id_to_key.get(&id).and_then(|key| self.data.get(key))
    }

    // Get ID by key
    pub fn get_id(&self, key: &str) -> Option<i32> {
        self.key_to_id.get(key).copied()
    }

    // Get key by ID
    pub fn get_key(&self, id: i32) -> Option<&'static str> {
        self.id_to_key.get(&id).copied()
    }
}
