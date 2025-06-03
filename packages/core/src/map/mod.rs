use crate::prelude::{Coordinates, Direction, Layer, LayerType, Tile};
use indexmap::IndexMap;
use layer::{Effect, Shape, SingleSelector};

pub mod effect;
pub mod layer;
pub mod routing;
pub mod selector;
pub mod tile;

/// Game map containing multiple layers with lifetimes.
#[derive(Clone)]
pub struct Map<'a> {
    pub name: String,
    pub layers: Vec<Layer<'a>>,
}

impl<'a> Map<'a> {
    pub fn new(name: String, mut layers: Vec<Layer<'a>>) -> Self {
        if !layers.iter().any(|layer| layer.kind == LayerType::Base) {
            layers.push(Layer::base(&layers));
        }
        Self { name, layers }
    }

    pub fn compose(
        name: String,
        maps: Vec<(Map<'a>, SingleSelector)>,
        layers: Vec<Layer<'a>>,
    ) -> Self {
        let mut map = Map::new(name, layers);
        for (other_map, top_left) in maps.iter() {
            map.merge_at(other_map, *top_left);
        }
        map
    }

    pub fn load_layer(&mut self, layer: Layer<'a>) {
        if let Some(base_layer) = self.get_base_layer() {
            let offset = Coordinates {
                x: layer.shape.width.saturating_sub(base_layer.shape.width) - 1,
                y: layer.shape.height.saturating_sub(base_layer.shape.height) - 1,
            };
            for existing_layer in &mut self.layers {
                existing_layer.offset(offset);
            }
            self.layers.push(layer);
            self.layers.retain(|l| l.kind != LayerType::Base);
            let base_layer = Layer::base(&self.layers);
            self.layers.push(base_layer);
        } else {
            self.layers.push(layer);
            let base_layer = Layer::base(&self.layers);
            self.layers.push(base_layer);
        }
    }

    pub fn layers_by_name(&self) -> IndexMap<String, Layer<'a>> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.clone()))
            .collect()
    }

    pub fn merge_at(&mut self, other: &Map<'a>, top_left: Coordinates) {
        let mut layers_by_name = self.layers_by_name();

        for layer in &other.layers {
            let mut offset_layer = layer.clone();
            offset_layer.offset(top_left);

            layers_by_name
                .entry(layer.name.clone())
                .and_modify(|existing| {
                    existing.tiles.extend(&offset_layer.tiles);
                    existing.shape.expand_to_include(top_left, layer.shape);
                })
                .or_insert(offset_layer);
        }

        self.layers = layers_by_name.into_values().collect();
    }

    pub fn duplicate_to_the(&mut self, direction: Direction) {
        let shape = self.get_shape();
        let top_left = match direction {
            Direction::Up | Direction::Down => Coordinates {
                x: 0,
                y: shape.height,
            },
            Direction::Left | Direction::Right => Coordinates {
                x: shape.width,
                y: 0,
            },
        };
        self.merge_at(&self.clone(), top_left);
    }

    pub fn is_blocking_at(&self, target: Coordinates) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.is_blocking_at(&target))
    }

    pub fn get_shape(&self) -> Shape {
        self.get_base_layer().map(|l| l.shape).unwrap_or_default()
    }

    pub fn get_base_layer(&'a self) -> Option<&'a Layer<'a>> {
        self.layers.iter().find(|l| l.kind == LayerType::Base)
    }

    pub fn get_layers_of_type(&self, kind: LayerType) -> Vec<Layer<'a>> {
        self.layers
            .iter()
            .filter(|l| l.kind == kind)
            .cloned()
            .collect()
    }

    pub fn get_base_tile(&self, pointer: Coordinates) -> Option<&Tile> {
        self.get_base_layer()
            .and_then(|layer| layer.get_tile_at(pointer))
    }

    pub fn get_tiles_at(&self, pointer: Coordinates) -> Vec<&Tile> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer))
            .collect()
    }

    pub fn get_effects_at(&self, pointer: Coordinates) -> Vec<Effect> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer).map(|tile| tile.effect))
            .collect()
    }

    pub fn get_actions_at(&self, pointer: Coordinates) -> Vec<i32> {
        self.get_layers_of_type(LayerType::Action)
            .into_iter()
            .flat_map(|layer| {
                layer
                    .get_tile_at(pointer)
                    .and_then(|tile| tile.effect.action_id)
            })
            .collect()
    }
}
/*
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::{Effect, Shape};

    fn dummy_tile(x: i32, y: i32) -> Tile {
        Tile {
            id: 1,
            pointer: Coordinates { x, y },
            shape: Shape::from_square(1),
            effect: Effect::default(),
        }
    }

    fn dummy_layer(name: &str, kind: LayerType, tiles: Vec<Tile>, shape: Shape) -> Layer {
        Layer {
            name: name.to_string(),
            kind,
            tiles,
            shape,
            masks: vec![],
            z: 1,
        }
    }

    #[test]
    fn creates_map_with_layers() {
        let tile = dummy_tile(0, 0);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(1));
        let map = Map::new("TestMap".to_string(), vec![layer.clone()]);

        assert_eq!(map.name, "TestMap");
        assert_eq!(map.layers.len(), 1);
        assert_eq!(map.get_base_layer().unwrap().name, "base");
    }

    #[test]
    fn gets_tile_from_base_layer() {
        let tile = dummy_tile(1, 2);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(3));
        let map = Map::new("TileMap".to_string(), vec![layer]);

        let result = map.get_base_tile(Coordinates { x: 1, y: 2 });
        assert!(result.is_some());
        assert_eq!(result.unwrap().pointer, Coordinates { x: 1, y: 2 });
    }

    #[test]
    fn detects_blocked_tile_across_layers() {
        let blocked_tile = Tile {
            id: 2,
            pointer: Coordinates { x: 0, y: 0 },
            shape: Shape::from_square(1),
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let blocking_layer = dummy_layer(
            "block",
            LayerType::Block,
            vec![blocked_tile],
            Shape::from_square(1),
        );
        let map = Map::new("BlockMap".to_string(), vec![blocking_layer]);

        assert!(map.is_blocking_at(Coordinates { x: 0, y: 0 }));
        assert!(!map.is_blocking_at(Coordinates { x: 1, y: 1 }));
    }

    #[test]
    fn expands_map_with_offset_layer() {
        let tile = dummy_tile(0, 0);
        let shape = Shape::from_square(1);
        let mut base_map = Map::new(
            "Base".to_string(),
            vec![dummy_layer("base", LayerType::Base, vec![tile], shape)],
        );

        let offset_tile = Tile {
            pointer: Coordinates { x: 0, y: 0 },
            id: 10,
            shape: Shape::from_square(1),
            effect: Effect {
                action_id: Some(42),
                ..Default::default()
            },
        };
        let offset_layer = dummy_layer("base", LayerType::Base, vec![offset_tile], shape);

        let overlay_map = Map::new("Overlay".to_string(), vec![offset_layer]);

        base_map.merge_at(&overlay_map, Coordinates { x: 2, y: 3 });

        let tile = base_map.get_base_tile(Coordinates { x: 2, y: 3 });
        assert!(tile.is_some());
        assert_eq!(tile.unwrap().effect.action_id, Some(42));
    }

    #[test]
    fn gets_actions_at_position() {
        let tile = Tile {
            pointer: Coordinates { x: 1, y: 1 },
            id: 5,
            shape: Shape::from_square(1),
            effect: Effect {
                action_id: Some(99),
                ..Default::default()
            },
        };
        let action_layer = dummy_layer(
            "action",
            LayerType::Action,
            vec![tile],
            Shape::from_square(2),
        );
        let map = Map::new("ActionMap".to_string(), vec![action_layer]);

        let actions = map.get_actions_at(Coordinates { x: 1, y: 1 });
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], 99);
    }
}
 */
