use crate::prelude::{Coordinates, Effect, Layer, Mask, Selector, Shape};

/// Generates a street `Layer` that surrounds the area defined by `shape`.
pub fn street_layer_around(shape: Shape, texture_id: u32) -> Layer {
    let outer_shape = Shape {
        width: shape.width + 2,
        height: shape.height + 2,
    };

    let mut edge_coords = Vec::new();

    // Top and Bottom edges
    for x in 0..outer_shape.width {
        edge_coords.push(Coordinates { x, y: 0 }); // Top edge
        edge_coords.push(Coordinates {
            x,
            y: outer_shape.height - 1,
        }); // Bottom edge
    }

    // Left and Right edges (excluding corners to avoid duplication)
    for y in 1..(outer_shape.height - 1) {
        edge_coords.push(Coordinates { x: 0, y }); // Left edge
        edge_coords.push(Coordinates {
            x: outer_shape.width - 1,
            y,
        }); // Right edge
    }

    let mask = Mask::new(
        "street_border".to_string(),
        Selector::Sparse(edge_coords),
        Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
    );

    Layer::new("street".to_string(), vec![mask], 3)
}
