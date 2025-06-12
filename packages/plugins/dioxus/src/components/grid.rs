use std::any::Any;

use dioxus::prelude::*;
use rpgx::{common::errors::MapError, engine::Engine, library::Library};

use crate::components::tile::Tile;

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    pub engine: Signal<Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: u32,
    pub onclick: EventHandler<Result<rpgx::prelude::Tile, MapError>>,
}

#[allow(non_snake_case)]
pub fn Grid(props: GridProps) -> Element {
    let engine = props.engine.read();
    if let Some(scene) = engine.get_active_scene() {
        rsx! {
            {
                scene
                    .map
                    .layers
                    .iter()
                    .enumerate()
                    .flat_map(|(layer_index, layer)| {
                        layer
                            .tiles
                            .iter()
                            .enumerate()
                            .map(move |(i, tile)| {
                                rsx! {
                                    Tile {
                                        key: "{layer_index}-{i}",
                                        tile: tile.clone(),
                                        layer_z: layer.z,
                                        // layer_kind: layer.kind,
                                        square_size: props.square_size,
                                        library: props.library.clone(),
                                        onclick: props.onclick.clone(),
                                    }
                                }
                            })
                    })
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
