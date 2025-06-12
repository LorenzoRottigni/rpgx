use crate::prelude::{Coordinates, Effect, Layer, Map, Mask, Selector, Shape};

pub fn building_2x3(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
            Effect {
                texture_id: Some(texture_id),
                block: true,
                group: true,
                shrink: Some((
                    Coordinates { x: 1, y: 1 },
                    Coordinates {
                        x: shape.width - 2,
                        y: shape.height - 2,
                    },
                )),
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

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
            Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
        )],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![
            // base_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
        Coordinates::default(),
    )
}

pub fn building_1x1(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
            Effect {
                texture_id: Some(texture_id),
                block: true,
                group: true,
                shrink: Some((
                    Coordinates { x: 0, y: 0 },
                    Coordinates {
                        x: shape.width,
                        y: shape.height,
                    },
                )),
                ..Default::default()
            },
        )],
        5,
    );

    let start_x = 0;
    let end_x = 1;
    let bottom_y = shape.height - 1;

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
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
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
            Effect {
                texture_id: Some(texture_id),
                block: true,
                group: true,
                shrink: Some((
                    Coordinates { x: 1, y: 1 },
                    Coordinates {
                        x: shape.width - 1,
                        y: shape.height - 1,
                    },
                )),
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

    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
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
