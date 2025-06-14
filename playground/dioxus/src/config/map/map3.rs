use std::any::Any;

use rpgx::{
    library::Library,
    prelude::{Coordinates, Effect, Layer, Map, Mask, Rect, Shape},
};

const SHAPE_WIDTH: u32 = 4;
const SHAPE_HEIGHT: u32 = 6;

fn is_center_tile(pointer: Coordinates, shape: Shape) -> bool {
    // Select a 2x2 block in the center
    let center_x = shape.width / 2;
    let center_y = shape.height / 2;
    (pointer.x == center_x || pointer.x == center_x - 1)
        && (pointer.y == center_y || pointer.y == center_y - 1)
}

pub fn use_map3(library: &Library<Box<dyn Any>>) -> Map {
    let shape: Shape = Shape {
        width: SHAPE_WIDTH,
        height: SHAPE_HEIGHT,
    };

    // Collect center 2x2 coords
    let center_coords = {
        let mut coords = Vec::new();
        for y in 0..shape.height {
            for x in 0..shape.width {
                let coord = Coordinates { x, y };
                if is_center_tile(coord, shape) {
                    coords.push(coord);
                }
            }
        }
        coords
    };

    let ground_layer = Layer::new(
        "ground".to_string(),
        vec![
            Mask::new(
                "default_floor".to_string(),
                // Selector::Block(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
                Rect::new(Coordinates { x: 0, y: 0 }, shape).as_many(),
                Effect {
                    texture_id: Some(library.get_id("floor_1").unwrap()),
                    ..Default::default()
                },
            ),
            Mask::new(
                "floor_alt".to_string(),
                center_coords
                    .iter()
                    .map(|c| Rect::new(*c, Shape::from_square(1)))
                    .collect(),
                Effect {
                    texture_id: library.get_id("floor_2"),
                    ..Default::default()
                },
            ),
        ],
        1,
    );

    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            // Selector::Block(Rect::new(
            //     Coordinates { x: 1, y: 1 },
            //     Shape {
            //         width: 3 - 1,  // 2
            //         height: 4 - 1, // 3
            //     },
            // )),
            vec![Rect::new(
                Coordinates { x: 1, y: 1 },
                Shape {
                    width: 3 - 1,  // 2
                    height: 4 - 1, // 3
                },
            )],
            Effect {
                texture_id: Some(library.get_id("building_1").unwrap()),
                block: None,
                // group: true,
                // Removed shrink to avoid hiding tiles
                ..Default::default()
            },
        )],
        5,
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            // Selector::Block(Rect::new(
            //     Coordinates { x: 2, y: 4 },
            //     Shape {
            //         width: 3 - 2,  // 1
            //         height: 4 - 4, // 0, maybe you want height = 1? Adjust if needed.
            //     },
            // )),
            Rect::new(
                Coordinates { x: 2, y: 4 },
                Shape {
                    width: 3 - 2,  // 1
                    height: 4 - 4, // 0, maybe you want height = 1? Adjust if needed.
                },
            )
            .as_many(),
            Effect {
                texture_id: Some(library.get_id("portal_1").unwrap()),
                action_id: Some(library.get_id("consolelog").unwrap()),
                ..Default::default()
            },
        )],
        6,
    );

    // Create base map
    let base_map = Map::new(
        "home".to_string(),
        vec![
            ground_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
        Coordinates { x: 0, y: 0 },
    );

    // Merge maps at consistent offsets like in use_map1
    let mut map = base_map.clone();
    map.merge_at(
        &base_map,
        Coordinates {
            x: SHAPE_WIDTH,
            y: 0,
        },
        None,
    );
    map.merge_at(
        &base_map,
        Coordinates {
            x: 0,
            y: SHAPE_HEIGHT,
        },
        None,
    );
    map.merge_at(
        &base_map,
        Coordinates {
            x: SHAPE_WIDTH,
            y: SHAPE_HEIGHT,
        },
        None,
    );

    map
}
