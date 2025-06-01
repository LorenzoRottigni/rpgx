use dioxus::prelude::*;
use rpgx::{common::errors::MapError, library::Library, prelude::LayerType};
use std::any::Any;

#[derive(PartialEq, Props, Clone)]
pub struct TileProps {
    tile: rpgx::prelude::Tile,
    layer_z: i32,
    layer_kind: LayerType,
    square_size: i32,
    library: Signal<Library<Box<dyn Any>>>,
    onclick: EventHandler<Result<rpgx::prelude::Tile, MapError>>,
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

    let x = props.tile.pointer.x;
    let y = props.tile.pointer.y;

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
        if props.tile.effect.group {
            props.tile.shape.width
        } else {
            1
        } * props.square_size,
        if props.tile.effect.group {
            props.tile.shape.height
        } else {
            1
        } * props.square_size,
        props.layer_z,
        if props.layer_kind == LayerType::Base {
            "auto"
        } else {
            "none"
        }
    );

    let onclick_tile = {
        let tile = props.tile.clone();
        move |_| {
            if props.layer_kind == LayerType::Base {
                let _ = props.onclick.call(Ok(tile.clone()));
            }
        }
    };

    let library = props.library.read();

    let render_fn_opt: Option<&Box<dyn Fn() -> Result<VNode, RenderError>>> =
        props.tile.effect.render_id.and_then(|render_id| {
            library.get_by_id(render_id).and_then(|boxed| {
                boxed.downcast_ref::<Box<dyn Fn() -> Result<VNode, RenderError>>>()
            })
        });

    // Now run the render function, handling the Result<VNode, RenderError> properly:
    let rendered_element: VNode = if let Some(render_fn) = render_fn_opt {
        render_fn().unwrap()
    } else {
        rsx! {}.unwrap()
    };

    rsx! {
        div {
            class: if props.layer_kind == LayerType::Base { "base-layer-tile" } else { "layer-tile" },
            style: "{base_style}",
            onclick: onclick_tile,

            {rendered_element}
        }
    }
}
