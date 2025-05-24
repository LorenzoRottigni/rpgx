use crate::prelude::{Coordinates, Effect, Layer, LayerType, Map, Mask, Selector, Shape};

pub fn building_2x3(shape: Shape, texture_id: i32, action_id: i32) -> Map {
    let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        shape,
        vec![Mask {
            name: "logo".to_string(),
            effect: Effect {
                texture_id: Some(texture_id),
                action_id: None,
                block: true,
                group: true,
                shrink: Some((
                    Coordinates { x: 1, y: 1 },
                    Coordinates {
                        x: shape.width - 2,
                        y: shape.height - 2,
                    },
                )),
            },
            selector: Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
        }],
    );

    let action_layer = Layer::new(
        "actions".to_string(),
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: Effect {
                texture_id: None,
                action_id: Some(action_id),
                block: false,
                group: false,
                shrink: None,
            },
            selector: Selector::Block((Coordinates { x: 2, y: 11 }, Coordinates { x: 3, y: 11 })),
        }],
    );

    Map::new(
        "default".to_string(),
        vec![
            base_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    )
}
