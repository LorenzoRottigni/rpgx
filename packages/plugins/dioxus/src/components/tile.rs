use dioxus::prelude::*;
use rpgx::{library::Library, prelude::MoveError};
use std::any::Any;
use web_sys::console;

#[derive(PartialEq, Props, Clone)]
pub struct TileProps {
    tile: rpgx::prelude::Tile,
    layer_z: u32,
    // layer_kind: LayerType,
    square_size: u32,
    library: Signal<Library<Box<dyn Any>>>,
    onclick: EventHandler<Result<rpgx::prelude::Tile, MoveError>>,
}

#[allow(non_snake_case)]
pub fn Tile(props: TileProps) -> Element {
    let background = if let Some(texture_id) = props.tile.effect.texture_id {
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

    let x = props.tile.area.origin.x;
    let y = props.tile.area.origin.y;

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
        props.tile.area.shape.width * props.square_size,
        // if props.tile.effect.group {
        //     props.tile.area.shape.height
        // } else {
        //     1
        // } * props.square_size,
        props.tile.area.shape.height * props.square_size,
        props.layer_z,
        // if props.layer_kind == LayerType::Base {
        //     "auto"
        // } else {
        //     "none"
        // }
        "auto"
    );

    let onclick_tile = {
        let tile = props.tile.clone();
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
                props
                    .tile
                    .effect
                    .render_id
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
}
