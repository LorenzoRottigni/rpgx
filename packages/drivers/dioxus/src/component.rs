use crate::controller::Command;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use rpgx::common::coordinates::Coordinates;
use rpgx::common::direction::Direction;
use rpgx::common::errors::MapError;
use rpgx::library::ResourceLibrary;
use rpgx::prelude::{Engine, LayerType, Tile};

#[derive(PartialEq, Props, Clone)]
pub struct DioxusDriverProps {
    engine: Signal<Engine>,
    library: Signal<ResourceLibrary>,
    square_size: i32,
}

#[allow(non_snake_case)]
pub fn Engine(props: DioxusDriverProps) -> Element {
    let controller = use_controller(props.clone());

    let onclick = move |tile: Tile| -> Result<(), MapError> {
        controller.send(Command::WalkTo(tile.pointer));
        Ok(())
    };

    let onkeydown = {
        move |evt: KeyboardEvent| {
            let direction = match evt.key() {
                Key::ArrowUp => Some(Direction::Up),
                Key::ArrowDown => Some(Direction::Down),
                Key::ArrowLeft => Some(Direction::Left),
                Key::ArrowRight => Some(Direction::Right),
                Key::Character(k) => match k.as_str() {
                    "w" | "W" => Some(Direction::Up),
                    "s" | "S" => Some(Direction::Down),
                    "a" | "A" => Some(Direction::Left),
                    "d" | "D" => Some(Direction::Right),
                    _ => None,
                },
                _ => None,
            };

            if let Some(d) = direction {
                controller.send(Command::Step(d));
            }
        }
    };

    rsx! {
        div {
            class: "container",
            tabindex: "0",
            onkeydown,
            style: "position: relative;",

            {
                engine
                    .read()
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
                            .map(move |(_i, tile)| {
                                let background = if let Some(texture_id) = tile.effect.texture_id
                                {
                                    if let Some(asset) = props
                                        .library
                                        .read()
                                        .get_texture_by_id(texture_id)
                                    {
                                        format!(
                                            "background-image: url({}); background-size: cover;",
                                            asset,
                                        )
                                    } else {
                                        "background-size: cover;".to_string()
                                    }
                                } else {
                                    "background-size: cover;".to_string()
                                };
                                let x = tile.pointer.x;
                                let y = tile.pointer.y;
                                let base_style = format!(
                                    "{background} \
                                                                                                                        position: absolute; \
                                                                                                                        left: {}px; \
                                                                                                                        top: {}px; \
                                                                                                                        width: {}px; \
                                                                                                                        height: {}px; \
                                                                                                                        border: solid 1px rgba(255,255,255,0.1); \
                                                                                                                        opacity: 0.7; \
                                                                                                                        z-index: {}; \
                                                                                                                        pointer-events: {}; \
                                                                                                                        cursor: pointer;",
                                    x * props.square_size,
                                    y * props.square_size,
                                    if tile.effect.group { tile.shape.width } else { 1 }
                                        * props.square_size,
                                    if tile.effect.group { tile.shape.height } else { 1 }
                                        * props.square_size,
                                    layer.z,
                                    if layer.kind == LayerType::Base { "auto" } else { "none" },
                                );
                                let onclick_handler = if layer.kind == LayerType::Base {
                                    let _tile = tile.clone();
                                    rsx! {
                                        div {
                                            class: "base-layer-tile",
                                            key: {format!("layer-{}-{}", layer_index, i)},
                                            style: "{base_style}",
                                            onclick: move |_| {
                                                let _ = onclick(_tile);
                                            },
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div {
                                            class: "layer-tile",
                                            key: {format!("layer-{}-{}", layer_index, i)},
                                            style: "{base_style}",
                                        }
                                    }
                                };
                                onclick_handler
                            })
                    })
            }

            div {
                class: "pawn",
                style: "position: absolute; \
                        left: {engine.read().pawn.tile.pointer.x * props.square_size}px; \
                        top: {engine.read().pawn.tile.pointer.y * props.square_size - props.square_size}px; \
                        background-image: url({props.library.read().get_texture_by_id(engine.read().pawn.texture_id).unwrap()}); \
                        background-size: cover; \
                        background-position: center center; \
                        z-index: 100; \
                        width: {props.square_size}px; \
                        height: {props.square_size * 2}px; \
                        transition: all 0.1s;",
            }
        }
    }
}
