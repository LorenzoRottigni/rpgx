# Map Module

The `Map` module defines the `Map` struct and associated methods for managing game maps composed of multiple layers of tiles. It provides functionality for layering, merging, querying tiles, and handling map composition in an RPG game engine.

---

## Overview

A `Map` represents a game area composed of multiple `Layer`s, each containing tiles arranged in a grid with a defined shape and type (`Base`, `Action`, `Block`, etc.). The map tracks a spawn point (starting coordinates for pawns or players) and supports operations such as merging maps, layering new tile sets, and querying tiles and effects at specific coordinates.

---

## Struct: `Map`

```rust
use rpgx::prelude::{Layer,Coordinates};

pub struct Map {
    pub name: String,
    pub layers: Vec<Layer>,
    pub spawn: Coordinates,
}
```

- `name`: Human-readable identifier for the map.
- `layers`: Collection of `Layer`s representing different aspects of the map (e.g., terrain, actions, blocking).
- `spawn`: Coordinates indicating the default spawn point within the map.

---

## Key Methods

### Creation and Composition

- **`new(name: String, layers: Vec<Layer>, spawn: Coordinates) -> Self`**  
  Creates a new `Map`. If no base layer exists among provided layers, a base layer is automatically generated from the existing layers.

- **`compose(name: String, maps: Vec<(Map, SingleSelector)>, layers: Vec<Layer>, spawn: Coordinates) -> Self`**  
  Constructs a composite map by merging multiple maps and layers. Each map is merged at the specified `SingleSelector` offset.

### Layer Management

- **`load_layer(&mut self, layer: Layer)`**  
  Adds a new layer to the map. If a base layer exists, it will be reshaped to include the new layer's dimensions, offsetting existing layers as necessary. If no base layer exists, a new one is created.

- **`layers_by_name(&self) -> IndexMap<String, Layer>`**  
  Returns a map (ordered dictionary) from layer names to their corresponding layers for easy lookup and merging.

### Map Modification

- **`merge_at(&mut self, other: &Map, top_left: Coordinates, spawn: Option<Coordinates>)`**  
  Merges another map into this one, offsetting the other's layers by `top_left` coordinates. Optionally updates the spawn point.

- **`duplicate_to_the(&mut self, direction: Direction, spawn: Option<Coordinates>)`**  
  Expands the map by duplicating itself in the specified direction (up, down, left, right), merging the duplicate with an offset.

### Tile Queries

- **`is_blocking_at(&self, target: Coordinates) -> bool`**  
  Returns `true` if any layer blocks movement at the given coordinate.

- **`get_shape(&self) -> Shape`**  
  Returns the map's shape, determined by the base layer's shape.

- **`get_base_layer(&self) -> Option<Layer>`**  
  Returns the base layer if one exists.

- **`get_layers_of_type(&self, kind: LayerType) -> Vec<Layer>`**  
  Returns all layers of the specified type.

- **`get_base_tile(&self, pointer: Coordinates) -> Option<Tile>`**  
  Retrieves the tile at the specified coordinates in the base layer.

- **`get_tiles_at(&self, pointer: Coordinates) -> Vec<Tile>`**  
  Retrieves stacked tiles from all layers at the given coordinate.

- **`get_effects_at(&self, pointer: Coordinates) -> Vec<Effect>`**  
  Returns all effects active at the coordinate across all layers.

- **`get_actions_at(&self, pointer: Coordinates) -> Vec<u32>`**  
  Returns all action IDs present at the coordinate from action layers.

---

## Testing

The module includes comprehensive unit tests verifying:

- Creation of maps and layers
- Tile retrieval in all layers
- Layer merging with offset logic
- Blocking and action detection
- Composite map building from multiple maps
