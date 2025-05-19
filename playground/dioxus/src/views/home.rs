use dioxus::prelude::*;
use rpgx::{
    common::{coordinates::Coordinates, shape::Shape},
    prelude::{Effect, Engine, Layer, LayerType, Map, Mask, Pawn, Selector},
};
use web_sys::console;

use crate::components::Engine;

const SQUARE_SIZE: i32 = 75;
const GRID_SIZE: i32 = 15;

#[component]
pub fn Home() -> Element {
    fn is_center_tile(pointer: Coordinates) -> bool {
        let x = pointer.x;
        let y = pointer.y;
        let center_x = GRID_SIZE / 2;
        let center_y = GRID_SIZE / 2;
        (x == center_x || x == center_x - 1) || (y == center_y || y == center_y - 1)
    }

    let default_layer = Layer::new(
        "base",
        LayerType::Default,
        Shape {
            height: GRID_SIZE,
            width: GRID_SIZE,
        },
        vec![],
    );

    let ground_layer = Layer::new(
        "ground",
        LayerType::Texture,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![
            Mask {
                name: "default_floor",
                effect: Effect {
                    texture: Some(asset!("/assets/rpg/spaceship_floor_1.webp")),
                    action: None,
                    block: false,
                    group: false,
                    shrink: None,
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
                name: "floor_alt",
                effect: Effect {
                    texture: Some(asset!("/assets/rpg/spaceship_floor_2.webp")),
                    action: None,
                    block: false,
                    group: false,
                    shrink: None,
                },
                selector: Selector::Filter(is_center_tile),
            },
        ],
    );

    let building_layer = Layer::new(
        "buildings",
        LayerType::Block,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![Mask {
            name: "logo",
            effect: Effect {
                texture: Some(asset!("/assets/rpg/k8sville_1.webp")),
                action: None,
                block: true,
                group: true,
                shrink: Some((Coordinates { x: 2, y: 7 }, Coordinates { x: 3, y: 10 })),
            },
            selector: Selector::Block((Coordinates { x: 1, y: 6 }, Coordinates { x: 4, y: 11 })),
        }],
    );

    let action_layer = Layer::new(
        "actions",
        LayerType::Action,
        Shape {
            width: GRID_SIZE,
            height: GRID_SIZE,
        },
        vec![Mask {
            name: "action_test",
            effect: Effect {
                texture: Some(asset!("/assets/rpg/portal_1.webp")),
                action: Some(|| {
                    let nav = use_navigator();
                    nav.push("/room/3");
                    // console::log_1(&"action".into());
                }),
                block: false,
                group: true,
                shrink: None,
            },
            selector: Selector::Block((Coordinates { x: 2, y: 11 }, Coordinates { x: 3, y: 11 })),
        }],
    );

    // let action_masks = Layer {
    //     name: "actions".to_string(),
    //     masks: [
    //         Mask {
    //             effect: engine::map::Effect {
    //                 texture: Some(asset!("/assets/rpg/portal_1.webp")),
    //                 block: false,
    //                 group: false,
    //                 action: Some(|| {
    //                     let nav = use_navigator();
    //                     nav.push("/room/3");
    //                 }),
    //             },
    //             shrink: None,
    //             selector: engine::map::Selector::Block((
    //                 Coordinates {
    //                     x: 11.0,
    //                     y: 14.0,
    //                 },
    //                 Coordinates { x: 12.0, y: 14.0 }
    //             ))
    //         }
    //     ].to_vec()
    // };

    let mut map = Map::new(
        "default",
        vec![
            default_layer.clone(),
            ground_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    );

    map.expand_at(
        &Map::new(
            "default",
            vec![
                default_layer.clone(),
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates { x: GRID_SIZE, y: 0 },
    );

    map.expand_at(
        &Map::new(
            "default",
            vec![
                default_layer.clone(),
                ground_layer.clone(),
                building_layer.clone(),
                action_layer.clone(),
            ],
        ),
        Coordinates { x: 0, y: GRID_SIZE },
    );

    map.expand_at(
        &Map::new(
            "default",
            vec![
                default_layer.clone(),
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

    map.expand_at(
        &Map::new(
            "default",
            vec![
                default_layer.clone(),
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

    // map.expand_at(
    //     &Map::new(
    //         "default",
    //         vec![
    //             default_layer.clone(),
    //             ground_layer.clone(),
    //             building_layer.clone(),
    //             action_layer.clone(),
    //         ],
    //     ),
    //     Direction::Down,
    // );
    //
    // map.expand_at(
    //     &Map::new(
    //         "default",
    //         vec![
    //             default_layer.clone(),
    //             ground_layer.clone(),
    //             building_layer.clone(),
    //             action_layer.clone(),
    //         ],
    //     ),
    //     Direction::Right,
    // );
    // map.expand_at(
    //     &Map::new(
    //         "default",
    //         vec![
    //             default_layer.clone(),
    //             ground_layer.clone(),
    //             building_layer.clone(),
    //             action_layer.clone(),
    //         ],
    //     ),
    //     Direction::Down,
    // );

    match map.get_base_layer() {
        Some(layer) => {
            if let Some(tile) = layer.get_tile(Coordinates { x: 0, y: 0 }) {
                let pawn = Pawn {
                    tile,
                    texture: asset!("/assets/rpg/character_1.webp"),
                };
                let engine = use_signal(|| Engine::new(map, pawn));
                rsx! {
                    div { class: "cluster",
                        Engine { engine, square_size: SQUARE_SIZE }
                    }
                }
            } else {
                rsx! {
                    div { "no base tile" }
                }
            }
        }
        None => rsx! {
            div { "no base layer" }
        }, // xengine.write().move_to(Coordinates { x: (GRID_SIZE / 2) as f32, y: (GRID_SIZE / 2) as f32 });
    }
}
