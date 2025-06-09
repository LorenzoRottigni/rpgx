use std::any::Any;

use dioxus::prelude::*;
use rpgx::{
    common::errors::MapError,
    library::Library,
    prelude::{Direction, Tile},
};

use crate::{
    components::{grid::Grid, pawn::Pawn},
    controller::{Command, use_controller},
};

#[derive(PartialEq, Props, Clone)]
pub struct EngineProps {
    pub engine: Signal<rpgx::prelude::Engine>,
    pub library: Signal<Library<Box<dyn Any>>>,
    pub square_size: u32,
}

#[allow(non_snake_case)]
pub fn Engine(props: EngineProps) -> Element {
    let engine = props.engine.clone();
    let controller = use_controller(engine.clone(), props.library.clone());

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

    use_effect(move || {
        let _ = engine(); // cause the effect to re-run when engine changes

        let js_code = r#"
            (() => {
                console.log('trigger update');
                const container = document.querySelector('#scroll-container');
                const pawn = document.querySelector('#pawn');
                if (!container || !pawn) return;
                const scrollX = pawn.offsetLeft + pawn.offsetWidth / 2 - container.clientWidth / 2;
                const scrollY = pawn.offsetTop + pawn.offsetHeight / 2 - container.clientHeight / 2;
                container.scrollTo({
                    left: scrollX,
                    top: scrollY,
                    behavior: 'smooth'
                });
            })();
        "#;

        #[cfg(feature = "web")]
        {
            let _ = js_sys::eval(js_code);
        }

        #[cfg(feature = "desktop")]
        {
            use dioxus_desktop::use_webview;

            if let Some(webview) = use_webview() {
                let _ = webview.eval(js_code);
            }
        }
    });

    rsx! {
        div {
            id: "scroll-container",
            class: "container",
            tabindex: "0",
            onkeydown,
            style: "position: relative; overflow: auto; width: 100vw; height: 100vh;",

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
