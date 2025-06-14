use crate::prelude::Scene;

pub mod pawn;
pub mod scene;

#[doc = include_str!("../../docs/engine.md")]
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
