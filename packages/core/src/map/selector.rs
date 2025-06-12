use crate::prelude::{Coordinates, Shape};

/// Targets a single tile using its coordinates on the grid.
pub type SingleSelector = Coordinates;

/// Defines a rectangular area by specifying two opposite corner coordinates,
/// typically top-left and bottom-right, to select a block of tiles.
pub type BlockSelector = (Coordinates, Coordinates);

/// A function that takes a coordinate and a shape and returns whether
/// the coordinate matches a filter criterion.
// pub type FilterSelector = fn(Coordinates, Shape) -> bool;

pub type SparseSelector = Vec<Coordinates>;

#[doc = include_str!("../../docs/selector.md")]
/// Defines how to target tiles on a grid for effects or logic.
///
/// Can select a single tile, a rectangular block, or tiles filtered by custom logic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    /// Selects a single tile at the given coordinates.
    Single(SingleSelector),

    /// Selects a rectangular block of tiles between two coordinates.
    Block(BlockSelector),

    Sparse(SparseSelector),
    // Selects tiles based on a custom filtering function.
    // Filter(FilterSelector),
}

impl Selector {
    /// Returns the bounding `Shape` that contains all selected coordinates.
    /// For `Single`, the shape is 1x1.
    /// For `Block`, the shape is computed from bounds.
    /// For `Sparse`, the minimal bounding rectangle containing all coordinates is returned.
    pub fn get_shape(&self) -> Shape {
        match self {
            Selector::Single(_) => Shape::from_square(1),
            Selector::Block((start, end)) => Shape::from_bounds(*start, *end),
            Selector::Sparse(coords) => {
                if coords.is_empty() {
                    return Shape::default();
                }

                let (min_x, max_x) = coords
                    .iter()
                    .map(|c| c.x)
                    .fold((u32::MAX, 0), |(min, max), x| (min.min(x), max.max(x)));

                let (min_y, max_y) = coords
                    .iter()
                    .map(|c| c.y)
                    .fold((u32::MAX, 0), |(min, max), y| (min.min(y), max.max(y)));

                Shape::from_bounds(
                    Coordinates { x: min_x, y: min_y },
                    Coordinates { x: max_x, y: max_y },
                )
            }
        }
    }
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

    /* #[test]
    fn selector_filter_works() {
        fn only_even(c: Coordinates, _s: Shape) -> bool {
            c.x % 2 == 0 && c.y % 2 == 0
        }

        let selector = Selector::Filter(only_even);
        if let Selector::Filter(f) = selector {
            assert!(f(
                Coordinates { x: 2, y: 2 },
                Shape {
                    width: 6,
                    height: 6
                }
            ));
            assert!(!f(
                Coordinates { x: 1, y: 2 },
                Shape {
                    width: 6,
                    height: 6
                }
            ));
        } else {
            panic!("Expected Selector::Filter");
        }
    } */

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
        let copy = clone_selector(original.clone());

        assert_eq!(format!("{:?}", original), format!("{:?}", copy));
    }

    // Additional tests for PartialEq and Eq
    #[test]
    fn selector_equality() {
        let c1 = Coordinates { x: 5, y: 6 };
        let c2 = Coordinates { x: 5, y: 6 };
        let c3 = Coordinates { x: 7, y: 8 };

        assert_eq!(Selector::Single(c1), Selector::Single(c2));
        assert_ne!(Selector::Single(c1), Selector::Single(c3));

        let block1 = Selector::Block((c1, c3));
        let block2 = Selector::Block((c2, c3));
        let block3 = Selector::Block((c3, c1));

        assert_eq!(block1, block2);
        assert_ne!(block1, block3);
    }
}
