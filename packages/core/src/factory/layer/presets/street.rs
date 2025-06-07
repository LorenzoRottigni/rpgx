use crate::prelude::{Coordinates, Effect, Layer, LayerType, Mask, Selector, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn street_layer_around(shape: Shape, texture_id: u32) -> Layer {
    let outer_shape = Shape {
        width: shape.width + 2,
        height: shape.height + 2,
    };

    let mask = Mask {
        name: "street_border".to_string(),
        effect: Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
        selector: Selector::Filter(move |coords: Coordinates, shape: Shape| {
            coords.x == 0
                || coords.y == 0
                || coords.x == shape.width - 1
                || coords.y == shape.height - 1
        }),
    };

    Layer::new(
        "street".to_string(),
        LayerType::Texture,
        outer_shape,
        vec![mask],
        3,
    )
}
