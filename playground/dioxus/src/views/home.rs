use dioxus::{prelude::*};
use rpgx::{
    common::{coordinates::Coordinates, shape::Shape},
    library::ResourceLibrary,
    prelude::{Effect, Engine, Layer, LayerType, Map, Mask, Pawn, Selector},
};

// Platform-agnostic logger
fn log_message(message: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&message.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{message}");
}

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

    let mut library = use_signal(|| ResourceLibrary::new());

    {
        let mut w_library = library.write();

        w_library.insert_texture(
            "floor_1",
            "https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp".to_string(),
        );
        w_library.insert_texture(
            "floor_2",
            "https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp".to_string(),
        );
        w_library.insert_texture(
            "building_1",
            "https://s3.rottigni.tech/rpgx/processor_8.webp".to_string(),
        );
        w_library.insert_texture(
            "portal_1",
            "https://s3.rottigni.tech/rpgx/portal_1.webp".to_string(),
        );
        w_library.insert_texture(
            "character_1",
            "https://s3.rottigni.tech/rpgx/character_1.webp".to_string(),
        );

        // Platform-agnostic action
        w_library.insert_action("consolelog", || {
            log_message("Hello from Rust!");
        });
    }

    let w_library = library.read();

    let default_layer = Layer::new(
        "base".to_string(),
        LayerType::Default,
        Shape {
            height: GRID_SIZE,
            width: GRID_SIZE,
        },
        vec![],
    );

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
                    texture_id: Some(w_library.get_key_id("floor_1")),
                    action_id: None,
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
                name: "floor_alt".to_string(),
                effect: Effect {
                    texture_id: Some(w_library.get_key_id("floor_2")),
                    action_id: None,
                    block: false,
                    group: false,
                    shrink: None,
                },
                selector: Selector::Filter(is_center_tile),
            },
        ],
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
                texture_id: Some(w_library.get_key_id("building_1")),
                action_id: None,
                block: true,
                group: true,
                shrink: Some((Coordinates { x: 2, y: 7 }, Coordinates { x: 3, y: 10 })),
            },
            selector: Selector::Block((Coordinates { x: 1, y: 6 }, Coordinates { x: 4, y: 11 })),
        }],
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
                texture_id: Some(w_library.get_key_id("portal_1")),
                action_id: Some(w_library.get_key_id("consolelog")),
                block: false,
                group: false,
                shrink: None,
            },
            selector: Selector::Block((Coordinates { x: 2, y: 11 }, Coordinates { x: 3, y: 11 })),
        }],
    );

    let mut map = Map::new(
        "default".to_string(),
        vec![
            default_layer.clone(),
            ground_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    );

    map.expand_at(
        &Map::new(
            "default".to_string(),
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
            "default".to_string(),
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
            "default".to_string(),
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
            "default".to_string(),
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

    match map.get_base_layer() {
        Some(layer) => {
            if let Some(tile) = layer.get_tile(Coordinates { x: 0, y: 0 }) {
                let pawn = Pawn {
                    tile,
                    texture_id: w_library.get_key_id("character_1"),
                };
                let engine = use_signal(|| Engine::new(map, pawn));
                rsx! {
                    div { class: "cluster",
                        Engine { engine, square_size: SQUARE_SIZE, library }
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
        },
    }
}
