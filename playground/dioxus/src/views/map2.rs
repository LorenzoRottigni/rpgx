use dioxus::prelude::*;
use rpgx::{
    common::{coordinates::Coordinates, shape::Shape},
    library::ResourceLibrary,
    prelude::{Effect, Engine, Layer, LayerType, Map, Mask, Pawn, Selector, Direction},
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
pub fn Map2() -> Element {
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
            "floor_3",
            "https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp".to_string(),
        );
        w_library.insert_texture(
            "building_1",
            "https://s3.rottigni.tech/rpgx/processor_8.webp".to_string(),
        );
        w_library.insert_texture(
            "building_2",
            "https://s3.rottigni.tech/rpgx/processor_9.webp".to_string(),
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

        w_library.insert_action("teleport", || {

        });
    }

    let w_library = library.read();

    let mut building_1 = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        w_library.get_key_id("building_1"),
        w_library.get_key_id("consolelog"),
    );
    building_1.load_layer(
        rpgx::factory::layer::presets::ground::ground_layer(
            Shape {
                width: 6,
                height: 8,
            }, 
            w_library.get_key_id("floor_1")
        ),
    );
    building_1.load_layer(
        rpgx::factory::layer::presets::street::street_layer_around(
            Shape {
                width: 4,
                height: 6,
            }, 
            w_library.get_key_id("floor_2")
        ),
    );
    building_1.load_layer(
        rpgx::factory::layer::presets::street::street_layer_around(
            Shape {
                width: 6,
                height: 8,
            }, 
            w_library.get_key_id("floor_2")
        ),
    );

    let mut building_2 = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        w_library.get_key_id("building_2"),
        w_library.get_key_id("consolelog"),
    );
    building_2.load_layer(
        rpgx::factory::layer::presets::ground::ground_layer(
            Shape {
                width: 6,
                height: 8,
            }, 
            w_library.get_key_id("floor_1")
        ),
    );
    building_2.load_layer(
        rpgx::factory::layer::presets::street::street_layer_around(
            Shape {
                width: 4,
                height: 6,
            }, 
            w_library.get_key_id("floor_2")
        ),
    );
    building_2.load_layer(
        rpgx::factory::layer::presets::street::street_layer_around(
            Shape {
                width: 6,
                height: 8,
            }, 
            w_library.get_key_id("floor_2")
        ),
    );

    let mut map = Map::compose(
        "TestMap".to_string(),
        vec![
            (building_1, Coordinates { x: 0, y: 0 }),
            (building_2, Coordinates { x: 8, y: 0 }),
        ],
        vec![]
    );

    map.load_layer(
        Layer::new(
            "ground_decoration".to_string(),
            LayerType::Texture,
            map.get_shape(),
            vec![
                Mask::new(
                    "ground_decoration".to_string(),
                    Selector::Filter(|pointer, shape| pointer.x == 0 || pointer.y == 0 || pointer.x == shape.width - 1 || pointer.y == shape.height - 1),
                    Effect { action_id: None, texture_id: Some(w_library.get_key_id("floor_3")), block: false, group: false, shrink: None }
                ),
            ],
            1
        )
    );

    map.duplicate_to_the(Direction::Right);
    map.duplicate_to_the(Direction::Down);

    match map.get_base_layer() {
        Some(layer) => {
            if let Some(tile) = layer.get_tile_at(Coordinates { x: 0, y: 0 }) {
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
