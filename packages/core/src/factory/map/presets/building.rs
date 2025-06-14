use crate::prelude::{Coordinates, Effect, Layer, Map, Mask, Rect, Shape};

pub fn building_2x3(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            // Selector::Block(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
            vec![Rect::new(Coordinates { x: 0, y: 0 }, shape)],
            Effect {
                texture_id: Some(texture_id),
                block: Some(Rect::new(Coordinates { x: 1, y: 1 }, shape - 2)),
                // group: true,
                ..Default::default()
            },
        )],
        5,
    );

    let (start_x, end_x) = if shape.width % 2 == 0 {
        let mid_left = shape.width / 2 - 1;
        let mid_right = shape.width / 2;
        (mid_left, mid_right + 1)
    } else {
        let mid = shape.width / 2;
        (mid, mid + 1)
    };
    let bottom_y = shape.height - 1;

    // Use Rect here instead of tuple
    let action_rect = Rect::new(
        Coordinates {
            x: start_x,
            y: bottom_y,
        },
        Shape {
            width: end_x - start_x,
            height: 1,
        },
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            // Selector::Block(action_rect),
            action_rect.as_many(),
            Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
        )],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![building_layer.clone(), action_layer.clone()],
        Coordinates::default(),
    )
}

pub fn building_1x1(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            // Selector::Block(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
            vec![Rect::new(Coordinates { x: 0, y: 0 }, shape)],
            Effect {
                texture_id: Some(texture_id),
                block: Some(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
                // group: true,
                ..Default::default()
            },
        )],
        5,
    );

    let bottom_y = shape.height - 1;

    let action_rect = Rect::new(
        Coordinates { x: 0, y: bottom_y },
        Shape {
            width: 1,
            height: 2,
        },
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            // Selector::Block(action_rect),
            action_rect.as_many(),
            Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
        )],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![building_layer, action_layer],
        Coordinates::default(),
    )
}

pub fn building_3x2(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            // Selector::Block(Rect::new(Coordinates { x: 0, y: 0 }, shape)),
            vec![Rect::new(Coordinates { x: 0, y: 0 }, shape)],
            Effect {
                texture_id: Some(texture_id),
                block: Some(Rect::new(
                    Coordinates { x: 1, y: 1 },
                    Shape {
                        width: shape.width.saturating_sub(1),
                        height: shape.height.saturating_sub(1),
                    },
                )),
                // group: true,
                ..Default::default()
            },
        )],
        5,
    );

    let (start_x, end_x) = if shape.width % 2 == 0 {
        let mid_left = shape.width / 2 - 1;
        let mid_right = shape.width / 2;
        (mid_left, mid_right + 1)
    } else {
        let mid = shape.width / 2;
        (mid, mid + 1)
    };
    let bottom_y = shape.height - 1;

    let action_rect = Rect::new(
        Coordinates {
            x: start_x,
            y: bottom_y,
        },
        Shape {
            width: end_x - start_x,
            height: 2,
        },
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            // Selector::Block(action_rect),
            action_rect.as_many(),
            Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
        )],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![building_layer, action_layer],
        Coordinates::default(),
    )
}
