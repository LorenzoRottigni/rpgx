# RPGX

<img src="https://s3.rottigni.tech/rpgx/rpgx_logo_transparent.webp" alt="RPGX Logo" width="400" />

RPGX is a lightweight, modular, and extensible 2D RPG game engine written in Rust, designed for flexibility and ease of use. It provides a rich grid-based architecture with layered maps, tile effects, pathfinding, and entity movement logic.

## Features

- **Layered Maps:** Compose complex scenes with multiple logical and visual layers.
- **Tiles & Effects:** Attach visual and interactive effects to grid tiles.
- **Selectors & Masks:** Flexible targeting and zone definition for tiles.
- **Pathfinding:** Efficient movement and blocking logic for entities.
- **Pawns:** Manage entities and their interactions on the map.
- **Extensible:** Designed for integration with WASM, Dioxus, and more.

## Getting Started

Add RPGX to your Rust project:

```bash
cargo add rpgx
```

## Glossary

### RPGX

- [Effect](./packages/core/docs/effect.md)
- [Mask](./packages/core/docs/mask.md)
- [Layer](./packages/core/docs/layer.md)
- [Map](./packages/core/docs/map.md)
- [Scene](./packages/core/docs/scene.md)
- [Engine](./packages/core/docs/engine.md)

### Euclidean

- [Rect](./packages/core/docs/rect.md)
- [Coordinates](./packages/core/docs/coordinates.md)
- [Delta](./packages/core/docs/delta.md)
- [Direction](./packages/core/docs/direction.md)
- [Shape](./packages/core/docs/shape.md)

## Contributing

See [Contributing Guidelines](../../README.md#contributing).

## License

RPGX is licensed under the [MIT License](../../LICENSE).