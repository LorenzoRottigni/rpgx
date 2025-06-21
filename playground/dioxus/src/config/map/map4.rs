use std::{any::Any, vec};

use rpgx::{library::Library, prelude::*};

pub fn use_map4(library: &Library<Box<dyn Any>>) -> Map {
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
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_perimeter(0, 2),
            vec![Effect::Texture(library.get_id("floor_3").unwrap())],
        )],
        1,
    );
    let layer3 = Layer::new(
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).into_circle(),
            vec![Effect::Texture(library.get_id("floor_2").unwrap())],
        )],
        1,
    );
    let layer4 = Layer::new(
        "ground".into(),
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
        "map4".into(),
        vec![layer1, layer2, layer3, layer4, layer5],
        Coordinates::default(),
    );
    map.duplicate_to_the(rpgx::prelude::Direction::Right, None);
    map.duplicate_to_the(rpgx::prelude::Direction::Down, None);

    let layer6 = Layer::new(
        "portal".into(),
        vec![Mask::new(
            "portal".into(),
            vec![Rect::from_many(Rect::from_shape(map.get_shape()).into_center(0, 2)).unwrap()],
            vec![Effect::Texture(library.get_id("portal_1").unwrap())],
        )],
        2,
    );

    map.load_layer(layer6);

    map
}
