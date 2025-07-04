use crate::prelude::{Coordinates, Effect, Layer, Mask, Rect, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn ground_layer(shape: Shape, texture_id: u32) -> Layer {
    let mask = Mask::new(
        "street_border".to_string(),
        // Selector::Block(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
        Rect::new(Coordinates { x: 0, y: 0 }, shape).into_many(),
        vec![Effect::Texture(texture_id)],
    );

    Layer::new("street".to_string(), vec![mask], 1)
}
