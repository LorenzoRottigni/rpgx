# `Rect`

A `Rect` represents a rectangular area in the RPGX engine. It is defined by an **origin** (top-left corner) and a **shape** (width and height). `Rect` is widely used to define regions in maps, apply masks, or generate spatial layouts.

---

## Fields

### `origin: Coordinates`

The top-left position of the rectangle on a 2D grid.

### `shape: Shape`

The dimensions of the rectangle, representing width (`x`) and height (`y`) as an increment from the origin.

---

## Methods

### Constructors

#### `Rect::new(origin: Coordinates, shape: Shape) -> Self`

Constructs a new `Rect` using the provided origin and shape.

```rust
let origin = Coordinates::new(2, 3);
let shape = Shape::new(4, 5);
let rect = Rect::new(origin, shape);
```

---

#### `Rect::from_shape(shape: Shape) -> Self`

Creates a `Rect` from a shape, defaulting the origin to `(0, 0)`.

```rust
let rect = Rect::from_shape(Shape::new(3, 3));
// origin = (0, 0), shape = (3, 3)
```

---

#### `Rect::from_origin(origin: Coordinates) -> Self`

Creates a `Rect` from an origin, defaulting the shape to `1x1`.

```rust
let rect = Rect::from_origin(Coordinates::new(5, 5));
// shape = (1, 1)
```

---

#### `Rect::from_many(rects: Vec<Self>) -> Result<Self, RectError>`

Merges multiple `Rect`s into the smallest `Rect` that contains them all.

```rust
let group = vec![
    Rect::from_xywh(1, 1, 2, 2),
    Rect::from_xywh(4, 3, 1, 1),
];
let merged = Rect::from_many(group)?;
```

---

#### `Rect::from_xywh(x: u32, y: u32, width: u32, height: u32) -> Self`

Creates a `Rect` from origin `(x, y)` and dimensions `(width, height)`.

```rust
let rect = Rect::from_xywh(2, 2, 5, 4);
```

---

### Drawing API

#### `Rect::into_many(&self) -> Vec<Self>`

Splits the `Rect` into a vector of `1x1` rects covering the entire area.

```rust
let rect = Rect::from_xywh(0, 0, 2, 2);
let tiles = rect.into_many();
// tiles: [(0,0), (1,0), (0,1), (1,1)]
```

---

#### `Rect::into_single(&self) -> Vec<Self>`

Wraps the whole rect as a single unit in a vector.

```rust
let rect = Rect::from_xywh(0, 0, 3, 3);
let single = rect.into_single();
// single.len() == 1
```

---

#### `Rect::into_perimeter(&self, offset: u32, size: u32) -> Vec<Self>`

Returns the outer `1x1` perimeter tiles of the rect. `offset` moves the perimeter inward, `size` defines thickness.

```rust
let perimeter = rect.into_perimeter(0, 1);
// Outer edge of the rectangle
```

---

#### `Rect::into_bisector(&self, offset: u32, size: u32) -> Vec<Self>`

Returns tiles forming a central vertical or horizontal line across the rect.

```rust
let bisector = rect.into_bisector(0, 1);
```

---

#### `Rect::into_center(&self, offset: u32, size: u32) -> Vec<Self>`

Returns the center square block of the rect. Offset and size control exact position and area.

```rust
let center = rect.into_center(0, 1);
```

---

#### `Rect::into_rhombus(&self, dial: u32) -> Vec<Self>`

Returns a rhombus pattern inscribed within the rect. `dial` controls radius.

```rust
let rhombus = rect.into_rhombus(2);
```

---

#### `Rect::into_circle(&self) -> Vec<Rect>`

Returns an approximate circle pattern inscribed within the rect.

```rust
let circle = rect.into_circle();
```

---

#### `Rect::into_odds(&self) -> Vec<Rect>`

Returns only the 1x1 tiles with odd coordinates within the rect.

```rust
let odds = rect.into_odds();
```

---

#### `Rect::into_evens(&self) -> Vec<Rect>`

Returns only the 1x1 tiles with even coordinates within the rect.

```rust
let evens = rect.into_evens();
```

---

### Bounding API

#### `Rect::top_left(&self) -> Coordinates`

Returns the top-left coordinate of the rect.

---

#### `Rect::top_right(&self) -> Coordinates`

Returns the top-right coordinate of the rect.

---

#### `Rect::bottom_left(&self) -> Coordinates`

Returns the bottom-left coordinate of the rect.

---

#### `Rect::bottom_right(&self) -> Coordinates`

Returns the bottom-right coordinate of the rect.

---

#### `Rect::center(&self) -> Coordinates`

Returns the center coordinate of the rect.

---

#### `Rect::contains(&self, target: &Coordinates) -> bool`

Checks whether the given coordinate is inside the rect.

```rust
let inside = rect.contains(&Coordinates::new(1, 1));
```

---

### Transformations

#### `Rect::offset(&mut self, delta: Delta)`

Applies an in-place positional offset to the rect. Useful when merging maps or shifting regions.

```rust
rect.offset(Delta::new(2, 0));
```

---

#### `Rect::translate(&self, delta: Delta) -> Self`

Returns a new `Rect` with the same shape but moved by the delta.

```rust
let moved = rect.translate(Delta::new(1, 1));
```
