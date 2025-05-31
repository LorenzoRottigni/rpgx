use dioxus::prelude::*;
use rpgx::{
    common::errors::MapError,
    library::ResourceLibrary,
    prelude::{Direction, Tile},
};

use crate::{
    components::{grid::Grid, pawn::Pawn},
    controller::{Command, use_controller},
};

#[derive(PartialEq, Props, Clone)]
pub struct EngineProps {
    pub engine: Signal<rpgx::prelude::Engine>,
    pub library: Signal<ResourceLibrary>,
    pub square_size: i32,
}

#[allow(non_snake_case)]
pub fn Engine(props: EngineProps) -> Element {
    let engine = props.engine.clone();
    let controller = use_controller(engine.clone());

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
            Grid {
                engine: engine.clone(),
                library: props.library.clone(),
                square_size: props.square_size,
                onclick: EventHandler::new(move |tile: Result<Tile, MapError>| {
                    if let Ok(tile) = tile {
                        let _ = onclick(tile);
                    }
                }),
            }
            Pawn {
                engine: engine.clone(),
                library: props.library.clone(),
                square_size: props.square_size,
            }
        }
    }
}
