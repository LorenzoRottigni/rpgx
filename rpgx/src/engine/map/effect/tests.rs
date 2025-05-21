use super::*;
use crate::prelude::Coordinates;

#[test]
fn default_effect_has_no_action_or_texture_and_is_not_blocking() {
    let effect = Effect::default();
    assert_eq!(effect.action_id, None);
    assert_eq!(effect.texture_id, None);
    assert!(!effect.block);
    assert!(!effect.group);
    assert_eq!(effect.shrink, None);
}

#[test]
fn effect_with_all_fields_set_correctly() {
    let shrink_bounds = Some((
        Coordinates { x: 0, y: 0 },
        Coordinates { x: 2, y: 2 },
    ));

    let effect = Effect {
        action_id: Some(42),
        texture_id: Some(7),
        block: true,
        group: true,
        shrink: shrink_bounds,
    };

    assert_eq!(effect.action_id, Some(42));
    assert_eq!(effect.texture_id, Some(7));
    assert!(effect.block);
    assert!(effect.group);
    assert_eq!(effect.shrink, shrink_bounds);
}

#[test]
fn effect_can_be_cloned_and_copied() {
    let original = Effect {
        action_id: Some(1),
        texture_id: Some(2),
        block: true,
        group: false,
        shrink: None,
    };

    let cloned = original.clone();
    let copied = original;

    assert_eq!(original, cloned);
    assert_eq!(original, copied);
}
