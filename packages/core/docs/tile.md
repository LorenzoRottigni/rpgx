# Tile

A `Tile` represents a single unit or shape on a 2D grid in the game world. It encapsulates:

- A unique identifier (`id`)
- Its spatial location (`pointer`)
- A shape (`shape`)
- An [`Effect`] that determines behavior like blocking or status effects

---

## Fields

- **`id: u32`**  
  A unique identifier for the tile. Used for tracking or indexing.

- **`effect: Effect`**  
  Describes gameplay-related metadata for the tile, such as whether it's blocking or applies a condition.

- **`pointer: SingleSelector`**  
  Starting position (`Coordinates`) for the tile. This usually refers to the top-left corner.

- **`shape: Shape`**  
  Describes the tile's size in width and height.

---

## Behavior

### `contains(&self, point: Coordinates) -> bool`

Checks if a given coordinate lies **within the tile's area**, based on its shape and starting location.

### `is_blocking_at(&self, target: Coordinates) -> bool`

Returns `true` if the tile blocks movement at the given coordinate. Blocking depends on:

- Whether `effect.block == true`
- Whether the optional `effect.shrink` bounds override the full shape

### `offset(&mut self, delta: Coordinates)`

Shifts the tile's position (and any blocking region) by a given delta.

---

## Related Types

- [`Effect`](crate::prelude::Effect)
- [`Coordinates`](crate::prelude::Coordinates)
- [`Shape`](crate::prelude::Shape)
- [`SingleSelector`](crate::prelude::SingleSelector)
