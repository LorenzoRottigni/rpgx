use crate::prelude::Scene;

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

    /// Get a mutable reference to the currently active scene.
    pub fn get_active_scene(&self) -> Option<&Scene> {
        self.timeline.get(self.timenow)
    }

    pub fn get_active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.timeline.get_mut(self.timenow)
    }

    /// Push a new scene to the timeline and move the pointer to it.
    pub fn push_scene(&mut self, scene: Scene) {
        self.timeline.push(scene);
        self.timenow = self.timeline.len() - 1;
    }

    /// Pop the last scene from the timeline if there's more than one.
    pub fn pop_scene(&mut self) {
        if self.timeline.len() > 1 {
            self.timeline.pop();
            self.timenow = self.timeline.len() - 1;
        }
    }

    /// Roll back to a specific timenow
    pub fn rollback_to(&mut self, index: usize) {
        if index < self.timeline.len() {
            self.timeline.truncate(index + 1);
            self.timenow = self.timeline.len() - 1;
        }
    }

    /// Rewind to a specific point in the timeline without truncating it.
    pub fn rewind_to(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.timeline.len() {
            self.timenow = index;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn get_scene_at(&self, index: usize) -> Option<&Scene> {
        self.timeline.get(index)
    }
}
