use crate::prelude::Coordinates;

/// A [`Pawn`] represents an entity or character on the map, positioned on a specific [`Tile`].
///
/// It holds a reference to the [`Tile`] it currently occupies, along with a [`Asset`] used
/// to render its visual representation in the UI.
#[derive(Clone)]
pub struct Pawn {
    pub pointer: Coordinates,
    pub texture_id: u32,
}
