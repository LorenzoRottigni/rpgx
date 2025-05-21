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
