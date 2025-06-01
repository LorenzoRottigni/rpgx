use std::any::Any;

use dioxus::prelude::*;
use rpgx::{engine::Engine, library::Library};

#[derive(PartialEq, Props, Clone)]
pub struct PawnProps {
    pub engine: Signal<Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: i32,
}

#[allow(non_snake_case)]
pub fn Pawn(props: PawnProps) -> Element {
    let engine = props.engine.read();
    if let Some(scene) = engine.get_active_scene() {
        let pawn_pos = scene.pawn.tile.pointer;
        let library = props.library.read();
        let pawn_texture = library
            .get_by_id(scene.pawn.texture_id)
            .and_then(|boxed| boxed.downcast_ref::<String>())
            .unwrap();

        rsx! {
            div {
                class: "pawn",
                style: format!(
                    "position: absolute; \
                    left: {}px; \
                    top: {}px; \
                    background-image: url({}); \
                    background-size: cover; \
                    background-position: center center; \
                    z-index: 100; \
                    width: {}px; \
                    height: {}px; \
                    transition: all 0.1s;",
                    pawn_pos.x * props.square_size,
                    pawn_pos.y * props.square_size - props.square_size,
                    pawn_texture,
                    props.square_size,
                    props.square_size * 2,
                ),
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
