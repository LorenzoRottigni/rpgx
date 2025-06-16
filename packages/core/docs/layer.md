# Layer

A `Layer` represents a stackable, named collection of [`Mask`](mask.md)s applied over a grid to define visual, interactive, or collision behavior. Layers can be used to separate concerns like base terrain, obstacles, or triggers, and are typically composited in rendering or logic processing using a `z`-index.

---

## Structure

```rust
use rpgx::prelude::*;
pub struct Layer {
    pub name: String,
    pub masks: Vec<Mask>,
    pub z: u32,
}
```

### Fields

- **`name: String`**  
  A human-readable identifier, e.g., `"collision"`, `"decorations"`, `"interactables"`.

- **`masks: Vec<Mask>`**  
  A list of [`Mask`](mask.md) objects, each of which contains tiles with applied [`Effect`](effect.md)s.

- **`z: u32`**  
  Z-index used to determine render or evaluation order. Layers with higher `z` appear "on top".

---

## Example

```rust
use rpgx::prelude::*;

let effect = Effect {
    block: Some(Rect::from_xywh(0, 0, 1, 1)),
    ..Default::default()
};
let mask = Mask::new("hazard_zone".into(), vec![Rect::from_xywh(5, 5, 2, 2)], effect);
let layer = Layer::new("hazards".into(), vec![mask], 10);
```

This creates a `hazards` layer with one mask that defines a 2×2 blocking zone starting at (5,5), rendered at z-index 10.

---

## See Also

- [`Tile`](tile.md)
- [`Effect`](effect.md)
- [`Mask`](mask.md)
- [`Map`](map.md)
- [`Rect`](rect.md), [`Coordinates`](coordinates.md), [`Delta`](delta.md), [`Shape`](shape.md)
