# `Mask`

A `Mask` defines a named collection of [`Rect`](rect.md)s that share a common set of [`Effect`](effect.md)s. It is primarily used to apply reusable patterns—like textures, blocking regions, or scripted behaviors—across different map areas in the RPGX engine.

---

## Fields

### `name: String`

A descriptive identifier for the mask. Useful for debugging, editor tooling, and logging.

### `tiles: Vec<Rect>`

The rectangular areas covered by the mask.

---

## Methods

### `Mask::new(name: String, areas: Vec<Rect>, effects: Vec<Effect>) -> Self`

Creates a new `Mask` from a list of `Rect`s and associated `Effect`s. Each `Rect` will receive the specified effects.

```rust
use rpgx::prelude::*;

let mask = Mask::new(
    "trap_zones".to_string(),
    vec![
        Rect::new(Coordinates::new(2, 2), Shape::new(1, 1)),
        Rect::new(Coordinates::new(4, 4), Shape::new(2, 2)),
    ],
    vec![Effect::Texture(1)],
);
```

> This mask applies texture `1` to the 1x1 area at `(2,2)` and the 2x2 block starting at `(4,4)`.

---

### `fn offset(&mut self, delta: Delta)`

Applies an in-place positional shift to all `Rect`s in the mask and to any region described within its `Effect`s.

```rust
mask.offset(Delta::new(1, 1));
```

> Useful when relocating predefined features (e.g. buildings or zones) during map generation or merging.

---

### `fn translate(&self, delta: Delta) -> Self`

Returns a new `Mask` that is offset by the given delta, without modifying the original.

```rust
let shifted = mask.translate(Delta::new(3, -1));
```

---

### `fn get_shape(&self) -> Shape`

Returns the bounding `Shape` (width and height) that encloses all `Rect`s in the mask.

```rust
let bounds = mask.get_shape();
```

> This is computed from `(0, 0)` as origin and represents the extent of the mask.

---

### `fn contains(&self, coord: Coordinates) -> bool`

Returns `true` if any of the mask’s `Rect`s contains the given coordinate.

```rust
assert!(mask.contains(Coordinates::new(4, 5)));
```

---

### `fn is_blocking_at(&self, target: &Coordinates) -> bool`

Returns `true` if the mask contains the given coordinate and one of its `Effect`s marks it as blocking.

---

### `fn get_actions(&self) -> Vec<u32>`

Returns a list of all `Action` effect IDs applied within the mask.

```rust
let actions = mask.get_actions();
// e.g., vec![10, 42]
```

---

### `fn get_texture(&self) -> Option<u32>`

Returns the texture ID applied to the mask, if any.

---

### `fn get_render(&self) -> Option<u32>`

Returns the render callback ID associated with the mask, if any.

---

## Usage Example

```rust
use rpgx::prelude::*;

let mask = Mask::new(
    "building".to_string(),
    vec![
        Rect::new(Coordinates::new(0, 0), Shape::new(4, 6)),
    ],
    vec![
        Effect::Block(Rect::new(Coordinates::new(1, 1), Shape::new(3, 5))),
        Effect::Texture(2),
    ]
);

// Checks
assert!(mask.contains(Coordinates::new(1, 1)));
assert!(mask.is_blocking_at(&Coordinates::new(3, 3)));
assert!(!mask.is_blocking_at(&Coordinates::new(0, 0)));
```

> This mask defines a `4x6` building area with texture ID `2`. The central area from `(1,1)` to `(3,5)` is blocking, simulating a solid building core while leaving edges walkable.

---

## Design Notes

- `Mask` allows defining reusable and composable building blocks for maps, layers, and logic.
- `Rect`s inside a mask can be arbitrarily positioned; they are not constrained to a uniform grid.
- Effects are applied uniformly to all regions in the mask unless scoped more narrowly inside each `Effect`.
- Use `offset` and `translate` to reposition masks dynamically, especially during procedural generation.

---

## See Also

- [`Rect`](rect.md): Defines a rectangular region.
- [`Effect`](effect.md): Controls behavior, appearance, and interaction of tiles.
- [`Layer`](layer.md): Often constructed from one or more masks.
- [`Coordinates`](coordinates.md): Represents a point in 2D space.
- [`Shape`](shape.md): Represents width and height.
- [`Delta`](delta.md): Represents a 2D vector offset.
