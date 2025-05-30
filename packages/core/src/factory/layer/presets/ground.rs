use crate::prelude::{Coordinates, Effect, Layer, LayerType, Mask, Selector, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn ground_layer(shape: Shape, texture_id: i32) -> Layer {
    let mask = Mask {
        name: "street_border".to_string(),
        effect: Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
        selector: Selector::Block((
            Coordinates { x: 0, y: 0 },
            Coordinates {
                x: shape.width - 1,
                y: shape.height - 1,
            },
        )),
    };

    Layer::new(
        "street".to_string(),
        LayerType::Texture,
        shape,
        vec![mask],
        1,
    )
}
