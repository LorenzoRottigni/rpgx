# Scene Module

The `Scene` module defines the `Scene` struct and associated methods to manage an RPG game scene involving a map and a pawn. It provides functionality for loading pawns, moving pawns across the map, and computing paths with movement restrictions.

---

## Overview

A `Scene` represents an interactive game environment composed of:

- A **name** identifying the scene.
- A **map** (`Map`), containing multiple layers of tiles with terrain, actions, and effects.
- An optional **pawn** (`Pawn`), representing the player or entity moving on the map.

The `Scene` supports loading pawns at default or custom locations, walking along computed paths, stepping in specific directions, and querying movement feasibility.

---

## Struct: `Scene`

```rust
use rpgx::prelude::{Map,Pawn};

#[derive(Clone)]
pub struct Scene {
    pub name: String,
    pub map: Map,
    pub pawn: Option<Pawn>,
}
```

- `name`: Human-readable scene identifier.
- `map`: The game `Map` defining terrain and layout.
- `pawn`: Optional movable entity within the scene.

---

## Methods

### Creation

- `new(name: String, map: Map, pawn: Option<Pawn>) -> Self`  
  Constructs a new scene with the given name, map, and optional pawn.

### Pawn Management

- `load_pawn(&mut self, texture_id: u32)`  
  Loads a pawn at the map's default spawn location using the provided texture ID.

- `load_pawn_at(&mut self, pawn: Pawn)`  
  Loads a pawn at a specified location overriding the default spawn.

### Movement

- `walk_to(&mut self, target_position: Coordinates) -> Result<Tile, RPGXError>` (async)  
  Attempts to walk the pawn to a target coordinate by finding and following a path step-by-step.

- `step_to(&mut self, direction: Direction) -> Result<Tile, RPGXError>`  
  Moves the pawn one step in the specified direction if possible.

- `move_to(&mut self, target_position: Coordinates) -> Result<Tile, RPGXError>`  
  Moves the pawn directly to the specified coordinate if the tile is accessible.

- `steps_to(&self, target_position: Coordinates) -> Result<Vec<Coordinates>, RPGXError>`  
  Computes and returns a list of steps to reach the target position from the pawn’s current location.

---

## Errors

The movement methods return `RPGXError` variants indicating failures such as:

- `TileNotFound`: The target tile or pawn is missing.
- `TileBlocked`: Movement is blocked by obstacles.
- `PathNotFound`: No valid path exists to the target coordinate.

---

## Testing

The module includes comprehensive unit tests covering:

- Successful and blocked movement scenarios.
- Stepwise movement by direction.
- Pathfinding correctness for `walk_to` and `steps_to`.
- Handling missing pawn or blocked tiles errors.
- Walking failure when no path exists.

---

This design enables a flexible and extensible RPG scene abstraction combining map layout and pawn navigation.
