use crate::prelude::{Coordinates, Effect, Layer, Mask, Selector, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn ground_layer(shape: Shape, texture_id: u32) -> Layer {
    let mask = Mask::new(
        "street_border".to_string(),
        Selector::Block((
            Coordinates { x: 0, y: 0 },
            Coordinates {
                x: shape.width - 1,
                y: shape.height - 1,
            },
        )),
        Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
    );

    Layer::new("street".to_string(), vec![mask], 1)
}
