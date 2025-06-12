use std::any::Any;

use rpgx::{
    library::Library,
    map::Map,
    prelude::{Coordinates, Direction, Effect, Layer, Mask, Selector, Shape},
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

    let decoration_shape = Shape {
        width: 6,
        height: 8,
    };
    let border_coords = {
        let mut coords = Vec::new();
        for y in 0..decoration_shape.height {
            for x in 0..decoration_shape.width {
                if x == 0
                    || y == 0
                    || x == decoration_shape.width - 1
                    || y == decoration_shape.height - 1
                {
                    coords.push(Coordinates { x, y });
                }
            }
        }
        coords
    };

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
        vec![Mask::new(
            "ground_decoration".to_string(),
            Selector::Sparse(border_coords),
            Effect {
                texture_id: Some(library.get_id("floor_3").unwrap()),
                ..Default::default()
            },
        )],
        1,
    ));

    map.duplicate_to_the(Direction::Right, None);
    map.duplicate_to_the(Direction::Down, None);

    map
}
