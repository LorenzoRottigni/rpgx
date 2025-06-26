# `Coordinates`

A `Coordinates` represents a 2D grid position with unsigned `x` and `y` values. It is the primary spatial unit used throughout the RPGX engine for addressing tiles, shapes, and spatial operations.

Coordinates support basic arithmetic, delta-based translation, and bounding logic.

---

## Example

```rust
use rpgx::prelude::*;

let a = Coordinates::new(2, 3);
let b = Coordinates::new(1, 1);
let delta = Delta::new(-1, 2);

assert_eq!(a + b, Coordinates::new(3, 4));
assert_eq!(a.offseted(delta), Coordinates::new(1, 5));
```

---

## Fields

### `x: u32`

The horizontal position on the grid (0-indexed).

### `y: u32`

The vertical position on the grid (0-indexed).

---

## Constructors

### `Coordinates::new(x: u32, y: u32) -> Self`

Creates a new `Coordinates` instance at the specified `x`, `y` position.

```rust
use rpgx::prelude::*;

let coord = Coordinates::new(2, 3);
```

---

### `Coordinates::bounding_box(coords: &[Coordinates]) -> Option<(Coordinates, Coordinates)>`

Computes the smallest **exclusive bounding box** that contains all the given coordinates.

```rust
use rpgx::prelude::*;

let points = vec![
    Coordinates::new(1, 2),
    Coordinates::new(3, 4),
    Coordinates::new(2, 1),
];
let (min, max) = Coordinates::bounding_box(&points).unwrap();
assert_eq!(min, Coordinates::new(1, 1));
assert_eq!(max, Coordinates::new(4, 5)); // Exclusive
```

Returns `None` if the input is empty.

---

## Methods

### `fn is_origin(&self) -> bool`

Returns `true` if the coordinate is at the origin `(0, 0)`.

---

### `fn is_aligned_with(self, other: Self) -> bool`

Returns `true` if `self` shares either the same `x` or `y` value with `other`.

---

### `fn is_within(&self, origin: Coordinates, shape: Shape) -> bool`

Checks if the coordinate lies inside the rectangle defined by `origin` and `shape`. The bounds are **exclusive**: `[origin, origin + shape)`.

```rust
use rpgx::prelude::*;

let inside = Coordinates::new(3, 4).is_within(Coordinates::new(2, 2), Shape::new(3, 3));
```

---

### `fn offseted(self, delta: Delta) -> Self`

Applies a [`Delta`](delta.md) to this coordinate, using **saturating arithmetic** to prevent underflow.

---

### `fn try_offseted(self, delta: Delta) -> Option<Coordinates>`

Applies a delta, returning `None` if it would result in negative coordinates.

---

### `fn to_delta(self) -> Delta`

Converts the coordinate into a delta with `dx = x` and `dy = y`.

---

## Design Notes

- Coordinates are unsigned and saturate on subtraction. Use `try_offseted` when negative shifts should be validated.
- Supports composable arithmetic with `Shape`, `Delta`, and other `Coordinates`.
- Designed for grid-based logic in pathfinding, masking, and tile manipulation.

---

## See Also

- [`Delta`](delta.md): Describes relative movement across coordinates.
- [`Shape`](shape.md): Describes dimensions for regions starting at a coordinate.
- [`Rect`](rect.md): Represents rectangular areas of the grid.
- [`Mask`](mask.md), [`Layer`](layer.md), [`Map`](map.md)
