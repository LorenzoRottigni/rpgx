# `Effect`

An `Effect` represents a behavior, visual property, or gameplay rule applied to a tile or group of tiles in the RPGX engine. `Effect`s are attached to [`Mask`](mask.md)s and combined through [`Layer`](layer.md)s to build complex interactive maps.

Each `Effect` variant modifies how tiles are interpreted, rendered, or interacted with.

---

## Variants

### `Effect::None`

Represents the absence of any effect. This is the default state of a tile and has no impact on logic or rendering.

---

### `Effect::Action(u32)`

Associates an action ID with a tile. Action effects are typically consumed by logic engines or event systems to trigger behaviors such as opening a door, playing a cutscene, or enabling interactions.

```rust
Effect::Action(42)
```

> This assigns action `42` to the tile.

---

### `Effect::Texture(u32)`

Associates a texture ID with a tile. This is used by the renderer to display visual elements like terrain, objects, or decorations.

```rust
Effect::Texture(3)
```

> The renderer will display texture `3` for this tile.

---

### `Effect::Render(u32)`

Applies a render callback or shader ID to a tile. This effect allows runtime visual customization, such as animations, lighting, or shader overlays.

```rust
Effect::Render(1)
```

> This tells the rendering system to use render callback `1` on the tile.

---

### `Effect::Block(Rect)`

Defines a blocking region that prevents movement through the specified [`Rect`](rect.md). This is commonly used to define solid obstacles, walls, or restricted areas.

```rust
Effect::Block(Rect::from_xywh(1, 1, 3, 2))
```

> This blocks the area starting at `(1,1)` with width `3` and height `2`.

---

## Design Notes

- Effects are composable: multiple effects can be applied through a single [`Mask`](mask.md), enabling complex tile behavior.
- Blocking areas (`Block`) are region-based, while `Action`, `Texture`, and `Render` are usually per-tile.
- The engine typically merges multiple `Effect`s by priority or stacking logic when multiple layers overlap.
- `Effect::None` can be used to clear or reset a region without side effects.

---

## See Also

- [`Mask`](mask.md): Groups of `Rect`s with associated `Effect`s.
- [`Rect`](rect.md): A rectangular tile area.
- [`Layer`](layer.md): A logical group of masks with z-index stacking.
- [`Map`](map.md): The structure that assembles base layers and overlays.
- [`Coordinates`](coordinates.md): Used with `Rect` to apply effects to specific positions.
