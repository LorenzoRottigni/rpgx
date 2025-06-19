use std::{any::Any, convert::TryFrom};

use dioxus::prelude::*;
use rpgx::{engine::Engine, library::Library};

#[derive(PartialEq, Props, Clone)]
pub struct PawnProps {
    pub engine: Signal<Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: u32,
}

#[allow(non_snake_case)]
pub fn Pawn(props: PawnProps) -> Element {
    let engine = props.engine.read();

    if let Some(scene) = engine.get_active_scene() {
        let pawn = scene.pawn.as_ref().unwrap();
        let pawn_pos = pawn.pointer;
        let library = props.library.read();
        let default_texture = String::new();
        let pawn_texture = library
            .get_by_id(pawn.texture_id)
            .and_then(|boxed| boxed.downcast_ref::<String>())
            .unwrap_or(&default_texture);

        // Safely calculate pixel position
        let left = pawn_pos
            .x
            .checked_mul(props.square_size)
            .and_then(|v| i32::try_from(v).ok())
            .unwrap_or(0);

        let top = pawn_pos
            .y
            .checked_mul(props.square_size)
            .and_then(|v| i32::try_from(v).ok())
            .map(|v| v.saturating_sub(props.square_size as i32))
            .unwrap_or(0);

        rsx! {
            div {
                id: "pawn",
                class: "pawn",
                style: format!(
                    "position: absolute; \
                                                                                                                     left: {}px; \
                                                                                                                     top: {}px; \
                                                                                                                     background-image: url({}); \
                                                                                                                     background-size: cover; \
                                                                                                                     background-position: center center; \
                                                                                                                     z-index: 999; \
                                                                                                                     width: {}px; \
                                                                                                                     height: {}px; \
                                                                                                                     transition: all 0.1s;",
                    left,
                    top,
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
