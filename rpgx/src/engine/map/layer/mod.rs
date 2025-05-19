use mask::Mask;

use crate::common::{coordinates::Coordinates, shape::Shape};

pub use crate::prelude::{BlockSelector, Effect, SingleSelector, Tile};

pub mod mask;

#[derive(Clone, Copy, PartialEq)]
pub enum LayerType {
    Default,
    Action,
    Texture,
    Block,
}

/// A visual or logical overlay on top of the base [`Grid`], used to apply effects
/// on specific [`Tile`]s based on spatial [`Mask`] and [`Selector`].
/// [`Layer`]s simulate stacking behavior along the Z-axis and allow grouped or
/// conditional [`Tile`] modifications without altering the original [`Grid`].
#[derive(Clone)]
pub struct Layer {
    pub name: &'static str,
    pub kind: LayerType,
    pub tiles: Vec<Tile>,
    pub shape: Shape,
}

impl Layer {
    pub fn new(name: &'static str, kind: LayerType, shape: Shape, masks: Vec<Mask>) -> Self {
        let tiles = if kind == LayerType::Default {
            Tile::generate_default_grid(shape, Effect::default())
        } else {
            masks.iter().flat_map(|mask| mask.apply(shape)).collect()
        };
        Self {
            name,
            kind,
            shape,
            tiles,
        }
    }

    /// Retrieve a [`Tile`] from within the [`Grid`]
    pub fn get_tile(&self, pointer: SingleSelector) -> Option<Tile> {
        self.tiles
            .iter()
            .find(|tile| tile.pointer == pointer)
            .copied()
    }

    /// Retrieve a block of [`Tile`]s from within the [`Grid`]
    pub fn get_block(&self, pointer: BlockSelector) -> Vec<Tile> {
        self.shape
            .coordinates_in_range(pointer.0, pointer.1)
            .into_iter()
            .filter_map(|coord| self.tiles.iter().find(|t| t.pointer == coord).copied())
            .collect()
    }

    pub fn is_tile_blocked(&self, target: &Coordinates) -> bool {
        self.tiles.iter().any(|tile| tile.is_blocking_at(*target))
    }

    pub fn offset_tiles(mut self, delta: Coordinates) -> Self {
        for tile in &mut self.tiles {
            tile.offset(delta);
        }
        self.shape.width += delta.x; // optionally adjust shape here
        self.shape.height += delta.y;
        self
    }
}
