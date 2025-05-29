# RPGX Core

RPGX Core is the foundational Rust crate powering the RPGX game engine. It provides a modular, extensible architecture for grid-based RPGs, including layered maps, tile effects, pathfinding, and entity movement logic.

## Features

- **Layered Maps:** Compose complex scenes with multiple logical and visual layers.
- **Tiles & Effects:** Attach visual and interactive effects to grid tiles.
- **Selectors & Masks:** Flexible targeting and zone definition for tiles.
- **Pathfinding:** Efficient movement and blocking logic for entities.
- **Pawns:** Manage entities and their interactions on the map.
- **Extensible:** Designed for integration with WASM, Dioxus, and more.

## Getting Started

Add RPGX Core to your Rust project:

```toml
[dependencies]
rpgx = { git = "https://github.com/LorenzoRottigni/rpgx", package = "rpgx" }
```

## Example

```rust
use rpgx::prelude::*;

let map = Map::new("demo", 10, 10);
let pawn = Pawn::new("hero", (0, 0));
let mut engine = Engine::new(map, pawn);

engine.move_to((5, 5)).unwrap();
```

## Documentation

- [API Reference](https://github.com/LorenzoRottigni/rpgx/tree/master/packages/core)
- [Main Project README](../../README.md)

## Contributing

See [Contributing Guidelines](../../README.md#contributing).

## License

RPGX is licensed under the [MIT License](../../LICENSE).