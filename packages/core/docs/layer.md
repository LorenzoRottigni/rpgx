# `Layer`

A `Layer` is a logical or visual overlay composed of one or more [`Mask`](mask.md)s, each of which defines a group of tile-based [`Effect`](effect.md)s (such as collisions, textures, or actions).

Layers enable the modular composition of game behavior, visual rendering, and interactive elements. They can be stacked using their `z` index to control rendering or processing order.

---

## Fields

### `name: String`

A human-readable identifier for the layer (e.g., `"collision"`, `"decals"`, `"interactions"`). Useful for debugging, editor tools, or dynamic filtering.

---

### `masks: Vec<Mask>`

A list of [`Mask`](mask.md)s applied in this layer. Each mask can apply one or more effects over a rectangular region.

---

### `z: u32`

The z-index of the layer. Layers are rendered or evaluated in ascending z-order (lower `z` appears below higher `z`).

---

## Methods

### `Layer::new(name: String, masks: Vec<Mask>, z: u32) -> Self`

Constructs a new `Layer` with the given name, list of masks, and z-index.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);
```

---

### `fn get_shape(&self) -> Shape`

Returns the bounding [`Shape`](shape.md) that contains all the masks in the layer.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

let bounds = layer.get_shape();
```

---

### `fn offset(&mut self, delta: Delta)`

Applies a positional shift to all the masks in the layer, updating all coordinates and effect areas.

```rust
use rpgx::prelude::*;

let mut layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

layer.offset(Delta::new(2, 3));
```

> This modifies the layer in-place and is useful for relocating or merging map segments.

---

### `fn translate(&self, delta: Delta) -> Self`

Returns a new `Layer` with the same masks but offset by the specified `Delta`.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

let translated = layer.translate(Delta::new(1, 1));
```

> The original layer remains unchanged.

---

### `fn contains(&self, coord: &Coordinates) -> bool`

Returns `true` if any mask in the layer contains the given coordinate.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

if layer.contains(&Coordinates::new(5, 5)) {
    // ...
}
```

---

### `fn is_blocking_at(&self, target: &Coordinates) -> bool`

Returns `true` if the specified coordinate is marked as blocking in any mask within the layer.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

if layer.is_blocking_at(&Coordinates::new(2, 3)) {
    // Prevent movement
}
```

---

### `fn get_actions_at(&self, target: &Coordinates) -> Vec<u32>`

Returns a list of action effect IDs applied at the given coordinate. If no action applies, returns an empty list.

```rust
use rpgx::prelude::*;

let layer = Layer::new(
    "combined_grounds".into(),
    vec![
        Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(10)).into_many(),
            vec![Effect::Texture(1)]
        ),
        Mask::new(
            "inner_ground".into(),
            Rect::new(Coordinates::new(1,1), Shape::from_square(8)).into_many(),
            vec![Effect::Texture(2)]
        )
    ],
    1
);

let actions = layer.get_actions_at(&Coordinates::new(1, 2));
```

---

## Usage Example

```rust
use rpgx::prelude::*;

let mask = Mask::new(
    "interaction_zone".into(),
    vec![Rect::from_xywh(1, 1, 2, 2)],
    vec![Effect::Action(42)],
);

let layer = Layer::new("interactions".into(), vec![mask], 2);

assert!(layer.contains(&Coordinates::new(2, 2)));
assert_eq!(layer.get_actions_at(&Coordinates::new(2, 2)), vec![42]);
assert!(!layer.is_blocking_at(&Coordinates::new(0, 0)));
```

---

## Design Notes

- `Layer`s enable composition of logic like collision, decoration, event zones, and rendering overlays.
- They can be programmatically offset or duplicated via `offset` and `translate`.
- Layers are independent of the map grid and do not need to cover the full area.
- When merged into a [`Map`](map.md), their effects are combined according to their z-index.

---

## See Also

- [`Mask`](mask.md): A group of [`Rect`](rect.md)s with associated [`Effect`](effect.md)s.
- [`Effect`](effect.md): The behaviors or visuals associated with a tile.
- [`Rect`](rect.md): A rectangular region.
- [`Shape`](shape.md): Width and height abstraction.
- [`Coordinates`](coordinates.md): X, Y position on the map.
- [`Delta`](delta.md): A vector used for position shifting.
- [`Map`](map.md): The main structure that contains base layers and overlays.
