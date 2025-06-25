# `Map`

A `Map` represents a game level or region composed of multiple [`Layer`](layer.md)s. Each layer applies [`Mask`](mask.md)s that define effects like collisions, textures, or actions. A map also includes a name identifier and a default spawn location.

Maps can be composed, shifted, or merged to build larger environments dynamically.

---

## Fields

### `name: String`

The name or identifier of the map. Useful for logging, editor tools, or dynamic selection.

---

### `layers: Vec<Layer>`

A list of stacked [`Layer`](layer.md)s. These define all effects applied to the map, such as collision logic, visual rendering, or event triggers.

---

### `spawn: Coordinates`

The default spawn position for players, pawns, or other entities when the map is loaded.

---

## Methods

### `Map::new(name: String, layers: Vec<Layer>, spawn: Coordinates) -> Self`

Creates a new map with a name, a list of layers, and a spawn position.

```rust
use rpgx::prelude::*;

let map = Map::new(
    "dungeon".into(),
    vec![],
    Coordinates::new(0, 0),
);
```

---

### `fn get_shape(&self) -> Shape`

Returns the bounding [`Shape`](shape.md) that encloses all layers.

```rust
let bounds = map.get_shape();
```

---

### `fn contains(&self, coord: &Coordinates) -> bool`

Returns `true` if any layer in the map contains the specified coordinate.

---

### `fn is_blocking_at(&self, coord: &Coordinates) -> bool`

Returns `true` if any layer contains a blocking effect at the specified position.

```rust
if map.is_blocking_at(&Coordinates::new(3, 4)) {
    // prevent movement
}
```

---

### `fn get_actions_at(&self, coord: &Coordinates) -> Vec<u32>`

Returns a list of action IDs applied at the specified tile across all layers.

```rust
let actions = map.get_actions_at(&Coordinates::new(1, 1));
```

---

### `Map::compose(name, maps, layers, spawn) -> Self`

Builds a new `Map` by merging multiple maps (with positional offsets), adding new layers, and setting a spawn point.

```rust
let merged = Map::compose(
    "composed_map".into(),
    vec![(map1, Coordinates::new(0, 0)), (map2, Coordinates::new(10, 0))],
    vec![some_extra_layer],
    Coordinates::new(0, 0),
);
```

---

### `fn load_layer(&mut self, layer: Layer)`

Adds a new layer to the map, offsetting existing layers if needed to make space.

> Useful when dynamically layering content (e.g. loading a dungeon section on the fly).

---

### `fn layers_by_name(&self) -> IndexMap<String, Layer>`

Returns a map from each layer’s name to its [`Layer`](layer.md) object. Useful for quickly accessing specific layers.

```rust
let visual_layer = map.layers_by_name().get("visuals");
```

---

### `fn merge_at(&mut self, other: &Map, top_left: Coordinates, spawn: Option<Coordinates>)`

Merges another `Map` into this one, offsetting its layers by `top_left`.

If a `spawn` is provided, updates the current map's spawn point.

---

### `fn duplicate_to_the(&mut self, direction: Direction, spawn: Option<Coordinates>)`

Clones and attaches this map in the specified `Direction` (e.g., `Right`, `Down`) to itself. Useful for building tile-based infinite maps or test grids.

```rust
map.duplicate_to_the(Direction::Right, None);
```

---

## Usage Example

```rust
use rpgx::prelude::*;

let blocked = vec![Coordinates::new(1, 1)];
let map = Map::new(
    "sample".into(),
    vec![Layer::new(
        "blocking".into(),
        vec![
            Mask::new(
                "block_1".into(),
                vec![Rect::from_xywh(1, 1, 1, 1)],
                vec![Effect::Block(Rect::from_xywh(1, 1, 1, 1))],
            )
        ],
        1,
    )],
    Coordinates::new(0, 0),
);

assert!(map.is_blocking_at(&Coordinates::new(1, 1)));
```

---

## Design Notes

- `Map` is a high-level composition of effects, built from layers and masks.
- It supports composability via `compose`, `merge_at`, and `duplicate_to_the`, enabling procedural and modular map generation.
- Layer order (Z-index) determines render or processing priority.
- Spawn points allow maps to define default entry locations for game entities.

---

## See Also

- [`Layer`](layer.md): A stackable collection of masks with z-order.
- [`Mask`](mask.md): A group of [`Rect`](rect.md)s with attached [`Effect`](effect.md)s.
- [`Effect`](effect.md): Modifiers like blocking, texture, and action.
- [`Rect`](rect.md), [`Shape`](shape.md), [`Coordinates`](coordinates.md), [`Delta`](delta.md)
