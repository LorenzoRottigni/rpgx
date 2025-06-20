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
