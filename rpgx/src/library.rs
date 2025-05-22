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

#[cfg(test)]
pub mod tests {
    use super::*;

    fn dummy_action() {
        println!("Action executed.");
    }

    #[test]
    fn inserts_and_retrieves_texture() {
        let mut lib = ResourceLibrary::new();
        lib.insert_texture("hero", "hero.png".to_string());

        let texture = lib.get_texture("hero");
        assert!(texture.is_some());
        assert_eq!(texture.unwrap(), "hero.png");

        let id = lib.get_key_id("hero");
        let texture_by_id = lib.get_texture_by_id(id);
        assert!(texture_by_id.is_some());
        assert_eq!(texture_by_id.unwrap(), "hero.png");
    }

    #[test]
    fn inserts_and_retrieves_action() {
        let mut lib = ResourceLibrary::new();
        lib.insert_action("jump", dummy_action);

        let action = lib.get_action("jump");
        assert!(action.is_some());

        let id = lib.get_key_id("jump");
        let action_by_id = lib.get_action_by_id(id);
        assert!(action_by_id.is_some());

        // Run action to verify it's valid (prints to console)
        action.unwrap()();
        action_by_id.unwrap()();
    }

    #[test]
    fn returns_none_for_missing_texture() {
        let lib = ResourceLibrary::new();
        assert!(lib.get_texture("nonexistent").is_none());
    }

    #[test]
    fn returns_none_for_invalid_id() {
        let lib = ResourceLibrary::new();
        assert!(lib.get_texture_by_id(999).is_none());
        assert!(lib.get_action_by_id(999).is_none());
    }

    #[test]
    #[should_panic(expected = "Key ID not found. Insert the resource before requesting its ID. missing")]
    fn panics_on_missing_key_id() {
        let lib = ResourceLibrary::new();
        lib.get_key_id("missing");
    }

    #[test]
    fn maintains_consistent_ids() {
        let mut lib = ResourceLibrary::new();
        lib.insert_texture("item", "item.png".to_string());
        lib.insert_action("interact", dummy_action);

        let id_item = lib.get_key_id("item");
        let id_interact = lib.get_key_id("interact");

        assert_ne!(id_item, id_interact);
        assert_eq!(lib.get_texture_by_id(id_item).unwrap(), "item.png");
        assert!(lib.get_action_by_id(id_interact).is_some());
    }
}
