use crate::prelude::{Coordinates, Delta, Effect, Rect, Shape, Tile};

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
        // let tiles = areas
        //     .into_iter()
        //     .map(|area| Tile { area, effect })
        //     .collect();

        Self {
            name,
            rects,
            effect,
            offset: Delta::default(),
        }
    }

    pub fn render(&self) -> Vec<Tile> {
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

    pub fn is_blocking_at(&self, target: &Coordinates) -> bool {
        self.rects.iter().any(|rect| rect.contains(target))
            && self
                .effect
                .block
                .map_or(false, |block| block.contains(target))
    }

    /// Offsets all tiles in the mask and their effects by the specified delta.
    // pub fn offset(&mut self, delta: Delta) {
    //     for tile in &mut self.tiles {
    //         tile.area.offset(delta);
    //         tile.effect.offset(delta);
    //     }
    // }

    /// Computes the bounding shape of all tiles.
    pub fn get_shape(&self) -> Shape {
        if self.rects.is_empty() {
            return Shape::default();
        }

        let (max_x, max_y) = self.rects.iter().fold((0, 0), |(mx, my), rect| {
            let x_end = rect.origin.x + rect.shape.width;
            let y_end = rect.origin.y + rect.shape.height;
            (mx.max(x_end), my.max(y_end))
        });

        Shape::from_rectangle(max_x, max_y)
    }

    /// Returns true if any tile contains the given coordinate.
    pub fn contains(&self, coord: &Coordinates) -> bool {
        self.rects.iter().any(|rect| rect.contains(&coord))
    }

    // pub fn translate(&self, delta: Delta) -> Self {
    //     let mut mask = self.clone();
    //     mask.offset = mask.offset + delta;
    //     mask
    // }

    // /// Returns the tile at the specified coordinate, if any.
    // pub fn tile_at(&self, coord: Coordinates) -> Option<Tile> {
    //     if let Some(rect) = self.rects.iter().find(|rect| rect.contains(coord)) {
    //         let effect = self.effect.clone();
    //         Some(Tile {
    //             area: rect.clone(),
    //             effect,
    //         })
    //     } else {
    //         None
    //     }
    // }
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
    fn mask_get_shape_returns_correct_bounds() {
        let tiles = vec![
            Rect::new(Coordinates::new(1, 1), Shape::new(2, 2)),
            Rect::new(Coordinates::new(4, 3), Shape::new(3, 1)),
        ];
        let mask = Mask::new("shape_mask".to_string(), tiles.clone(), Effect::default());

        let shape = mask.get_shape();
        assert_eq!(shape.width, 7); // 4 + 3
        assert_eq!(shape.height, 4); // max(1+2, 3+1)
    }

    /*#[test]
    fn mask_contains_and_tile_at_work() {
        let tile = Rect::new(Coordinates::new(5, 5), Shape::new(2, 2));
        let mut mask = Mask::new(
            "lookup_mask".to_string(),
            vec![tile.clone()],
            Effect::default(),
        );

        mask.offset = Delta::new(1, 1);

        let check_inside = Coordinates::new(6, 6); // Originally (5,5) → offset by (1,1)
        let check_outside = Coordinates::new(8, 8); // Outside

        assert!(mask.contains(check_inside));
        assert!(!mask.contains(check_outside));

        let found = mask.tile_at(check_inside);
        assert!(found.is_some());
        assert_eq!(found.unwrap().area, tile); // tile_at ignores offset

        assert!(mask.tile_at(check_outside).is_none());
    } */
}
