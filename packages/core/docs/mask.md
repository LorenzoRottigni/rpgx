# Mask

A `Mask` defines a named collection of [`Rect`](rect.md)s with a common set of Effects applied.

## Fields

### `name: String`

A descriptive identifier for the mask. This is primarily used for debugging, editor tooling, or logging purposes.

### `tiles: Vec<Rect>`

The set of tiles that make up the mask.

---

## Methods

### `Mask::new(name: String, areas: Vec<Rect>, effect: Effect) -> Self`

Constructs a new `Mask` from a list of `Tile`s and `Effect`s, each `Tile` will have `Mask` `Effect`s applied.

```rust
use rpgx::prelude::*;

let mask = Mask::new(
    "trap_zones".to_string(),
    vec![
        Rect::new(Coordinates::new(2, 2), Shape::new(1, 1)),
        Rect::new(Coordinates::new(4, 4), Shape::new(2, 2)),
    ];,
    [Effect::texture(1)]
);
```

The rect at 2;2 spanning 1x1 and the rect at 4;4 spanning 2x2 will get the texture with ID 1.

---

### `fn offset(&mut self, delta: Delta)`

Applies a positional offset to every tile's area and its effect’s internal blocking region (if any).
This is designed to manage the merge of several maps with different shapes.

```rust
use rpgx::prelude::*;
mask.offset(Delta::new(1, 1));
```

The previosuly defined mask rects are now shifted by 1x1.

### `fn translate(&self, delta: Delta) -> Self`

Computes a positional offset without changing the original mask.

```rust
use rpgx::prelude::*;
let translated = mask.translate(Delta::new(1, 1));
```

---

### `fn get_shape(&self) -> Shape`

Computes the bounding rectangle (`Shape`) that contains all tiles in the mask. The result's width and height represent the maximum extent from the top-left origin (0, 0).

---

### `fn contains(&self, coord: Coordinates) -> bool`

Returns `true` if any tile in the mask contains the given coordinate.

---

### `is_blocking_at(&self, target: &Coordinates) -> bool`

Returns `true` if the mask contains the target position and it's marked as blocking.

---

### `get_actions(&self) -> Vec<u32>`

Retrieve the list of actions ids that are applied to the mask.

---

### `get_texture(&self) -> Option<u32>`

Retrieve the texture of the mask.

---

### `get_render(&self) -> Option<u32>`

Retrieve the render function of the mask.

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
        Effect::Texture(2)
    ]
);

assert!(mask.contains(&Coordinates::new(1, 1)));
assert!(mask.is_blocking_at(&Coordinates::new(3,3)));
assets!(!mask.is_blocking_at(&Coordinates::new(1,1)));
```

The building mask contains a texture 4x6 repesenting a building.
The -1x1 (Rect::new(Coordinates::new(1, 1), Shape::new(3, 5))) shrinked block allows to make the building walkable only on its edges. (Just the inset 1;1 3x5 rect is blocking).

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
