# Engine

The `Engine` struct represents the core state manager of the RPG system.  
It maintains a timeline of `Scene`s (like a stack of game states or world instances), enabling dynamic transitions and rewinds across gameplay moments.

---

## Fields

- `timeline: Vec<Scene>`  
  A timeline of game scenes, ordered by when they were pushed into the engine.

- `timenow: usize`  
  The current index in the timeline, representing the active scene.

---

## Methods

### `new(scene: Scene) -> Self`

Creates a new engine with the given initial scene.  
The engine starts with one scene on its timeline and the pointer set to it.

---

### `get_active_scene(&self) -> Option<&Scene>`

Returns an immutable reference to the currently active scene, or `None` if the timeline is empty.

---

### `get_active_scene_mut(&mut self) -> Option<&mut Scene>`

Returns a mutable reference to the currently active scene, allowing modifications like pawn movement.

---

### `push_scene(&mut self, scene: Scene)`

Adds a new scene to the end of the timeline and updates the active pointer to it.  
Useful when switching maps, progressing story, or entering sub-areas.

---

### `pop_scene(&mut self)`

Removes the last scene from the timeline if there is more than one.  
Also updates `timenow` to point to the new last scene.

---

### `rollback_to(&mut self, index: usize)`

Rolls back the timeline to a specified earlier index.  
All scenes after the index are removed, and the pointer is updated.

No-op if the index is out of bounds.

---

### `rewind_to(&mut self, index: usize) -> Result<(), &str>`

Moves the current scene pointer to a past scene in the timeline without removing any entries.  
Returns an error string if the index is invalid.

---

### `get_scene_at(&self, index: usize) -> Option<&Scene>`

Gets an immutable reference to a scene at a specific point in the timeline, useful for review or replay features.

---

## Notes

- The engine timeline is inspired by save-states or undo-redo systems.
- Scenes can be pushed/popped for forward progression or state rollback.
- `Engine` provides centralized control over which scene is currently active.

