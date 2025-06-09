use crate::prelude::{BlockSelector, Coordinates, Shape};

#[doc = include_str!("../../docs/tile.md")]
/// Visual and interactive properties applied to a [`super::tile::Tile`] or UI element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    /// Optional action ID attached to the tile.
    pub action_id: Option<u32>,
    /// Optional texture ID attached to the tile.
    pub texture_id: Option<u32>,
    /// Optional render callback ID attached to the tile.
    pub render_id: Option<u32>,
    /// Whether this tile blocks movement or interaction.
    pub block: bool,
    /// Whether the tile belongs to a group of contiguous tiles.
    pub group: bool,
    /// Optional bounding box that restricts the blocking region on the tile.
    pub shrink: Option<BlockSelector>,
}

impl Effect {
    /// Checks if the given point lies within the `shrink` bounding box, if any.
    ///
    /// If no `shrink` box is defined, always returns `true`.
    ///
    /// # Arguments
    ///
    /// * `point` - Coordinates to test for containment within the shrink box.
    ///
    /// # Returns
    ///
    /// `true` if the point is inside the shrink bounds or no shrink is defined; otherwise `false`.
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
    use crate::prelude::Coordinates;

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

    #[test]
    fn shrink_contains_returns_true_if_no_shrink_defined() {
        let effect = Effect::default();
        let point = Coordinates { x: 0, y: 0 };
        assert!(effect.shrink_contains(point));
    }

    #[test]
    fn shrink_contains_returns_true_if_point_inside_bounds() {
        let start = Coordinates { x: 1, y: 1 };
        let end = Coordinates { x: 3, y: 3 };
        let effect = Effect {
            shrink: Some((start, end)),
            ..Default::default()
        };
        let inside_point = Coordinates { x: 2, y: 2 };
        assert!(effect.shrink_contains(inside_point));
    }

    #[test]
    fn shrink_contains_returns_false_if_point_outside_bounds() {
        let start = Coordinates { x: 1, y: 1 };
        let end = Coordinates { x: 3, y: 3 };
        let effect = Effect {
            shrink: Some((start, end)),
            ..Default::default()
        };
        let outside_point = Coordinates { x: 0, y: 0 };
        assert!(!effect.shrink_contains(outside_point));
    }

    #[test]
    fn shrink_contains_returns_false_if_point_underflows_when_subtracting() {
        let start = Coordinates { x: 2, y: 2 };
        let end = Coordinates { x: 4, y: 4 };
        let effect = Effect {
            shrink: Some((start, end)),
            ..Default::default()
        };
        // point with smaller x than start.x should return false due to underflow
        let underflow_point = Coordinates { x: 1, y: 3 };
        assert!(!effect.shrink_contains(underflow_point));
    }
}
