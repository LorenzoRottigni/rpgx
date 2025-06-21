use std::any::Any;

use dioxus::prelude::*;
use rpgx::{engine::Engine, library::Library, prelude::RPGXError};

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    pub engine: Signal<Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: u32,
    pub onclick: EventHandler<Result<rpgx::prelude::Rect, RPGXError>>,
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
                    .flat_map(|layer| {
                        layer
                            .masks
                            .iter()
                            .flat_map(move |mask| {
                                mask.tiles
                                    .iter()
                                    .flat_map(move |tile| {
                                        let background = if let Some(texture_id) = mask.get_texture() {
                                        if let Some(asset) = props
                                            .library
                                            .read()
                                            .get_by_id(texture_id)
                                            .and_then(|boxed| boxed.downcast_ref::<String>())
                                        {
                                            format!("background-image: url({}); background-size: cover;", asset)
                                        } else {
                                            "background-size: cover;".to_string()
                                        }
                                    } else {
                                        "background-size: cover;".to_string()
                                    };

                                    let x = tile.origin.x;
                                    let y = tile.origin.y;

                                    let base_style = format!(
                                        "{background} \
                                        position: absolute; \
                                        left: {}px; \
                                        top: {}px; \
                                        width: {}px; \
                                        height: {}px; \
                                        border: solid 1px rgba(255,255,255,0.1); \
                                        z-index: {}; \
                                        pointer-events: {}; \
                                        cursor: pointer;",
                                        x * props.square_size,
                                        y * props.square_size,
                                        // if props.tile.effect.group {
                                        //     props.tile.area.shape.width
                                        // } else {
                                        //     1
                                        // } * props.square_size,
                                        tile.shape.width * props.square_size,
                                        // if props.tile.effect.group {
                                        //     props.tile.area.shape.height
                                        // } else {
                                        //     1
                                        // } * props.square_size,
                                        tile.shape.height * props.square_size,
                                        layer.z,
                                        // if props.layer_kind == LayerType::Base {
                                        //     "auto"
                                        // } else {
                                        //     "none"
                                        // }
                                        "auto"
                                    );

                                    let onclick_tile = {
                                        let tile = tile.clone();
                                        // console::log_1(&"onclick_tile".into());
                                        move |_| {
                                            println!("onclick_tile");
                                            // console::log_1(&"onclick_tile_emit_props".into());
                                            let _ = props.onclick.call(Ok(tile.clone()));
                                        }
                                    };

                                    let library = props.library.read();
                                        rsx! {
                                            div {
                                                class: "layer-tile",
                                                style: "{base_style}",
                                                onclick: onclick_tile,
                                                {
                                                    mask
                                                        .get_render()
                                                        .and_then(|id| {
                                                            println!(
                                                                "Rendering custom VNode from library {:?}",
                                                                library.get_by_id(id),
                                                            );
                                                            let f = library.get_by_id(id)?.downcast_ref::<Box<dyn Fn() -> VNode>>()?;
                                                            Some(f())
                                                        })
                                                        .unwrap_or(rsx! {}.unwrap())
                                                }
                                            }
                                            // Tile {
                                            //     key: "{layer_index}-{i}",
                                            //     tile: tile.clone(),
                                            //     layer_z: layer.z,
                                            //     // layer_kind: layer.kind,
                                            //     square_size: props.square_size,
                                            //     library: props.library.clone(),
                                            //     onclick: props.onclick.clone(),
                                            // }
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
