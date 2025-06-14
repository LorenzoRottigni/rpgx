use dioxus::prelude::*;
use rpgx::prelude::{Engine, Scene};

use crate::config::{library::use_library, map::map1::use_map1};

#[component]
pub fn Map1() -> Element {
    let library = use_signal(|| use_library());

    let map = use_map1(&library.read());

    let mut scene = Scene::new("default".into(), map, None);
    scene.load_pawn(library.read().get_id("character_1").unwrap());
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
