use std::any::Any;

use rpgx::{
    library::Library,
    map::Map,
    prelude::{Coordinates, Effect, Layer, Mask, Selector, Shape},
};

const GRID_SIZE: u32 = 12;

fn is_center_tile(pointer: Coordinates, _shape: Shape) -> bool {
    let x = pointer.x as u32;
    let y = pointer.y as u32;
    let center_x = GRID_SIZE / 2;
    let center_y = GRID_SIZE / 2;
    (x == center_x || x == center_x - 1) || (y == center_y || y == center_y - 1)
}

pub fn use_map3(library: &Library<Box<dyn Any>>) -> Map {
    let ground_layer = Layer::new(
        "ground".to_string(),
        Shape {
            width: GRID_SIZE as u32,
            height: GRID_SIZE as u32,
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
                        x: (GRID_SIZE - 1) as u32,
                        y: (GRID_SIZE - 1) as u32,
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
        Shape {
            width: GRID_SIZE as u32,
            height: GRID_SIZE as u32,
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
        Shape {
            width: GRID_SIZE as u32,
            height: GRID_SIZE as u32,
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
        Coordinates { x: 0, y: 0 },
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
            Coordinates { x: 0, y: 0 },
        ),
        Coordinates {
            x: GRID_SIZE as u32,
            y: 0,
        },
        None,
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
            Coordinates { x: 0, y: 0 },
        ),
        Coordinates {
            x: 0,
            y: GRID_SIZE as u32,
        },
        None,
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
            Coordinates { x: 0, y: 0 },
        ),
        Coordinates {
            x: 0,
            y: GRID_SIZE as u32 * 2,
        },
        None,
    );

    map.merge_at(
        &Map::new(
            "home".to_string(),
            vec![
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
            Coordinates { x: 0, y: 0 },
        ),
        Coordinates {
            x: GRID_SIZE as u32,
            y: GRID_SIZE as u32 * 2,
        },
        None,
    );

    map
}
