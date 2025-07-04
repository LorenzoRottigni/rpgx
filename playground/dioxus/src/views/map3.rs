use dioxus::prelude::*;
use rpgx::prelude::{Coordinates, Engine, Pawn, Scene};

use crate::config::{library::use_library, map::map3::use_map3};

#[component]
pub fn Map3() -> Element {
    let library = use_signal(|| use_library());

    let map = use_map3(&library.read());

    let pawn = Pawn {
        pointer: Coordinates { x: 0, y: 0 },
        texture_id: library.read().get_id("character_1").unwrap(),
    };
    let scene = Scene::new("default".into(), map, Some(pawn));
    let engine = use_signal(|| Engine::new(scene));
    rsx! {
        div { class: "cluster",
            rpgx_dioxus::components::engine::Engine {
                engine: engine.clone(),
                library: library.clone(),
                square_size: 32,
            }
        }
    }
    // map.get_tile() ...
    // match map.get_base_layer() {
    //     Some(layer) => {
    //         if let Some(tile) = layer.get_tile_at(Coordinates { x: 0, y: 0 }) {
    //
    //         } else {
    //             rsx! {
    //                 div { "no base tile" }
    //             }
    //         }
    //     }
    //     None => rsx! {
    //         div { "no base layer" }
    //     },
    // }
}
