use crate::prelude::{BlockSelector, Coordinates, Shape};

/// Visual and interactive properties applied to a [`super::tile::Tile`] or an UI element
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    /// attach an action to a [`super::tile::Tile`]
    pub action_id: Option<u32>,
    /// attach a texture to a [`super::tile::Tile`]
    pub texture_id: Option<u32>,
    /// Attach a rendering callback to a [`super::tile::Tile`]
    pub render_id: Option<u32>,
    /// make a [`super::tile::Tile`] entity blocking
    pub block: bool,
    /// determine if [`super::tile::Tile`] belongs to a group spanning several contingent [`super::tile::Tile`]s
    pub group: bool,
    pub shrink: Option<BlockSelector>,
}

impl Effect {
    pub fn shrink_contains(&self, point: Coordinates) -> bool {
        if let Some((start, end)) = self.shrink {
            if let (Some(rel_x), Some(rel_y)) =
                (point.x.checked_sub(start.x), point.y.checked_sub(start.y))
            {
                let relative_point = Coordinates { x: rel_x, y: rel_y };
                let shrink_shape = Shape::from_bounds(start, end);
                shrink_shape.in_bounds(relative_point)
            } else {
                false
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_default_doesnt_apply_any_effect() {
        let effect = Effect::default();
        assert_eq!(effect.action_id, None);
        assert_eq!(effect.texture_id, None);
        assert_eq!(effect.render_id, None);
        assert!(!effect.block);
        assert!(!effect.group);
        assert_eq!(effect.shrink, None);
    }
}
