use dioxus::prelude::*;
use rpgx::{
    common::coordinates::Coordinates,
    prelude::{Engine, Pawn},
};

use crate::config::{library::use_library, map::map1::use_map1};

#[component]
pub fn Map1() -> Element {
    let library = use_signal(|| use_library());

    let map = use_map1(library.read().clone());

    match map.get_base_layer() {
        Some(layer) => {
            if let Some(tile) = layer.get_tile_at(Coordinates { x: 0, y: 0 }) {
                let pawn = Pawn {
                    tile,
                    texture_id: library.read().get_key_id("character_1"),
                };
                let engine = use_signal(|| Engine::new(map, pawn));
                rsx! {
                    div { class: "cluster",
                        rpgx_dioxus::component::Engine { engine, square_size: 75, library }
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
