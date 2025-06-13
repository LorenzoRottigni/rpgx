use crate::{
    common::delta::Delta,
    prelude::{Coordinates, Effect, Layer, Mask, Selector, Shape},
};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn street_layer_around(shape: Shape, texture_id: u32) -> Layer {
    // Outer shape is 2 tiles bigger in both width and height
    let outer_shape = Shape {
        width: shape.width + 2,
        height: shape.height + 2,
    };

    let mut edge_coords = Vec::new();

    // Top and Bottom edges
    for x in 0..outer_shape.width {
        edge_coords.push(Coordinates { x, y: 0 }); // Top
        edge_coords.push(Coordinates {
            x,
            y: outer_shape.height - 1,
        }); // Bottom
    }

    // Left and Right edges
    for y in 1..(outer_shape.height - 1) {
        edge_coords.push(Coordinates { x: 0, y }); // Left
        edge_coords.push(Coordinates {
            x: outer_shape.width - 1,
            y,
        }); // Right
    }

    let mask = Mask::new(
        "street_border".to_string(),
        Selector::Sparse(edge_coords),
        Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
    );

    // Offset inward to wrap the original shape
    let street_layer = Layer::new("street".to_string(), vec![mask], 3);

    // street_layer.offset(Delta { dx: -1, dy: -1 });

    street_layer
}
