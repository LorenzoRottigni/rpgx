# `Map`

A `Map` represents a 2D game world composed of multiple stacked [`Layer`]s. Each layer contributes visual or logical elements like collision, decoration, triggers, or interactive objects. Layers are ordered and processed in stack order, with later layers drawn or evaluated after earlier ones.

## Fields

- `name: String`:  
  A human-readable identifier for the map.

- `layers: Vec<Layer>`:  
  A stack of layers, each providing overlays of tiles, effects, and logic.

- `spawn: Coordinates`:  
  The default coordinate where a player or pawn begins.

## Key Methods

### `new(name, layers, spawn) -> Self`

Creates a new map with a given name, initial layers, and spawn point. Layers may be empty.

---

### `compose(name, maps, layers, spawn) -> Self`

Composes a new map by merging multiple other maps, each placed at a specified top-left coordinate. This is useful for building larger maps from reusable pieces.

- `maps`: A list of `(Map, Coordinates)` tuples to place on the new map.
- `layers`: Additional layers to stack on top after merging.
- `spawn`: Spawn point for the new map.

---

### `load_layer(layer: Layer)`

Loads a new `Layer` into the map. If the new layer would exceed the bounds of the existing map, all existing layers are offset accordingly to make room.

---

### `layers_by_name() -> IndexMap<String, Layer>`

Returns a lookup map of layer names to their corresponding `Layer` objects.

---

### `merge_at(other: &Map, top_left: Coordinates, spawn: Option<Coordinates>)`

Merges another map into the current one. All layers in the other map are offset by `top_left` and added to the current map. Optionally, a new spawn point can be set.

---

### `duplicate_to_the(direction: Direction, spawn: Option<Coordinates>)`

Duplicates the current map adjacent to itself in a specified `Direction`. Useful for procedurally extending the world.

---

### `move_allowed(target: Coordinates) -> bool`

Checks if movement is allowed at a given coordinate.

- A coordinate is allowed if:
  - At least one layer contains a tile at that position.
  - No layer at that position has a tile with a blocking effect.

---

### `get_shape() -> Shape`

Returns the overall bounding `Shape` that encloses all layers.

---

### `get_tiles_at(pointer: Coordinates) -> Vec<Tile>`

Returns all tiles located at a specific coordinate from all layers.

---

### `get_effects_at(pointer: Coordinates) -> Vec<Effect>`

Returns all tile effects applied at the given coordinate.

---

### `get_actions_at(pointer: Coordinates) -> Vec<u32>`

Returns all action IDs defined at a given coordinate.

## Notes

- The `Map` is primarily a composition container — individual layers define how tiles and effects behave.
- All tile interactions (movement, collision, effect resolution) are driven by the layered structure.
- The `Map` system supports rich dynamic composition through `compose`, `merge_at`, and `duplicate_to_the`, making it suitable for procedurally generated or modular tile worlds.

---
See also: [`Layer`](layer.md), [`Tile`](tile.md), [`Effect`](effect.md)
