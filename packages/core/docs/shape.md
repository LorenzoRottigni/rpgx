# `Shape`

A `Shape` represents a 2D rectangular area using a `width` and `height`.  
It is used extensively for defining bounds, tilemaps, collision areas, and spatial logic.

---

## Struct Definition

```rust
pub struct Shape {
    pub width: u32,
    pub height: u32,
}
```

---

## Constructors

### `Shape::new(width, height)`

Creates a shape with the specified width and height.

```rust
use rpgx::prelude::*;

let shape = Shape::new(3, 4); // → Shape { width: 3, height: 4 }
```

### `Shape::from_square(side)`

Creates a square with equal width and height.

### `Shape::from_rectangle(width, height)`

Alias for `new`.

### `Shape::from_bounds(start, end)`

Creates a shape that spans the rectangular area between two [`Coordinates`](coordinates.md).

```rust
use rpgx::prelude::*;

let shape = Shape::from_bounds(Coordinates::new(2, 3), Coordinates::new(5, 6));
// → Shape { width: 3, height: 3 }
```

### `Shape::bounding_shape(&[Shape])`

Returns the minimal shape that fully contains all provided shapes by taking the component-wise maximum.

---

## Methods

### `shape.area() -> u32`

Returns the number of cells in the shape (width × height).

### `shape.union(other) -> Shape`

Returns a new `Shape` that is the union of two shapes, taking the max of each dimension.

### `shape.in_bounds(coord) -> bool`

Returns `true` if the coordinate lies within the shape's dimensions.

### `shape.delta_in_bounds(delta) -> bool`

Checks whether the given delta lies within the shape (non-negative and less than width/height).

### `shape.offset_by(coord) -> Shape`

Returns a new shape with width and height increased by `coord.x` and `coord.y`.

### `shape.expand_to_include(offset, other)`

Expands the shape to fit another shape placed at `offset`.

### `shape.coordinates_in_range(start, end) -> Vec<Coordinates>`

Returns a vector of [`Coordinates`](coordinates.md) clamped within the shape bounds.

### `shape.filter_coordinates(predicate) -> Vec<Coordinates>`

Iterates all coordinates in the shape and returns only those matching the filter function:

```rust
use rpgx::prelude::*;

let shape = Shape::from_rectangle(3, 3);
let diagonal = shape.filter_coordinates(|c, _| c.x == c.y);
// → [ (0,0), (1,1), (2,2) ]
```

---

## Operators

### Shape + Shape

Adds width and height component-wise, saturating on overflow.

### Shape - Shape

Subtracts width and height component-wise with saturation.

### Shape + u32 / Shape - u32

Adds or subtracts the scalar from both width and height.

### Shape + Coordinates / Shape - Coordinates

Adds or subtracts the coordinate’s x/y values from width/height.

### Shape / u32

Divides both width and height by the scalar.

---

## Examples

```rust
use rpgx::prelude::*;

let a = Shape::from_rectangle(4, 2);
let b = Shape::from_square(3);

let union = a.union(b);            // Shape { width: 4, height: 3 }
let expanded = a + Coordinates::new(1, 2); // Shape { width: 5, height: 4 }
let area = b.area();               // 9
```

---

## Design Notes

- Shape defines **non-positional** size, unlike [`Coordinates`](coordinates.md).
- Used in tilemaps, viewports, bounding box math, and placement calculations.
- Prefer `Shape` for **absolute space coverage**, `Delta` for movement, and `Coordinates` for positions.

---

## See Also

- [`Coordinates`](coordinates.md) – 2D positions
- [`Delta`](delta.md) – signed directional offsets
- [`Map`](map.md), [`Layer`](layer.md)
