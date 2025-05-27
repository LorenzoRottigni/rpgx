use crate::controller::{Command, use_controller};
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use log::error;
use rpgx::common::coordinates::Coordinates;
use rpgx::common::direction::Direction;
use rpgx::common::errors::MapError;
use rpgx::library::ResourceLibrary;
use rpgx::prelude::{Engine, LayerType, Tile};

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    engine: Signal<Engine>,
    library: Signal<ResourceLibrary>,
    square_size: i32,
}

#[allow(non_snake_case)]
pub fn Engine(props: GridProps) -> Element {
    let engine = props.engine.clone();

    let movement = use_controller(engine.clone());

    let onclick = move |tile: Tile| -> Result<(), MapError> {
        movement.send(Command::WalkTo(tile.pointer));
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
                movement.send(Command::Step(d));
            }
        }
    };

    let engine_ref = engine.read();
    let pawn_pos = engine_ref.pawn.tile.pointer;
    let pawn_texture = props
        .library
        .read()
        .get_texture_by_id(engine_ref.pawn.texture_id)
        .cloned()
        .unwrap_or_default();

    rsx! {
        div {
            class: "container",
            tabindex: "0",
            onkeydown,
            style: "position: relative;",
            {
                (engine_ref)
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
                            .filter_map(move |(i, tile)| {
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
                                if layer.kind == LayerType::Base {
                                    let _tile = tile.clone();
                                    rsx! {
                                        div {
                                            class: "base-layer-tile",
                                            key: format!("layer-{}-{}", layer_index, i),
                                            style: "{base_style}",
                                            onclick: move |_| {
                                                let _ = onclick(_tile);
                                            },
                                        }
                                    }.ok()
                                } else {
                                    rsx! {
                                        div {
                                            class: "layer-tile",
                                            key: format!("layer-{}-{}", layer_index, i),
                                            style: "{base_style}",
                                        }
                                    }.ok()
                                }
                            })
                    })
            }


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
    }
}
