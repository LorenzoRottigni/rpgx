use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Library<V> {
    data: HashMap<String, V>,
    key_to_id: HashMap<String, u32>,
    id_to_key: HashMap<u32, String>,
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

    /// Insert a key-value pair and assign a unique ID if the key is new
    pub fn insert(&mut self, key: impl Into<String>, value: V) {
        let key = key.into();
        if !self.key_to_id.contains_key(&key) {
            let id = self.next_id;
            self.key_to_id.insert(key.clone(), id);
            self.id_to_key.insert(id, key.clone());
            self.next_id += 1;
        }
        self.data.insert(key, value);
    }

    /// Get value by key
    pub fn get_by_key(&self, key: impl Into<String>) -> Option<&V> {
        self.data.get(&key.into())
    }

    /// Get value by ID
    pub fn get_by_id(&self, id: u32) -> Option<&V> {
        self.id_to_key.get(&id).and_then(|key| self.data.get(key))
    }

    /// Get ID by key
    pub fn get_id(&self, key: impl Into<String>) -> Option<u32> {
        self.key_to_id.get(&key.into()).copied()
    }

    /// Get key by ID
    pub fn get_key(&self, id: u32) -> Option<impl Into<String>> {
        self.id_to_key.get(&id).map(|s| s.as_str())
    }
}
