use dioxus::prelude::*;
use rpgx::prelude::{Coordinates, Engine, Pawn, Scene};

use crate::config::{library::use_library, map::map2::use_map2};

#[component]
pub fn Map2() -> Element {
    let library = use_signal(|| use_library());

    let map = use_map2(&library.read());

    let pawn = Pawn {
        pointer: Coordinates { x: 0, y: 0 },
        texture_id: library.read().get_id("character_1").unwrap(),
    };
    let mut scene = Scene::new("default".into(), map, None);
    scene.load_pawn_at(pawn);
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
    //             let pawn = Pawn {
    //                 pointer: tile.pointer,
    //                 texture_id: library.read().get_id("character_1").unwrap(),
    //             };
    //             let mut scene = Scene::new("default".into(), map, None);
    //             scene.load_pawn_at(pawn);
    //             let engine = use_signal(|| Engine::new(scene));
    //
    //             rsx! {
    //                 div { class: "cluster",
    //                     rpgx_dioxus::components::engine::Engine {
    //                         engine: engine.clone(),
    //                         library: library.clone(),
    //                         square_size: 32,
    //                     }
    //                 }
    //             }
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
