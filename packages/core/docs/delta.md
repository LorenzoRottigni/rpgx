# `Delta`

A `Delta` represents a signed 2D offset — a difference in position, movement, or translation across a grid.

It is typically used to describe relative shifts between [`Coordinates`](coordinates.md), direction-based motion, or area translation.


---

## Example

```rust
use rpgx::prelude::*;

let d1 = Delta::new(3, -2);
let d2 = Delta::new(-1, 1);
let combined = d1 + d2; // Delta { dx: 2, dy: -1 }
```

---

## Fields

### `dx: i32`

The horizontal delta (positive is rightward, negative is leftward).

### `dy: i32`

The vertical delta (positive is downward, negative is upward).

---

## Constructors

### `Delta::new(dx: i32, dy: i32) -> Self`

Creates a new delta with the specified components.

```rust
use rpgx::prelude::*;

let d = Delta::new(2, -1);
```

---

### `Delta::zero() -> Self`

Returns a delta representing no movement: `(0, 0)`.

---

## Methods

### `fn invert(self) -> Self`

Returns a new delta with inverted components (`-dx`, `-dy`).

---

### `fn is_zero(self) -> bool`

Returns `true` if both components are zero.

---

### `fn manhattan(self) -> u32`

Computes the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry), i.e., the sum of absolute values of `dx` and `dy`.

```rust
use rpgx::prelude::*;

let d = Delta::new(-2, 3);
assert_eq!(d.manhattan(), 5);
```

---

### `fn is_axis_aligned(self) -> bool`

Returns `true` if the delta moves **only** in one direction (either horizontal or vertical).

---

### `fn is_diagonal(self) -> bool`

Returns `true` if both `dx` and `dy` are nonzero, i.e., the movement is diagonal.

---

## Design Notes

- `Delta` is signed and supports flexible offsetting in all directions.
- Used heavily in coordinate math, pathfinding, area translation, and movement logic.
- Use `Coordinates::offseted()` or `try_offseted()` to apply deltas to unsigned positions.

---

## See Also

- [`Coordinates`](coordinates.md): Grid position values.
- [`Shape`](shape.md): Region dimensions.
- [`Rect`](rect.md): Rectangular areas of the grid.
- [`Layer`](layer.md), [`Map`](map.md)
