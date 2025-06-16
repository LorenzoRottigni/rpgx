
# Scene

The `Scene` struct represents an active gameplay scene containing a `Map` and a controllable `Pawn`. It acts as the orchestrator for movement logic, pathfinding, and interaction between the pawn and the terrain.

## Fields

- `name: String`  
  A unique identifier for the scene.

- `map: Map`  
  The associated map, which includes multiple layers and defines the terrain, obstacles, and interaction tiles.

- `pawn: Option<Pawn>`  
  The currently active pawn in the scene, if any. Pawns represent entities that can move and interact with the map.

## Usage

A `Scene` is responsible for high-level movement commands like walking to a target, stepping in a direction, and computing path steps.

---

## Methods

### `new(name: String, map: Map, pawn: Option<Pawn>) -> Self`

Creates a new scene with the given name, map, and an optional pawn.

---

### `load_pawn(texture_id: u32)`

Instantiates a new `Pawn` with the given `texture_id` and places it at the default spawn position defined by the map.

---

### `load_pawn_at(pawn: Pawn)`

Loads a given pawn into the scene at its defined coordinates.

---

### `walk_to(&mut self, target_position: Coordinates) -> Result<Coordinates, RPGXError>`

Asynchronously walks the pawn step-by-step to the target coordinates using the shortest computed path.  
Returns the final tile position or an error if movement fails.

Errors:
- `PawnNotFound`: if no pawn is loaded.
- `PathNotFround`: if no path to the target exists.
- `WalkFailed`: if movement to one of the tiles in the path fails.

---

### `step_to(&mut self, direction: Direction) -> Result<Coordinates, RPGXError>`

Attempts to move the pawn one step in the specified direction.  
Returns the new position or an error if movement is blocked or invalid.

Errors:
- `PawnNotFound`: if no pawn is loaded.
- `StepFailed`: if the step results in an invalid coordinate.
- `TileNotWalkable`: if the destination tile is blocked.

---

### `move_to(&mut self, target_position: Coordinates) -> Result<Coordinates, RPGXError>`

Moves the pawn directly to the target tile if movement is allowed.  
Returns the new coordinates or an error if the tile is not walkable.

Errors:
- `PawnNotFound`: if no pawn is loaded.
- `TileNotWalkable`: if the tile is blocked by the map.

---

### `steps_to(&self, target_position: Coordinates) -> Result<Vec<Coordinates>, RPGXError>`

Computes the full path of steps from the current pawn position to the target.  
Returns a vector of `Coordinates` or an error if the pawn is missing or the path can't be found.

Errors:
- `PawnNotFound`: if no pawn is loaded.
- `PathNotFround`: if no valid path exists to the target.

---

## Example
```rust
let mut scene = Scene::new("Overworld".to_string(), map, None);
scene.load_pawn(1);
let path = scene.steps_to(Coordinates::new(5, 5))?;
scene.walk_to(Coordinates::new(5, 5)).await?;
```

---

## Notes

- `Scene` provides convenience methods that delegate to the underlying `Map` for pathfinding.
- It abstracts away path computation and movement, enabling game logic to interact with higher-level APIs.
- Each movement is validated against map constraints like blocked tiles.
