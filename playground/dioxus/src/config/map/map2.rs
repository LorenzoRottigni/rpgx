use std::any::Any;

use rpgx::{
    library::Library,
    map::Map,
    prelude::{Coordinates, Direction, Effect, Layer, LayerType, Mask, Selector, Shape},
};

pub fn use_map2(library: &Library<Box<dyn Any>>) -> Map {
    let mut building_1 = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("building_1").unwrap(),
        library.get_id("consolelog").unwrap(),
    );
    building_1.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_1").unwrap(),
    ));
    building_1.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("floor_2").unwrap(),
    ));
    building_1.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_2").unwrap(),
    ));

    let mut building_2 = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("building_2").unwrap(),
        library.get_id("consolelog").unwrap(),
    );
    building_2.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_1").unwrap(),
    ));
    building_2.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("floor_2").unwrap(),
    ));
    building_2.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_2").unwrap(),
    ));

    let mut map = Map::compose(
        "TestMap".to_string(),
        vec![
            (building_1, Coordinates { x: 0, y: 0 }),
            (building_2, Coordinates { x: 8, y: 0 }),
        ],
        vec![],
        Coordinates { x: 0, y: 0 },
    );

    map.load_layer(Layer::new(
        "ground_decoration".to_string(),
        LayerType::Texture,
        map.get_shape(),
        vec![Mask::new(
            "ground_decoration".to_string(),
            Selector::Filter(|pointer, shape| {
                pointer.x == 0
                    || pointer.y == 0
                    || pointer.x == shape.width - 1
                    || pointer.y == shape.height - 1
            }),
            Effect {
                texture_id: Some(library.get_id("floor_3").unwrap()),
                ..Default::default()
            },
        )],
        1,
    ));

    map.duplicate_to_the(Direction::Right);
    map.duplicate_to_the(Direction::Down);

    map
}
