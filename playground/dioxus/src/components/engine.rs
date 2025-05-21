use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use rpgx::common::coordinates::Coordinates;
use rpgx::common::direction::Direction;
use rpgx::common::errors::MapError;
use rpgx::engine::library::{self, ResourceLibrary};
use rpgx::prelude::{Engine, LayerType, Tile};
use web_sys::console;

#[derive(PartialEq, Props, Clone)]
pub struct GridProps {
    engine: Signal<Engine>,
    library: Signal<ResourceLibrary>,
    square_size: i32,
}

#[derive(Clone)]
enum Command {
    WalkTo(Coordinates),
}

pub fn Engine(props: GridProps) -> Element {
    let mut engine = props.engine.clone();

    let movement = use_coroutine({
        to_owned![engine];
        move |mut rx: UnboundedReceiver<Command>| async move {
            while let Some(command) = rx.next().await {
                let result: Result<(), Box<dyn std::error::Error>> = async {
                    match command {
                        Command::WalkTo(target) => {
                            let steps = engine
                                .read()
                                .map
                                .find_path(&engine.read().pawn.tile.pointer, &target);
                            match steps {
                                None => {
                                    console::error_1(&"Path not found".into());
                                    return Err(Box::<dyn std::error::Error>::from(
                                        "Path not found",
                                    ));
                                }
                                Some(steps) => {
                                    for step in steps {
                                        gloo_timers::future::TimeoutFuture::new(100).await;
                                        engine.write().move_to(step).map_err(|e| {
                                            Box::<dyn std::error::Error>::from(format!("{:?}", e))
                                        })?;
                                    }
                                    Ok(())
                                }
                            }
                        }
                    }
                }
                .await;

                if let Err(e) = result {
                    console::error_1(&format!("Movement error: {:?}", e).into());
                }
            }
        }
    });

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
                let mut engine_w = engine.write();
                if let Ok(tile) = engine_w.step_to(d) {
                    let action_ids = engine_w.map.get_actions_at(tile.pointer);
                    for action_id in action_ids {
                        if let Some(action) = props.library.read().get_action_by_id(action_id) {
                            action()
                        }
                    }
                }
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
                    .enumerate()
                    .flat_map(|(layer_index, layer)| {
                        layer
                            .tiles
                            .iter()
                            .enumerate()
                            .map(move |(i, tile)| {
                                let background = if let Some(texture_id) = tile.effect.texture_id {
                                    if let Some(asset) = props.library.read().get_texture_by_id(texture_id) {
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
                                    if layer.kind == LayerType::Default {
                                        999
                                    } else {
                                        5 + layer_index
                                    },
                                    if layer.kind == LayerType::Default {
                                        "auto"
                                    } else {
                                        "none"
                                    },
                                );
                                let onclick_handler = if layer.kind == LayerType::Default {
                                    let _tile = tile.clone();
                                    rsx! {
                                        div {
                                            class: "base-layer-tile",
                                            key: {format!("layer-{}-{}", layer_index, i)},
                                            style: "{base_style}",
                                            onclick: move |_| {
                                                let _ = onclick(_tile);
                                            },
                                        // "{tile.pointer.x};{tile.pointer.y};{layer_index}"
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
