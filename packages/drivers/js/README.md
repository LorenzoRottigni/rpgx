# @rpgx/js

<img src="https://s3.rottigni.tech/rpgx/rpgx_logo_transparent.webp" alt="RPGX Logo" width="400" />

RPGX is a lightweight, modular, and extensible 2D RPG game engine written in Rust, designed for flexibility and ease of use. It provides a rich grid-based architecture with layered maps, tile effects, pathfinding, and entity movement logic.

This module provides the WASM bindings for the RPGX game engine allowing seamless integration with NodeJS and web browsers.

## Getting Started

Add RPGX to your JS project:

```bash
npm install @rpgx/js

yarn add @rpgx/js

pnpm install @rpgxjs
```

## Glossary

### RPGX

- [Effect](https://docs.rs/rpgx/0.1.3/rpgx/map/effect/enum.Effect.html)
- [Mask](https://docs.rs/rpgx/0.1.3/rpgx/map/mask/struct.Mask.html)
- [Layer](https://docs.rs/rpgx/0.1.3/rpgx/map/layer/struct.Layer.html)
- [Map](https://docs.rs/rpgx/0.1.3/rpgx/map/struct.Map.html)
- [Scene](https://docs.rs/rpgx/0.1.3/rpgx/engine/scene/struct.Scene.html)
- [Engine](https://docs.rs/rpgx/0.1.3/rpgx/engine/struct.Engine.html)

### Euclidean

- [Rect](https://docs.rs/rpgx/0.1.3/rpgx/eucl/rect/struct.Rect.html)
- [Coordinates](https://docs.rs/rpgx/0.1.3/rpgx/eucl/coordinates/struct.Coordinates.html)
- [Delta](https://docs.rs/rpgx/0.1.3/rpgx/eucl/delta/struct.Delta.html)
- [Direction](https://docs.rs/rpgx/0.1.3/rpgx/eucl/direction/enum.Direction.html)
- [Shape](https://docs.rs/rpgx/0.1.3/rpgx/eucl/shape/struct.Shape.html)

## Example

```js
import {
  Library,
  Layer,
  Map,
  Mask,
  Effect,
  Shape,
  Rect,
  Coordinates,
  Direction,
} from '@rpgx/wasm';

// Create a new library for managing resources
const library = new Library();

// Insert textures into the library
library.insert(
  'floor_1',
  'https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp'
);
library.insert(
  'floor_2',
  'https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp'
);
library.insert(
  'floor_3',
  'https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp'
);
library.insert(
  'building_1',
  'https://s3.rottigni.tech/rpgx/processor_8.webp'
);
library.insert(
  'building_2',
  'https://s3.rottigni.tech/rpgx/processor_9.webp'
);

// Create 15x15 ground layer with floor_1 texture
const layer1 = new Layer(
  'ground',
  [
    new Mask(
      'ground',
      Rect.fromShape(Shape.fromSquare(15)).intoMany(),
      [Effect.texture(library.getId('floor_1').unwrap())]
    ),
  ],
  1
);

// Decoration on even tiles with floor_3 texture
const layer2 = new Layer(
  'ground_evens_decoration',
  [
    new Mask(
      'ground',
      Rect.fromShape(Shape.fromSquare(15)).intoEvens(),
      [Effect.texture(library.getId('floor_3').unwrap())]
    ),
  ],
  1
);

// Decoration with circle shape and floor_2 texture
const layer3 = new Layer(
  'ground_circle_decoration',
  [
    new Mask(
      'ground',
      Rect.fromShape(Shape.fromSquare(15)).intoCircle(),
      [Effect.texture(library.getId('floor_2').unwrap())]
    ),
  ],
  1
);

// Rhombus shape decoration with floor_3 texture
const layer4 = new Layer(
  'ground_rhombus_decoration',
  [
    new Mask(
      'ground',
      Rect.fromShape(Shape.fromSquare(15)).intoRhombus(5),
      [Effect.texture(library.getId('floor_3').unwrap())]
    ),
  ],
  1
);

// Building placed at the center (5,3) with size (5x7)
const layer5 = new Layer(
  'building',
  [
    new Mask(
      'building',
      [new Rect(new Coordinates(5, 3), Shape.fromRectangle(5, 7))],
      [Effect.texture(library.getId('building_1').unwrap())]
    ),
  ],
  2
);

// Create initial map with all layers
const map = new Map(
  'example',
  [layer1, layer2, layer3, layer4, layer5],
  Coordinates.default()
);

// Duplicate right to make it 30x15
map.duplicateToThe(new Direction("right"), null);

// Duplicate down to make it 30x30
map.duplicateToThe(new Direction("down"), null);

// Add a big building at (11, 7) sized 8x11
const externalLayer = new Layer(
  'big_building',
  [
    new Mask(
      'building_alt',
      [new Rect(new Coordinates(11, 7), Shape.fromRectangle(8, 11))],
      [Effect.texture(library.getId('building_2').unwrap())]
    ),
  ],
  2
);
map.loadLayer(externalLayer);

// Final duplication to the right: map is now 60x30
map.duplicateToThe(new Direction("right"), null);
```


Produces the following output:

<img src="https://s3.rottigni.tech/rpgx/rpgx-example.png" alt="RPGX example" width="800" />

## Contributing

See [Contributing Guidelines](../../README.md#contributing).

## License

RPGX is licensed under the [MIT License](../../LICENSE).
