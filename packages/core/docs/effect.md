# Effect

The `Effect` struct defines the visual and interactive properties that can be applied to a [`Tile`](tile.md) or other UI/grid elements. These properties include optional identifiers for actions, textures, rendering callbacks, and movement-blocking regions.

## Purpose

Effects are used to enrich tiles with behavior and visuals:
- Mark tiles as blocking (e.g., walls or obstacles).
- Associate tiles with actions or triggers (e.g., interactive elements).
- Attach rendering or texture information for display purposes.

## Fields

### `action_id: Option<u32>`

Optional ID representing an action associated with the tile. This could correspond to interactive behaviors, scripts, or event triggers (e.g., open chest, start cutscene).

### `texture_id: Option<u32>`

Optional ID for the visual texture to render on the tile.

### `render_id: Option<u32>`

Optional ID used to delegate rendering to a custom callback or render handler. Useful for dynamic effects like animations or conditional visuals.

### `block: Option<Rect>`

An optional rectangular area (in tile-local coordinates) that blocks movement or interaction. When set, only the portion of the tile within this region is considered impassable.

This field enables partial blocking, such as only the left half of a tile being a wall, or the center of a large tile being impassable.

## Methods

### `offset(delta: Delta)`

Applies a positional delta to the `block` region if it exists. This is useful when moving or translating a tile’s position on the map.

```rust
use rpgx::prelude::*;

let mut effect = Effect {
    block: Some(Rect::new(Coordinates { x: 2, y: 3 }, Shape { width: 2, height: 2 })),
    ..Default::default()
};

// Shift the blocking region by (1, 1)
effect.offset(Delta { dx: 1, dy: 1 });

assert_eq!(effect.block.unwrap().origin, Coordinates { x: 3, y: 4 });
```