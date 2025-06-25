# Rect

A `Rect` is a flexible abstraction repesenting a rectangular area within the RPGX engine.

## Fields

### `origin: Coordinates`

The top-left origin coordinates of the rect.

### `shape: Shape`

The shape (dimensions) of the rect, incresement of X and Y from the origin

---

## Methods

### Constructors

#### `Rect::new(origin: Coordinates, shape: Shape) -> Self`

Constructs a new Rect from its origin and shape.

#### `Rect::from_shape(shape: Shape) -> Self`

Construct a new Rect from a shape, defaulting its origin to 0;0.

#### `Rect::from_origin(origin: Coordinates) -> Self`

Construct a new Rect from its origin, defaulting its shape to 1x1.

#### `Rect::from_many(rects: Vec<Self>) -> Result<Self, RectError>`

Construct a new rect by merging a block of smaller rects (tipically coming from rect.into_many())

#### `Rect::from_xywh(x: u32, y: u32, width: u32, height: u32) -> Self`

Creates a new `Rect` from origin `(x, y)` and dimensions `(width, height)`.

### Drawing API

#### `Rect::into_many(&Self) -> Vec<Self>`

Splits the Rect into its contained 1x1 rects.

#### `Rect::into_single(&Self) -> Vec<Self>`

Take a rect as a single unit, semantic purpose implementation.

#### `Rect::into_perimeter(&Self, offset: u32, size: u32) -> Vec<Self>`

Take the perimeter 1x1 tiles of current rect. offset allows to apply a delta to the perimeter calculation. Size allow to define how many 1x1 tiles the perimeter spans in its width.

#### `Rect::into_bisector(&self, offset: u32, size: u32) -> Vec<Self>`

Take the bisector of the current rect. Offset allows to apply a delta to the bisector calculation, Size allows to define how many 1x1 tiles the bisector spans in its width.

#### `Rect::into_center(&self, offset: u32, size: u32) -> Vec<Self>`

Take the center block of the current rect. Offset allows to apply a delta to the center calculation, Size allows to define how many 1x1 tiles the center spans in its width.

#### `Rect::into_rhombus(&self, dial: u32) -> Vec<Self>`

Take the Rect inset rhombus, dial allows to define the width of the rhoumbus starting from the Rect center.

#### `Rect::into_circle(&self) -> Vec<Rect>`

Take the Rect inset circle.

#### `Rect::into_odds(&self) -> Vec<Rect>`

Take the odds 1x1 rects contained in the Rect

#### `Rect::into_evens(&self) -> Vec<Rect>`

Take the evens 1x1 rects contained in the Rect

### Bounding API

#### `Rect::top_left(&Self) -> Coordinates`

Get the top left corner of the Rect

#### `Rect::top_right(&Self) -> Coordinates`

Get the top right corner of the Rect

#### `Rect::bottom_left(&Self) -> Coordinates`

Get the bottom left corner of the Rect

#### `Rect::bottom_right(&Self) -> Coordinates`

Get the bottom right corner of the Rect

#### `Rect::center(&Self) -> Coordinates`

Get the center point of the Rect

#### `Rect::contains(&Self, target: &Coordinates) -> bool`

Returns `true` if the rect contains the given coordinate.

### `Rect::offset(&mut self, delta: Delta)`

Applies a positional offset to the rect.
This is designed to manage the merge of several maps with different shapes.

### `Rect::translate(&self, delta: Delta) -> Self`

Computes a positional offset without changing the original Rect.
