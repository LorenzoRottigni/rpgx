use dioxus::prelude::*;
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
pub fn Test() -> Element {
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

    let single_map = rpgx::factory::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        w_library.get_key_id("building_1"),
        w_library.get_key_id("consolelog"),
    );
    let mut map = single_map.clone();
    map.expand_at(&single_map.clone(), Coordinates { x: 4, y: 0 });
    map.expand_at(&single_map.clone(), Coordinates { x: 0, y: 6 });
    map.expand_at(&single_map.clone(), Coordinates { x: 4, y: 6 });

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
