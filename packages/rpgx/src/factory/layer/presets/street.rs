use crate::prelude::{Coordinates, Effect, Layer, LayerType, Mask, Selector, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn street_layer_around(shape: Shape, texture_id: i32) -> Layer {
    let outer_shape = Shape {
        width: shape.width + 2,
        height: shape.height + 2,
    };

    let mask = Mask {
        name: "street_border".to_string(),
        effect: Effect {
            texture_id: Some(texture_id),
            action_id: None,
            block: false,
            group: false,
            shrink: None,
        },
        selector: Selector::Block((
            Coordinates { x: 0, y: 0 },
            Coordinates {
                x: outer_shape.width,
                y: outer_shape.height,
            },
        )),
    };

    Layer::new(
        "street".to_string(),
        LayerType::Texture,
        outer_shape,
        vec![mask],
    )
}
