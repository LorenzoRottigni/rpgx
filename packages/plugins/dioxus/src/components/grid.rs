use dioxus::prelude::*;
use rpgx::{
    common::errors::MapError, engine::Engine, library::ResourceLibrary, prelude::LayerType,
};

use crate::components::tile::Tile;

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    pub engine: Signal<Engine>,
    pub library: Signal<ResourceLibrary>,
    pub square_size: i32,
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
                    .filter(|layer| layer.kind != LayerType::Texture)
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
                                        layer_kind: layer.kind,
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
