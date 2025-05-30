use crate::prelude::{Coordinates, Effect, Layer, LayerType, Map, Mask, Selector, Shape};

pub fn building_2x3(shape: Shape, texture_id: i32, action_id: i32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        shape,
        vec![Mask {
            name: "logo".to_string(),
            effect: Effect {
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
            selector: Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
        }],
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
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
        }],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![
            // base_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    )
}

pub fn building_1x1(shape: Shape, texture_id: i32, action_id: i32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        shape,
        vec![Mask {
            name: "logo".to_string(),
            effect: Effect {
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
            selector: Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
        }],
        5,
    );

    let start_x = 0;
    let end_x = 1;
    let bottom_y = shape.height - 1;

    let action_layer = Layer::new(
        "actions".to_string(),
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
        }],
        6,
    );

    Map::new("base".to_string(), vec![building_layer, action_layer])
}

pub fn building_3x2(shape: Shape, texture_id: i32, action_id: i32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        shape,
        vec![Mask {
            name: "logo".to_string(),
            effect: Effect {
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
            selector: Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
        }],
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
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x,
                    y: bottom_y + 1,
                },
            )),
        }],
        6,
    );

    Map::new("base".to_string(), vec![building_layer, action_layer])
}
