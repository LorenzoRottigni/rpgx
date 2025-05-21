use super::*;
use crate::prelude::{Coordinates, Effect, Shape};

fn make_tile(pointer: Coordinates, shape: Shape, effect: Effect) -> Tile {
    Tile {
        id: 0,
        pointer,
        shape,
        effect,
    }
}

#[test]
fn contains_point_inside_shape() {
    let tile = make_tile(Coordinates { x: 0, y: 0 }, Shape::from_square(3), Effect::default());
    assert!(tile.contains(Coordinates { x: 1, y: 1 }));
}

#[test]
fn does_not_contain_point_outside_shape() {
    let tile = make_tile(Coordinates { x: 0, y: 0 }, Shape::from_square(2), Effect::default());
    assert!(!tile.contains(Coordinates { x: 3, y: 3 }));
}

#[test]
fn is_blocking_when_effect_has_block_true_and_contains_target() {
    let effect = Effect {
        block: true,
        shrink: None,
        ..Default::default()
    };
    let tile = make_tile(Coordinates { x: 0, y: 0 }, Shape::from_square(2), effect);
    assert!(tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
}

#[test]
fn is_not_blocking_when_effect_has_block_false() {
    let effect = Effect {
        block: false,
        shrink: None,
        ..Default::default()
    };
    let tile = make_tile(Coordinates { x: 0, y: 0 }, Shape::from_square(2), effect);
    assert!(!tile.is_blocking_at(Coordinates { x: 1, y: 1 }));
}

#[test]
fn is_blocking_when_within_shrink_bounds() {
    let effect = Effect {
        block: true,
        shrink: Some((
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 2, y: 2 },
        )),
        ..Default::default()
    };
    let tile = make_tile(Coordinates { x: 0, y: 0 }, Shape::from_square(4), effect);
    assert!(tile.is_blocking_at(Coordinates { x: 2, y: 2 }));
    assert!(!tile.is_blocking_at(Coordinates { x: 0, y: 0 }));
}

#[test]
fn generate_default_grid_creates_correct_number_of_tiles() {
    let shape = Shape { width: 3, height: 2 };
    let effect = Effect::default();
    let grid = Tile::generate_default_grid(shape, effect);
    assert_eq!(grid.len(), 6);
    assert_eq!(grid[0].pointer, Coordinates { x: 0, y: 0 });
    assert_eq!(grid[5].pointer, Coordinates { x: 2, y: 1 });
}

#[test]
fn offset_modifies_pointer() {
    let mut tile = make_tile(Coordinates { x: 2, y: 3 }, Shape::from_square(1), Effect::default());
    tile.offset(Coordinates { x: 1, y: 2 });
    assert_eq!(tile.pointer, Coordinates { x: 3, y: 5 });
}
