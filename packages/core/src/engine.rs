use crate::prelude::Scene;

#[doc = include_str!("../docs/engine.md")]
#[derive(Clone)]
pub struct Engine {
    /// Timeline of scene states over time.
    pub timeline: Vec<Scene>,
    /// Current index in the timeline (pointer to active scene).
    pub timenow: usize,
}

impl Engine {
    /// Create a new engine starting with an initial scene.
    pub fn new(scene: Scene) -> Self {
        Self {
            timeline: vec![scene],
            timenow: 0,
        }
    }

    /// Get a reference to the currently active scene.
    pub fn get_active_scene(&self) -> Option<&Scene> {
        self.timeline.get(self.timenow)
    }

    /// Get a mutable reference to the currently active scene.
    pub fn get_active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.timeline.get_mut(self.timenow)
    }

    /// Push a new scene to the timeline and move the pointer to it.
    pub fn push_scene(&mut self, scene: Scene) {
        self.timeline.push(scene);
        self.timenow = self.timeline.len() - 1;
    }

    /// Pop the last scene from the timeline if there's more than one.
    /// Updates `timenow` to point to the new last scene.
    pub fn pop_scene(&mut self) {
        if self.timeline.len() > 1 {
            self.timeline.pop();
            self.timenow = self.timeline.len() - 1;
        }
    }

    /// Roll back the timeline to the specified index, truncating all scenes after it.
    /// Updates `timenow` accordingly.
    pub fn rollback_to(&mut self, index: usize) {
        if index < self.timeline.len() {
            self.timeline.truncate(index + 1);
            self.timenow = self.timeline.len() - 1;
        }
    }

    /// Rewind to a specific point in the timeline without truncating.
    /// Returns error if the index is out of range.
    pub fn rewind_to(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.timeline.len() {
            self.timenow = index;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    /// Get a reference to the scene at the specified index.
    pub fn get_scene_at(&self, index: usize) -> Option<&Scene> {
        self.timeline.get(index)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Map, Scene};

    // Helper to create a dummy scene
    fn dummy_scene(name: &str) -> Scene {
        let map = Map::new(name.to_string(), vec![], Coordinates::default());
        Scene::new(name.to_string(), map, None)
    }

    #[test]
    fn test_new_engine() {
        let scene = dummy_scene("start");
        let engine = Engine::new(scene.clone());
        assert_eq!(engine.timeline.len(), 1);
        assert_eq!(engine.timenow, 0);
        assert_eq!(engine.get_active_scene().unwrap().name, scene.name);
    }

    #[test]
    fn test_push_scene() {
        let mut engine = Engine::new(dummy_scene("first"));
        engine.push_scene(dummy_scene("second"));
        assert_eq!(engine.timeline.len(), 2);
        assert_eq!(engine.timenow, 1);
        assert_eq!(engine.get_active_scene().unwrap().name, "second");
    }

    #[test]
    fn test_pop_scene() {
        let mut engine = Engine::new(dummy_scene("first"));
        engine.push_scene(dummy_scene("second"));
        engine.pop_scene();
        assert_eq!(engine.timeline.len(), 1);
        assert_eq!(engine.timenow, 0);
        assert_eq!(engine.get_active_scene().unwrap().name, "first");

        // Pop when only one scene should do nothing
        engine.pop_scene();
        assert_eq!(engine.timeline.len(), 1);
        assert_eq!(engine.timenow, 0);
    }

    #[test]
    fn test_rollback_to() {
        let mut engine = Engine::new(dummy_scene("first"));
        engine.push_scene(dummy_scene("second"));
        engine.push_scene(dummy_scene("third"));

        engine.rollback_to(0);
        assert_eq!(engine.timeline.len(), 1);
        assert_eq!(engine.timenow, 0);
        assert_eq!(engine.get_active_scene().unwrap().name, "first");

        // rollback to out of bounds does nothing
        engine.rollback_to(10);
        assert_eq!(engine.timeline.len(), 1);
    }

    #[test]
    fn test_rewind_to() {
        let mut engine = Engine::new(dummy_scene("first"));
        engine.push_scene(dummy_scene("second"));
        engine.push_scene(dummy_scene("third"));

        assert!(engine.rewind_to(1).is_ok());
        assert_eq!(engine.timenow, 1);
        assert_eq!(engine.get_active_scene().unwrap().name, "second");

        let result = engine.rewind_to(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_scene_at() {
        let mut engine = Engine::new(dummy_scene("one"));
        engine.push_scene(dummy_scene("two"));
        assert_eq!(engine.get_scene_at(0).unwrap().name, "one");
        assert_eq!(engine.get_scene_at(1).unwrap().name, "two");
        assert!(engine.get_scene_at(2).is_none());
    }

    #[test]
    fn test_get_active_scene_mut() {
        let mut engine = Engine::new(dummy_scene("start"));
        {
            let scene_mut = engine.get_active_scene_mut().unwrap();
            assert_eq!(scene_mut.name, "start");
            scene_mut.name = "changed".to_string();
        }
        assert_eq!(engine.get_active_scene().unwrap().name, "changed");
    }
}
 */
