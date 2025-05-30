use crate::prelude::Scene;

/// RPG engine providing [`Pawn`] movement computation across the [`Map`].
#[derive(Clone)]
pub struct Engine {
    /// Timeline of scene states over time.
    pub timeline: Vec<Scene>,
    /// Current index in the timeline (pointer to active scene).
    timenow: usize,
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

    /// Roll back N steps and replace the current future with a new branch.
    pub fn rollback_by(&mut self, steps: usize) {
        if steps > self.timenow {
            self.timenow = 0;
        } else {
            self.timenow -= steps;
        }
        self.timeline.truncate(self.timenow + 1);
    }

    /// Rewind the timeline pointer backward by N steps without removing states.
    pub fn rewind_by(&mut self, steps: usize) {
        if steps > self.timenow {
            self.timenow = 0;
        } else {
            self.timenow -= steps;
        }
    }

    /// Roll back to a specific index and branch with a new scene from there.
    pub fn rollback_to(&mut self, index: usize, new_scene: Scene) {
        if index < self.timeline.len() {
            self.timeline.truncate(index + 1);
            self.timeline.push(new_scene);
            self.timenow = self.timeline.len() - 1;
        }
    }

    /// Rewind to a specific point in the timeline without truncating it.
    pub fn rewind_to(&mut self, index: usize) {
        if index < self.timeline.len() {
            self.timenow = index;
        }
    }

    /// Permanently travel back to a previous scene, truncating future history.
    pub fn time_travel_to(&mut self, index: usize) {
        if index < self.timeline.len() {
            self.timeline.truncate(index + 1);
            self.timenow = index;
        }
    }
}
