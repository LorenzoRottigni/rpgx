use crate::prelude::*;

pub fn street_around(map: &mut Map, texture_id: u32) {
    let inner_shape = map.get_shape();
    let outer_width = inner_shape.width + 2;
    let outer_height = inner_shape.height + 2;

    let mut edge_coords = Vec::new();

    // Top and Bottom edges
    for x in 0..outer_width {
        edge_coords.push(Coordinates { x: x, y: 0 });
        edge_coords.push(Coordinates {
            x,
            y: (outer_height - 1),
        });
    }

    // Left and Right edges
    for y in 1..(outer_height - 1) {
        edge_coords.push(Coordinates { x: 0, y: y });
        edge_coords.push(Coordinates {
            x: (outer_width - 1),
            y,
        });
    }

    let mask = Mask::new(
        "street_border".to_string(),
        edge_coords
            .into_iter()
            .map(|c| Rect::new(c, Shape::from_square(1)))
            .collect(),
        Effect {
            texture_id: Some(texture_id),
            ..Default::default()
        },
    );

    let street_layer = Layer::new("street".to_string(), vec![mask], 3);

    map.load_layer(street_layer);
}

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
        // Selector::Sparse(edge_coords),
        edge_coords
            .iter()
            .map(|c| Rect::new(*c, Shape::from_square(1)))
            .collect(),
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
