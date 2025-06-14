use std::any::Any;

use rpgx::{
    library::Library,
    prelude::{Coordinates, Effect, Layer, Map, Mask, Rect, Shape},
};

pub fn use_map4(library: &Library<Box<dyn Any>>) -> Map {
    let layer1 = Layer::new(
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).as_many(),
            Effect {
                texture_id: library.get_id("floor_1"),
                ..Default::default()
            },
        )],
        1,
    );
    let layer2 = Layer::new(
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).as_perimeter(0, 2),
            Effect {
                texture_id: library.get_id("floor_3"),
                ..Default::default()
            },
        )],
        1,
    );
    let layer3 = Layer::new(
        "ground".into(),
        vec![Mask::new(
            "ground".into(),
            Rect::from_shape(Shape::from_square(15)).as_round(5),
            Effect {
                texture_id: library.get_id("floor_2"),
                ..Default::default()
            },
        )],
        1,
    );
    Map::new(
        "map4".into(),
        vec![layer1, layer2, layer3],
        Coordinates::default(),
    )
}
