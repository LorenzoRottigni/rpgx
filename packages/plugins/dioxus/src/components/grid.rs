use std::any::Any;

use dioxus::prelude::*;
use rpgx::traits::Renderable;
use rpgx::{engine::Engine, library::Library, prelude::RPGXError};

use crate::components::tile::Tile;

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    pub engine: Signal<Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: u32,
    pub onclick: EventHandler<Result<rpgx::prelude::Tile, RPGXError>>,
}

#[allow(non_snake_case)]
pub fn Grid(props: GridProps) -> Element {
    let engine = props.engine.read();
    if let Some(scene) = engine.get_active_scene() {
        rsx! {
            {
                scene
                    .map
                    .render()
                    .into_iter()
                    .enumerate()
                    .map(|(i, tile)| rsx! {
                        Tile {
                            key: "tile-{i}",
                            tile: tile.clone(),
                            layer_z: i as u32 + 1,
                            square_size: props.square_size,
                            library: props.library.clone(),
                            onclick: props.onclick.clone(),
                        }
                    })
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
