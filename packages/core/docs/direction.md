# `Direction`

A `Direction` represents a unit step in one of the four **cardinal directions** on a 2D grid.

These are used for pathfinding, movement logic, map duplication, and direction-based calculations.


---

## Example

```rust
use rpgx::prelude::*;

let dir = Direction::Right;
let delta = dir.to_delta();
let moved = Coordinates::new(2, 3).offseted(delta);
// moved is now (3, 3)
```

---

## Variants

### `Up`

Moves one tile **up** (negative Y): `(0, -1)`

### `Down`

Moves one tile **down** (positive Y): `(0, 1)`

### `Left`

Moves one tile **left** (negative X): `(-1, 0)`

### `Right`

Moves one tile **right** (positive X): `(1, 0)`

---

## Methods

### `Direction::from_delta(delta: &Delta) -> Option<Direction>`

Converts a [`Delta`](delta.md) into a `Direction`, if the delta matches exactly one of the four cardinal directions (unit steps only).

```rust
use rpgx::prelude::*;

let delta = Delta::new(0, 1);
let direction = Direction::from_delta(&delta);
assert_eq!(direction, Some(Direction::Down));
```

Returns `None` for diagonal or multi-step deltas.

---

### `fn to_delta(&self) -> Delta`

Converts a `Direction` into its corresponding unit [`Delta`](delta.md).

```rust
use rpgx::prelude::*;

let d = Direction::Left.to_delta(); // Delta { dx: -1, dy: 0 }
```

---

## Design Notes

- `Direction` is useful when directional logic must be constrained to axis-aligned movement.
- For diagonal, variable-length, or arbitrary movements, use [`Delta`](delta.md) directly.
- In pathfinding, directions can be used to restrict movement (e.g. no diagonals).

---

## See Also

- [`Delta`](delta.md): Represents signed movement along both axes.
- [`Coordinates`](coordinates.md): Absolute positions on the map.
- [`Map`](map.md), [`Scene`](scene.md)
