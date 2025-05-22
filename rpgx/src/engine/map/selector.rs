use crate::prelude::Coordinates;

/// A [`SingleSelector`] targets a single tile using its [`Coordinates`] on the grid.
pub type SingleSelector = Coordinates;

/// A [`BlockSelector`] defines a rectangular area by specifying two opposite corner [`Coordinates`],
/// typically top-left and bottom-right, to select a block of tiles.
pub type BlockSelector = (Coordinates, Coordinates);

/// A [`FilterSelector`] is a function that receives a reference to a [`Grid`] and returns a filtered list
/// of [`Tile`]s based on custom logic (e.g. pathfinding zones, terrain type).
pub type FilterSelector = fn(Coordinates) -> bool;

/// A [`Selector`] defines how to target a subset of [`Tile`]s on a [`Grid`] for applying effects or logic.
/// It supports selecting individual [`Tile`]s, rectangular blocks, or filtered custom selections.
#[derive(Clone, Debug, Copy)]
pub enum Selector {
    /// Selects a single [`Tile`] at the given [`Coordinates`].
    Single(SingleSelector),

    /// Selects a rectangular block of [`Tile`]s between two [`Coordinates`].
    Block(BlockSelector),

    /// Selects [`Tile`]s based on a custom filtering function.
    Filter(FilterSelector),
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn selector_single_works() {
        let coord = Coordinates { x: 3, y: 4 };
        let selector = Selector::Single(coord);

        if let Selector::Single(c) = selector {
            assert_eq!(c.x, 3);
            assert_eq!(c.y, 4);
        } else {
            panic!("Expected Selector::Single");
        }
    }

    #[test]
    fn selector_block_works() {
        let top_left = Coordinates { x: 0, y: 0 };
        let bottom_right = Coordinates { x: 2, y: 2 };
        let selector = Selector::Block((top_left, bottom_right));

        if let Selector::Block((a, b)) = selector {
            assert_eq!(a, top_left);
            assert_eq!(b, bottom_right);
        } else {
            panic!("Expected Selector::Block");
        }
    }

    #[test]
    fn selector_filter_works() {
        fn only_even(c: Coordinates) -> bool {
            c.x % 2 == 0 && c.y % 2 == 0
        }

        let selector = Selector::Filter(only_even);
        if let Selector::Filter(f) = selector {
            assert!(f(Coordinates { x: 2, y: 2 }));
            assert!(!f(Coordinates { x: 1, y: 2 }));
        } else {
            panic!("Expected Selector::Filter");
        }
    }

    #[test]
    fn selector_block_with_same_coords_is_valid() {
        let point = Coordinates { x: 1, y: 1 };
        let selector = Selector::Block((point, point));

        if let Selector::Block((a, b)) = selector {
            assert_eq!(a, point);
            assert_eq!(b, point);
        } else {
            panic!("Expected Selector::Block with same coordinates");
        }
    }

    #[test]
    fn selector_is_copy_and_clone() {
        fn clone_selector(sel: Selector) -> Selector {
            sel
        }

        let a = Coordinates { x: 1, y: 2 };
        let original = Selector::Single(a);
        let copy = clone_selector(original);

        assert_eq!(format!("{:?}", original), format!("{:?}", copy));
    }

}