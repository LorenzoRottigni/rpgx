# Mask

A `Mask` defines a named collection of [`Tile`](tile.md)s, each occupying a rectangular area and carrying a common [`Effect`](effect.md). Masks are often used to apply behavior or visuals to a group of tiles at once — such as marking certain regions of a map as blocked, interactive, or textured.

## Purpose

The `Mask` abstraction is useful for:
- Applying consistent `Effect`s to multiple `Tile`s.
- Defining shape-based overlays such as danger zones, trap regions, interactive areas, or visual enhancements.
- Applying batch offsets or transformations to rectangular tile groups.
- Merging masks into [`Layer`](layer.md)s or [`Map`](map.md) structures.

## Fields

### `name: String`

A descriptive identifier for the mask. This is primarily used for debugging, editor tooling, or logging purposes.

### `tiles: Vec<Tile>`

The set of tiles that make up the mask. Each tile contains a rectangular `area` and an associated `Effect`.

---

## Methods

### `Mask::new(name: String, areas: Vec<Rect>, effect: Effect) -> Self`

Constructs a new mask from a list of rectangular areas. Each rectangle gets assigned the same `Effect`.

```rust
let effect = Effect {
    block: Some(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
    ..Default::default()
};

let areas = vec![
    Rect::new(Coordinates::new(2, 2), Shape::new(1, 1)),
    Rect::new(Coordinates::new(4, 4), Shape::new(2, 2)),
];

let mask = Mask::new("trap_zones".to_string(), areas, effect);
```

---

### `fn offset(&mut self, delta: Delta)`

Applies a positional offset to every tile's area and its effect’s internal blocking region (if any).

```rust
mask.offset(Delta::new(1, 1)); // moves everything one tile down and right
```

---

### `fn get_shape(&self) -> Shape`

Computes the bounding rectangle (`Shape`) that contains all tiles in the mask. The result's width and height represent the maximum extent from the top-left origin (0, 0).

---

### `fn contains(&self, coord: Coordinates) -> bool`

Returns `true` if any tile in the mask contains the given coordinate.

---

### `fn tile_at(&self, coord: Coordinates) -> Option<&Tile>`

Finds the tile in the mask that contains the given coordinate, if any.

---

## Usage Example

```rust
let mask = Mask::new(
    "block_zone".to_string(),
    vec![
        Rect::new(Coordinates::new(0, 0), Shape::new(2, 2)),
        Rect::new(Coordinates::new(3, 3), Shape::new(1, 1)),
    ],
    Effect {
        block: Some(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
        ..Default::default()
    },
);

assert!(mask.contains(Coordinates::new(1, 1)));
assert!(mask.tile_at(Coordinates::new(3, 3)).is_some());
```

---

## Design Notes

- `Mask` enables composable and reusable logic across layers and maps.
- Tiles in a `Mask` are not automatically aligned in a grid — they can occupy arbitrary positions and sizes.
- Offsetting a mask can be useful when placing pre-built features (like buildings or zones) in different locations on the map.
- This abstraction fits naturally into map editing tools, prefab systems, and procedural generation.

## See Also

- [`Tile`](tile.md): Defines the basic building block of a mask.
- [`Effect`](effect.md): Describes the behavior or appearance attached to each tile.
- [`Layer`](layer.md): Often constructed from one or more masks.
- [`Rect`](rect.md), [`Coordinates`](coordinates.md), [`Shape`](shape.md), [`Delta`](delta.md)
