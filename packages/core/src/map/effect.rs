use crate::prelude::{Delta, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// Represents a tile with no special effects.
    None,

    /// Represents a tile with an action effect.
    Action(u32),

    /// Represents a tile with a texture effect.
    Texture(u32),

    /// Represents a tile with a render callback effect.
    Render(u32),

    /// Represents a tile with a blocking area effect.
    Block(Rect),
}

// #[doc = include_str!("../../docs/effect.md")]
// /// Visual and interactive properties applied to a [`super::tile::Tile`] or UI element.
// ///
// /// This struct defines optional metadata and behavior modifiers such as actions,
// /// textures, rendering callbacks, and blocking areas.
// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
// pub struct Effect {
//     /// Optional action ID attached to the tile.
//     pub action_id: Option<u32>,
//
//     /// Optional texture ID attached to the tile.
//     pub texture_id: Option<u32>,
//
//     /// Optional render callback ID attached to the tile.
//     pub render_id: Option<u32>,
//
//     /// Make the tile area, or a part of it, not walkable.
//     ///
//     /// When `Some`, defines a rectangular region within the tile area that blocks movement or interaction.
//     pub block: Option<Rect>,
// }

impl Effect {
    pub fn offset(&mut self, delta: Delta) {
        if let Effect::Block(rect) = self {
            rect.offset(delta);
        }
    }
}
