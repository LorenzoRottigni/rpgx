use std::any::Any;

use rpgx::{
    library::Library,
    map::Map,
    prelude::{Coordinates, Effect, Layer, LayerType, Mask, Selector, Shape},
};

const GRID_SIZE: i32 = 12;

fn is_center_tile(pointer: Coordinates, _shape: Shape) -> bool {
    let x = pointer.x;
    let y = pointer.y;
    let center_x = GRID_SIZE / 2;
    let center_y = GRID_SIZE / 2;
    (x == center_x || x == center_x - 1) || (y == center_y || y == center_y - 1)
}

pub fn use_map3(library: Library<Box<dyn Any>>) -> Map {
    let ground_layer = Layer::new(
        "ground".to_string(),
        LayerType::Texture,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![
            Mask {
                name: "default_floor".to_string(),
                effect: Effect {
                    texture_id: Some(library.get_id("floor_1").unwrap()),
                    ..Default::default()
                },
                selector: Selector::Block((
                    Coordinates { x: 0, y: 0 },
                    Coordinates {
                        x: GRID_SIZE - 1,
                        y: GRID_SIZE - 1,
                    },
                )),
            },
            Mask {
                name: "floor_alt".to_string(),
                effect: Effect {
                    texture_id: Some(library.get_id("floor_2").unwrap()),
                    ..Default::default()
                },
                selector: Selector::Filter(is_center_tile),
            },
        ],
        1,
    );

    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![Mask {
            name: "logo".to_string(),
            effect: Effect {
                texture_id: Some(library.get_id("building_1").unwrap()),
                block: true,
                group: true,
                shrink: Some((Coordinates { x: 2, y: 7 }, Coordinates { x: 3, y: 10 })),
                ..Default::default()
            },
            selector: Selector::Block((Coordinates { x: 1, y: 6 }, Coordinates { x: 4, y: 11 })),
        }],
        5,
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        LayerType::Action,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![Mask {
            name: "action_test".to_string(),
            effect: Effect {
                texture_id: Some(library.get_id("portal_1").unwrap()),
                action_id: Some(library.get_id("consolelog").unwrap()),
                ..Default::default()
            },
            selector: Selector::Block((Coordinates { x: 2, y: 11 }, Coordinates { x: 3, y: 11 })),
        }],
        6,
    );

    let mut map = Map::new(
        "home".to_string(),
        vec![
            ground_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates { x: GRID_SIZE, y: 0 },
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates { x: 0, y: GRID_SIZE },
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates {
            x: 0,
            y: GRID_SIZE * 2,
        },
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates {
            x: GRID_SIZE,
            y: GRID_SIZE * 2,
        },
    );

    map
}
