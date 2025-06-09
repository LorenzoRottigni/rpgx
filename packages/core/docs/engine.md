# Engine

The `Engine` struct manages the timeline of RPG scenes and provides control over scene progression and state history.

## Overview

An `Engine` maintains a timeline of `Scene` states representing the game world at different points in time. It allows navigation through this timeline to support features such as undo/redo, rollback, and branching storylines.

## Fields

- `timeline: Vec<Scene>`  
  Stores the sequence of `Scene` states over time.

- `timenow: usize`  
  Index pointer to the currently active scene in the timeline.

## Methods

### `new(scene: Scene) -> Self`

Creates a new engine starting with the provided initial scene.

### `get_active_scene(&self) -> Option<&Scene>`

Returns a reference to the currently active scene, or `None` if the timeline is empty.

### `get_active_scene_mut(&mut self) -> Option<&mut Scene>`

Returns a mutable reference to the currently active scene.

### `push_scene(&mut self, scene: Scene)`

Adds a new scene to the timeline and moves the pointer to this new scene.

### `pop_scene(&mut self)`

Removes the most recent scene from the timeline if there is more than one scene, moving the pointer back accordingly.

### `rollback_to(&mut self, index: usize)`

Rolls back the timeline to a specific index, truncating any scenes after that point and updating the pointer.

### `rewind_to(&mut self, index: usize) -> Result<(), &'static str>`

Moves the pointer to a specific index in the timeline without truncating it. Returns an error if the index is out of bounds.

### `get_scene_at(&self, index: usize) -> Option<&Scene>`

Returns a reference to the scene at a given index, if it exists.
