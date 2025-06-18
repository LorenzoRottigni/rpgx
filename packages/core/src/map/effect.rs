use crate::prelude::{Delta, Rect};

#[doc = include_str!("../../docs/effect.md")]
/// Visual and interactive properties applied to a [`super::tile::Tile`] or UI element.
///
/// This struct defines optional metadata and behavior modifiers such as actions,
/// textures, rendering callbacks, and blocking areas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    /// Optional action ID attached to the tile.
    pub action_id: Option<u32>,

    /// Optional texture ID attached to the tile.
    pub texture_id: Option<u32>,

    /// Optional render callback ID attached to the tile.
    pub render_id: Option<u32>,

    /// Make the tile area, or a part of it, not walkable.
    ///
    /// When `Some`, defines a rectangular region within the tile area that blocks movement or interaction.
    pub block: Option<Rect>,
}

impl Effect {
    pub fn offset(&mut self, delta: Delta) {
        if let Some(ref mut block_rect) = self.block {
            block_rect.offset(delta);
        }
    }

    pub fn translate(&self, delta: Delta) -> Self {
        let mut effect = *self;
        effect.offset(delta);
        effect
    }
}
