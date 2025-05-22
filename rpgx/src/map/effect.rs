use crate::prelude::BlockSelector;

/// Visual and interactive properties applied to a [`super::tile::Tile`] or an UI element
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    /// attach an action to a [`super::tile::Tile`]
    pub action_id: Option<i32>,
    /// attach a texture to a [`super::tile::Tile`]
    pub texture_id: Option<i32>,
    /// make a [`super::tile::Tile`] entity blocking
    pub block: bool,
    /// determine if [`super::tile::Tile`] belongs to a group spanning several contingent [`super::tile::Tile`]s
    pub group: bool,
    pub shrink: Option<BlockSelector>,
    // TODO: opaque
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_default_doesnt_apply_any_effect() {
        let effect = Effect::default();
        assert_eq!(effect.action_id, None);
        assert_eq!(effect.texture_id, None);
        assert!(!effect.block);
        assert!(!effect.group);
        assert_eq!(effect.shrink, None);
    }
}
