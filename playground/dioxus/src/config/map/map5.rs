use std::{any::Any, vec};

use rpgx::prelude::*;

pub fn use_map5(library: &Library<Box<dyn Any>>) -> Map {
    let layer1 = Layer::new(
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_many(),
            vec![Effect::Texture(library.get_id("floor_1").unwrap())],
        )],
        1,
    );
    let layer2 = Layer::new(
        "ground_evens_decoration".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_evens(),
            vec![Effect::Texture(library.get_id("floor_3").unwrap())],
        )],
        1,
    );
    let layer3 = Layer::new(
        "ground_circle_decoration".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_circle(),
            vec![Effect::Texture(library.get_id("floor_2").unwrap())],
        )],
        1,
    );
    let layer4 = Layer::new(
        "ground_rhombus_decoration".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_rhombus(5),
            vec![Effect::Texture(library.get_id("floor_3").unwrap())],
        )],
        1,
    );
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
    let mut map = Map::new(
        "example".into(),
        vec![layer1, layer2, layer3, layer4, layer5],
        Coordinates::default(),
    );
    map.duplicate_to_the(Direction::Right, None);
    map.duplicate_to_the(Direction::Down, None);

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

    map.duplicate_to_the(Direction::Left, None);

    map
}
