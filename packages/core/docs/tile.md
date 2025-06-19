# Tile

A `Tile` represents a rectangular area on a grid with an associated [`Effect`](effect.md). It is the atomic visual or interactive unit in the map system, used in [`Layer`](layer.md)s and [`Mask`](mask.md)s.

## Structure

```rust
use rpgx::prelude::*;

pub struct Tile {
    pub area: Rect,
    pub effect: Effect,
}
```

### Fields

- **`area: Rect`**  
  A rectangular region defined by an origin (`Coordinates`) and a `Shape` (width and height). This represents where the tile exists on the grid.

- **`effect: Effect`**  
  An [`Effect`](effect.md) that determines the tile’s interactive or visual behavior — such as texture ID, render callback, action, or blocking.

---

## Behavior

### Tile Area

The `Rect` stored in `Tile::area` defines the exact region the tile covers. Unlike traditional tiles that are always 1×1, a tile in this system can span multiple cells, allowing for flexible composition (e.g. a large trap tile, a wide platform texture, etc.).

### Tile Effects

The `Effect` attached to a tile may contain:
- An action ID (triggered when a pawn interacts with it)
- A texture ID (used to render a graphic)
- A render ID (used to apply custom drawing logic)
- A blocking `Rect` (to make a portion or the whole tile non-walkable)

---

## Methods

### `fn contains(&self, coord: Coordinates) -> bool`

Returns `true` if the coordinate lies within the tile’s rectangular area.

```rust
use rpgx::prelude::*;

let tile = Tile {
    area: Rect::new(Coordinates::new(1, 1), Shape::new(2, 2)),
    effect: Effect::default(),
};

assert!(tile.contains(&Coordinates::new(2, 2))); // Inside
assert!(!tile.contains(&Coordinates::new(3, 3))); // Outside
```

---

## Example

```rust
use rpgx::prelude::*;

let effect = Effect {
    action_id: Some(42),
    block: Some(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
    ..Default::default()
};

let tile = Tile {
    area: Rect::new(Coordinates::new(5, 5), Shape::new(2, 2)),
    effect,
};

assert!(tile.contains(&Coordinates::new(5, 6)));
```

---

## Design Notes

- A `Tile` can span arbitrary sizes — it's not restricted to single grid cells.
- Tiles are composable into `Mask`s and `Layer`s.
- Blocking areas and interactions are encoded through the `Effect`, allowing one tile to serve multiple gameplay or rendering purposes.
- Tiles are **data containers** and do not contain logic beyond `contains`.

---

## See Also

- [`Effect`](effect.md): Behavioral or visual data tied to a tile.
- [`Rect`](rect.md), [`Coordinates`](coordinates.md), [`Shape`](shape.md)
- [`Mask`](mask.md): A group of related tiles with a shared purpose.
- [`Layer`](layer.md): A composition of multiple tiles across a 2D space.
