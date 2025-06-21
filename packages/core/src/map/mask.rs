use crate::{
    prelude::{Coordinates, Delta, Effect, Rect, Shape},
    traits::{Shaped, Shiftable},
};

impl Mask {
    /// Creates a new mask with a given name, rectangular areas, and uniform effect.
    pub fn new(name: String, tiles: Vec<Rect>, effects: Vec<Effect>) -> Self {
        Self {
            name,
            tiles,
            effects,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mask {
    /// The name of the mask for identification or debugging.
    pub name: String,
    /// Tiles that define the mask area and their effects.
    pub tiles: Vec<Rect>,
    pub effects: Vec<Effect>,
}

impl Shaped for Mask {
    /// Computes the bounding shape of all tiles.
    fn get_shape(&self) -> Shape {
        if self.tiles.is_empty() {
            return Shape::default();
        }

        let (max_x, max_y) = self.tiles.iter().fold((0, 0), |(mx, my), tile| {
            let x_end = tile.origin.x + tile.shape.width;
            let y_end = tile.origin.y + tile.shape.height;
            (mx.max(x_end), my.max(y_end))
        });

        Shape::from_rectangle(max_x, max_y)
    }
}

impl Shiftable for Mask {
    /// Offsets the mask and all its tiles by the specified delta.
    fn offset(&mut self, delta: Delta) {
        for tile in &mut self.tiles {
            tile.offset(delta);
            self.effects.iter_mut().for_each(|e| e.offset(delta));
        }
    }

    /// Returns a new mask with the offset applied.
    fn translate(&self, delta: Delta) -> Self {
        let mut new_mask = self.clone();
        new_mask.offset(delta);
        new_mask
    }
}

impl Mask {
    /// Checks if the mask contains the specified coordinate.
    pub fn contains(&self, coord: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.contains(coord))
    }

    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.effects.iter().any(|effect| match effect {
            Effect::Block(rect) => rect.contains(target),
            _ => false,
        })
    }

    pub fn get_actions(&self) -> Vec<u32> {
        self.effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Action(id) => Some(*id),
                _ => None,
            })
            .collect()
    }

    pub fn get_texture(&self) -> Option<u32> {
        self.effects.iter().find_map(|effect| match effect {
            Effect::Texture(id) => Some(*id),
            _ => None,
        })
    }

    pub fn get_render(&self) -> Option<u32> {
        self.effects.iter().find_map(|effect| match effect {
            Effect::Render(id) => Some(*id),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape};

    #[test]
    fn mask_new_creates_tiles() {
        let areas = vec![
            Rect::new(Coordinates::new(0, 0), Shape::new(1, 1)),
            Rect::new(Coordinates::new(2, 2), Shape::new(3, 3)),
        ];
        let effects = vec![
            Effect::Block(Rect::new(Coordinates::new(0, 0), Shape::new(1, 1))),
            Effect::None,
        ];

        let mask = Mask::new("test_mask".to_string(), areas.clone(), effects.clone());

        assert_eq!(mask.name, "test_mask");
        assert_eq!(mask.tiles.len(), areas.len());
        assert_eq!(mask.effects.len(), effects.len());

        for ((tile, area), effect) in mask.tiles.iter().zip(areas.iter()).zip(effects.iter()) {
            assert_eq!(*tile, *area);
            // We compare the effect at the same index
            assert_eq!(mask.effects.iter().find(|e| *e == effect), Some(effect));
        }
    }

    #[test]
    fn mask_offset_moves_tiles_and_effects() {
        let block_rect = Rect::new(Coordinates::new(1, 1), Shape::new(2, 2));
        let effect = Effect::Block(block_rect.clone());
        let area = Rect::new(Coordinates::new(0, 0), Shape::new(3, 3));
        let mut mask = Mask::new("offset_mask".to_string(), vec![area.clone()], vec![effect]);

        let delta = Delta::new(5, 7);
        mask.offset(delta);

        let tile = &mask.tiles[0];
        assert_eq!(tile.origin.x, 5);
        assert_eq!(tile.origin.y, 7);

        match &mask.effects[0] {
            Effect::Block(block) => {
                assert_eq!(block.origin.x, block_rect.origin.x + delta.dx as u32);
                assert_eq!(block.origin.y, block_rect.origin.y + delta.dy as u32);
                assert_eq!(block.shape, block_rect.shape);
            }
            _ => panic!("Expected a Block effect"),
        }
    }

    #[test]
    fn mask_get_shape_returns_correct_bounds() {
        let tiles = vec![
            Rect::new(Coordinates::new(1, 1), Shape::new(2, 2)),
            Rect::new(Coordinates::new(4, 3), Shape::new(3, 1)),
        ];
        let effects = vec![Effect::None, Effect::None];
        let mask = Mask::new("shape_mask".to_string(), tiles.clone(), effects);

        let shape = mask.get_shape();
        assert_eq!(shape.width, 7); // 4 + 3
        assert_eq!(shape.height, 4); // max(1+2, 3+1)
    }
}
