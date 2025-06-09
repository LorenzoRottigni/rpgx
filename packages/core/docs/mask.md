# Mask

A `Mask` defines a logical area on a [`Grid`](super::grid::Grid) or [`Layer`](super::layer::Layer) where a specific [`Effect`](../Effect) is applied based on a [`Selector`](../Selector) pattern.

Masks allow you to apply effects (like blocking, overlays, or behaviors) in batch to multiple tiles. These are especially useful when building maps or dynamically modifying tile states at runtime.

## Usage

```rust
use rpgx::prelude::{Mask, Selector, Effect, Shape, SingleSelector};

let mask = Mask::new(
    "Highlight".to_string(),
    Selector::Block((SingleSelector { x: 1, y: 1 }, SingleSelector { x: 2, y: 2 })),
    Effect { group: true, ..Default::default() },
);

let shape = Shape::from_square(4);
let tiles = mask.apply(shape);

assert_eq!(tiles.len(), 1);
```
