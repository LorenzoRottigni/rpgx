use std::any::Any;

use dioxus::prelude::*;
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
                    .layers
                    .iter()
                    .enumerate()
                    .flat_map(|(layer_index, layer)| {
                        layer
                            .masks
                            .iter()
                            .flat_map(move |mask| {
                                mask.tiles
                                    .iter()
                                    .enumerate()
                                    .flat_map(move |(i, tile)| {
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
                            .collect::<Vec<_>>()
                    })
            }
        }
    } else {
        rsx! {
            div {}
        }
    }
}
