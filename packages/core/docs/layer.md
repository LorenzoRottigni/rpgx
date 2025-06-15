# Layer

A `Layer` represents a stackable, named collection of [`Mask`](mask.md)s applied over a grid to define visual, interactive, or collision behavior. Layers can be used to separate concerns like base terrain, obstacles, or triggers, and are typically composited in rendering or logic processing using a `z`-index.

---

## Structure

```rust
pub struct Layer {
    pub name: String,
    pub masks: Vec<Mask>,
    pub z: u32,
}
```

### Fields

- **`name: String`**  
  A human-readable identifier, e.g., `"collision"`, `"decorations"`, `"interactables"`.

- **`masks: Vec<Mask>`**  
  A list of [`Mask`](mask.md) objects, each of which contains tiles with applied [`Effect`](effect.md)s.

- **`z: u32`**  
  Z-index used to determine render or evaluation order. Layers with higher `z` appear "on top".

---

## Behavior

### Tile Lookup

```rust
pub fn get_tile_at(&self, pointer: Coordinates) -> Option<Tile>
```

Returns the first tile in the layer that contains the specified `Coordinates`. Useful for querying effects or properties at a location.

### Blocking Detection

```rust
pub fn is_blocking_at(&self, target: &Coordinates) -> bool
```

Returns `true` if any tile in the layer defines a blocking [`Effect`](effect.md) that covers the given `Coordinates`.

### Shape Queries

```rust
pub fn get_shapes(&self) -> Vec<Shape>
pub fn get_shape(&self) -> Shape
```

- `get_shapes`: Returns the bounding shape of each mask individually.
- `get_shape`: Computes a single shape that bounds all tiles in all masks.

### Rendering / Tile Collection

```rust
pub fn render(&self) -> Vec<Tile>
```

Flattens the layer into a list of all its tiles for rendering or processing.

### Offset

```rust
pub fn offset(&mut self, delta: Delta)
```

Applies a positional offset to all masks and their tiles/effects. This is useful when shifting layers in map transformations.

---

## Use Cases

- Stackable collision and decoration layers.
- Trigger zones for actions or animations.
- Visual overlays or path markers.
- Temporarily shifted effect zones (e.g., moving platforms, dynamic hazards).

---

## Example

```rust
let effect = Effect {
    block: Some(Rect::from_xywh(0, 0, 1, 1)),
    ..Default::default()
};
let mask = Mask::new("hazard_zone".into(), vec![Rect::from_xywh(5, 5, 2, 2)], effect);
let layer = Layer::new("hazards".into(), vec![mask], 10);
```

This creates a `hazards` layer with one mask that defines a 2×2 blocking zone starting at (5,5), rendered at z-index 10.

---

## See Also

- [`Tile`](tile.md)
- [`Effect`](effect.md)
- [`Mask`](mask.md)
- [`Map`](map.md)
- [`Rect`](rect.md), [`Coordinates`](coordinates.md), [`Delta`](delta.md), [`Shape`](shape.md)
