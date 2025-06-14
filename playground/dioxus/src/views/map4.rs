use dioxus::prelude::*;
use rpgx::prelude::{Coordinates, Engine, Pawn, Scene};

use crate::config::{library::use_library, map::map4::use_map4};

#[component]
pub fn Map4() -> Element {
    let library = use_signal(|| use_library());

    let map = use_map4(&library.read());

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
}
