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

```rust
use rpgx::prelude::*;


// Library provided by rpgx is a convenient way to manange resources
// outside the rpgx engine, keeping several internal processes on the stack.
let mut library: Library<Box<String>> = Library::new();

library.insert(
    "floor_1",
    Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp".to_string()),
);
library.insert(
    "floor_2",
    Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp".to_string()),
);
library.insert(
    "floor_3",
    Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp".to_string()),
);
library.insert(
    "building_1",
    Box::new("https://s3.rottigni.tech/rpgx/processor_8.webp".to_string()),
);
library.insert(
    "building_2",
    Box::new("https://s3.rottigni.tech/rpgx/processor_9.webp".to_string()),
);

// Create a ground layer that fits a dynamic grid currently spanning 15x15
let layer1 = Layer::new(
    "ground".into(),
    vec![Mask::new(
        "ground".into(),
        Rect::from_shape(Shape::from_square(15)).into_many(),
        vec![Effect::Texture(library.get_id("floor_1").unwrap())],
    )],
    1,
);
// Apply a specific texture only to even tiles within the 15x15 grid
let layer2 = Layer::new(
    "ground_evens_decoration".into(),
    vec![Mask::new(
        "ground".into(),
        Rect::from_shape(Shape::from_square(15)).into_evens(),
        vec![Effect::Texture(library.get_id("floor_3").unwrap())],
    )],
    1,
);
// Create a circle inset to the 15x15 grid with another texture
let layer3 = Layer::new(
    "ground_circle_decoration".into(),
    vec![Mask::new(
        "ground".into(),
        Rect::from_shape(Shape::from_square(15)).into_circle(),
        vec![Effect::Texture(library.get_id("floor_2").unwrap())],
    )],
    1,
);;
// Apply a rhombus shaped texture with dial 5 within the 15x15 grid
let layer4 = Layer::new(
    "ground_rhombus_decoration".into(),
    vec![Mask::new(
        "ground".into(),
        Rect::from_shape(Shape::from_square(15)).into_rhombus(5),
        vec![Effect::Texture(library.get_id("floor_3").unwrap())],
    )],
    1,
);
// Put a building at the center of the 15x15 grid
let layer5 = Layer::new(
    "building".into(),
    vec![Mask::new(
        "building".into(),
        vec![Rect::new(
            Coordinates::new(5, 3),
            Shape::from_rectangle(5, 7),
        )],
        vec![Effect::Texture(library.get_id("building_1").unwrap())],
    )],
    2,
);
// Arrange layers into the map
let mut map = Map::new(
    "example".into(),
    vec![layer1, layer2, layer3, layer4, layer5],
    Coordinates::default(),
);
// Duplicate the built map to its top right point
// transforming the map into a 30x15 grid
map.duplicate_to_the(Direction::Right, None);
// Duplicate the 30x15 grid to its bottom left point
// transforming the map into a 30x30 grid
map.duplicate_to_the(Direction::Down, None);

// Put a building at the center of the new 30x30 grid 
let external_layer = Layer::new(
    "big_building".into(),
    vec![Mask::new(
        "building_alt".into(),
        vec![Rect::new(
            Coordinates::new(11, 7),
            Shape::from_rectangle(8, 11),
        )],
        vec![Effect::Texture(library.get_id("building_2").unwrap())],
    )],
    2,
);
map.load_layer(external_layer);

// Duplicate the built 30x30 map to its top right point on more time,
// producing a final map with 60x30 grid.
map.duplicate_to_the(Direction::Right, None);
```

Produces the following output:

<img src="https://s3.rottigni.tech/rpgx/rpgx-example.png" alt="RPGX example" width="800" />

## Contributing

See [Contributing Guidelines](../../README.md#contributing).

## License

RPGX is licensed under the [MIT License](../../LICENSE).