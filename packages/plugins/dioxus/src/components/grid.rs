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
    let get_background = |texture_id: u32| {
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
    };
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
                                let mask_texture = mask.get_texture();
                                let background = if let Some(texture_id) = mask_texture {
                                    get_background(texture_id)
                                } else {
                                    "background-size: cover;".to_string()
                                };
                                mask.tiles
                                    .iter()
                                    .flat_map(move |tile| {
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
                                            tile.shape.width * props.square_size,
                                            tile.shape.height * props.square_size,
                                            layer.z,
                                            "auto"
                                        );

                                        let onclick_tile = {
                                            let tile = tile.clone();
                                            move |_| {
                                                println!("onclick_tile");
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
