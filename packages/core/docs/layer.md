# Layer Module

## Overview

The `Layer` module manages visual and logical overlays on top of a base tile grid (`Grid`) in an RPG engine. Layers enable grouped or conditional tile modifications without altering the base layer.

Layers simulate stacking along the Z-axis, each with a defined role via `LayerType`.

---

## Enum: `LayerType`

```rust
use rpgx::prelude::{Tile, Shape, Mask};

pub enum LayerType {
    Base,
    Action,
    Texture,
    Block,
}
```

- **Base**: The foundational layer covering the entire map, encompassing all layer shapes.
- **Action**: Contains tiles with gameplay actions or triggers.
- **Texture**: Visual overlay merged into the base layer to modify tile appearances.
- **Block**: Special layers with custom tile shapes or blocking properties.

---

## Struct: `Layer`

```rust
use rpgx::prelude::{LayerType, Tile, Shape, Mask};

pub struct Layer {
    pub name: String,
    pub kind: LayerType,
    pub tiles: Vec<Tile>,
    pub shape: Shape,
    pub masks: Vec<Mask>,
    pub z: u32,
}
```

- `name`: Identifier for the layer.
- `kind`: The role/type of the layer.
- `tiles`: Active tiles in this layer.
- `shape`: The rectangular bounds of the layer.
- `masks`: Selectors and effects applied to generate tiles.
- `z`: Z-index controlling rendering order.

---

## Constructors

### `Layer::new`

Creates a non-base layer by applying `masks` over a `shape`. Panics if `kind` is `Base`.

### `Layer::base`

Creates a base layer covering the union of all input layers’ shapes. Merges visual effects from `Texture` layers.

---

## Key Methods

- `reshape(shape)`: Reshapes the layer, discarding tiles outside bounds. Base layers regenerate all tiles preserving effects.
- `positive_reshape(shape)`: Expands layer shape if larger than current; never shrinks.
- `offset(delta)`: Offsets all tiles and shape by `delta` coordinates.
- `get_tile_at(pointer)`: Returns the tile covering given coordinates, considering tile shape.
- `get_block_at(block_selector)`: Returns all tiles inside a coordinate block.
- `is_blocking_at(pointer)`: Returns true if any tile covering the point is blocking.

---

## Tests

- Masks applying multiple effects.
- Mask application on blocks of tiles.
- Behavior with empty shapes.
- Tile retrieval beyond bounds returns `None`.
- Block queries only return existing tiles.
- Blocking tile detection.
- Tile and shape offsetting.
- Base layer creation from multiple layers.
- Behavior of `positive_reshape` expanding shape.
- Tile boundary checks in `get_tile_at`.

---

## Summary

The `Layer` struct and `LayerType` enum provide a flexible system for stacked tile layers with effects and conditional logic, forming the foundation for map representation and gameplay in RPGX.
