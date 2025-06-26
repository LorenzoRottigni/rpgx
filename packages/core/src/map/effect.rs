use crate::prelude::{Delta, Rect};

#[doc = include_str!("../../docs/effect.md")]
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
    // Allows a tile to have different texture effects stacked on top of each other.
    // By default only the texture at the top of the stack is rendered.
    // Opaque,
}

impl Effect {
    pub fn offset(&mut self, delta: Delta) {
        if let Effect::Block(rect) = self {
            rect.offset(delta);
        }
    }
}
