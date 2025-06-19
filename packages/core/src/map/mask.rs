use std::vec;

use crate::{
    prelude::{Coordinates, Delta, Effect, Rect, Shape, Tile},
    traits::{Bounded, Grid, Renderable, Shift, Spatial},
};

#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,
    /// Tiles that define the mask area and their effects.
    pub rects: Vec<Rect>,
    pub effect: Effect,
    /// Offset applied at rendering time to all tiles in the mask.
    pub offset: Delta,
}

impl Mask {
    /// Creates a new mask with a given name, rectangular areas, and uniform effect.
    pub fn new(name: String, rects: Vec<Rect>, effect: Effect) -> Self {
        Self {
            name,
            rects,
            effect,
            offset: Delta::default(),
        }
    }
}

impl Renderable for Mask {
    fn render(&self) -> Vec<Tile> {
        self.rects
            .iter()
            .map(|rect| {
                Tile::new(
                    self.effect.translate(self.offset),
                    rect.translate(self.offset),
                )
            })
            .collect()
    }
}

impl Bounded for Mask {
    fn get_bounding_rect(&self) -> Rect {
        if self.rects.is_empty() {
            return Rect::default(); // or however your Rect default is defined
        }

        let min_x = self.rects.iter().map(|r| r.origin.x).min().unwrap();
        let min_y = self.rects.iter().map(|r| r.origin.y).min().unwrap();

        let max_x = self
            .rects
            .iter()
            .map(|r| r.origin.x + r.shape.width)
            .max()
            .unwrap();

        let max_y = self
            .rects
            .iter()
            .map(|r| r.origin.y + r.shape.height)
            .max()
            .unwrap();

        let width = max_x - min_x;
        let height = max_y - min_y;

        Rect::new(Coordinates::new(min_x, min_y), Shape::new(width, height))
    }
}

impl Spatial for Mask {
    fn contains(&self, target: &Coordinates) -> bool {
        self.rects.iter().any(|rect| rect.contains(target))
    }
}

impl Grid for Mask {
    fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.rects.iter().any(|rect| rect.contains(target))
            && self
                .effect
                .block
                .map_or(false, |block| block.contains(target))
    }

    fn get_actions_at(&self, target: &Coordinates) -> Vec<u32> {
        if let Some(action_id) = self.effect.action_id {
            if self.contains(target) {
                vec![action_id]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn get_effects_at(&self, target: &Coordinates) -> Vec<Effect> {
        if self.contains(target) {
            vec![self.effect.clone()]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape};

    #[test]
    fn mask_new_preserves_data() {
        let effect = Effect {
            block: Some(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
            ..Default::default()
        };
        let areas = vec![
            Rect::new(Coordinates::new(0, 0), Shape::new(1, 1)),
            Rect::new(Coordinates::new(2, 2), Shape::new(3, 3)),
        ];
        let mask = Mask::new("test_mask".to_string(), areas.clone(), effect.clone());

        assert_eq!(mask.name, "test_mask");
        assert_eq!(mask.rects.len(), areas.len());

        let rendered = mask.render();
        assert_eq!(rendered.len(), areas.len());
        for (tile, area) in rendered.iter().zip(areas.iter()) {
            assert_eq!(tile.area, area.translate(mask.offset));
            assert_eq!(tile.effect, effect.translate(mask.offset));
        }
    }

    #[test]
    fn mask_offset_moves_tiles_and_effects() {
        let effect = Effect {
            block: Some(Rect::new(Coordinates::new(1, 1), Shape::new(2, 2))),
            ..Default::default()
        };
        let area = Rect::new(Coordinates::new(0, 0), Shape::new(3, 3));
        let mut mask = Mask::new("offset_mask".to_string(), vec![area], effect.clone());

        let delta = Delta::new(5, 7);
        mask.offset = delta;

        let rendered = mask.render();
        let tile = &rendered[0];
        assert_eq!(tile.area.origin.x, 5);
        assert_eq!(tile.area.origin.y, 7);

        let block = tile.effect.block.expect("Effect block should be set");
        assert_eq!(block.origin.x, 6); // 1 + 5
        assert_eq!(block.origin.y, 8); // 1 + 7

        assert_eq!(block.shape.width, 2);
        assert_eq!(block.shape.height, 2);
    }

    #[test]
    fn mask_get_shape_is_bound_exclusive() {
        let mask = Mask::new(
            "shape_mask".to_string(),
            vec![Rect::new(Coordinates::new(0, 0), Shape::new(4, 6))],
            Effect::default(),
        );
        assert_eq!(mask.get_bounding_rect().shape.width, 4);
        assert_eq!(mask.get_bounding_rect().shape.height, 6);
    }
}
