use crate::{
    common::delta::Delta,
    prelude::{Coordinates, Direction, Layer, Tile},
};
use indexmap::IndexMap;
use layer::{Effect, Shape};

pub mod effect;
pub mod grid;
pub mod layer;
pub mod routing;
pub mod selector;
pub mod tile;

#[doc = include_str!("../../docs/map.md")]
/// Game map containing multiple layers.
#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub layers: Vec<Layer>,
    pub spawn: Coordinates,
}

impl Map {
    /// Creates a new map, adding a base layer if none exists.
    ///
    /// # Arguments
    /// * `name` - The name of the map.
    /// * `layers` - Vector of layers to include in the map.
    /// * `spawn` - The spawn coordinates within the map.
    ///
    /// # Behavior
    /// If no layer of kind `Base` exists, a base layer is generated from existing layers.
    pub fn new(name: String, layers: Vec<Layer>, spawn: Coordinates) -> Self {
        // if !layers.iter().any(|layer| layer.kind == LayerType::Base) {
        //     layers.push(Layer::base(layers.clone()));
        // }
        Self {
            name,
            layers,
            spawn,
        }
    }

    /// Composes a map from multiple maps and layers.
    ///
    /// # Arguments
    /// * `name` - The name of the resulting composed map.
    /// * `maps` - Vector of tuples containing maps and their placement offsets.
    /// * `layers` - Additional layers to include in the composed map.
    /// * `spawn` - The spawn coordinates for the composed map.
    ///
    /// # Returns
    /// A new `Map` instance that merges the given maps and layers.
    pub fn compose(
        name: String,
        maps: Vec<(Map, Coordinates)>,
        layers: Vec<Layer>,
        spawn: Coordinates,
    ) -> Self {
        let mut map = Map::new(name, layers, spawn);
        for (other_map, top_left) in maps.iter() {
            map.merge_at(other_map, *top_left, None);
        }
        map
    }
}

impl Map {
    /// Adds a layer to the map.
    ///
    /// If a base layer exists, the base and existing layers are offset and reshaped to accommodate the new layer.
    /// If no base layer exists, it creates one from all layers.
    ///
    /// # Arguments
    /// * `layer` - The new layer to add.
    pub fn load_layer(&mut self, layer: Layer /* , offset: Coordinates */) {
        let current_shape = self.get_shape();
        let target_shape = layer.get_shape();

        // -1 beacause coordinates starts form 0 and shape from 1
        let dx = (target_shape.width as i32) - (current_shape.width as i32) - 1;
        let dy = (target_shape.height as i32) - (current_shape.height as i32) - 1;

        let offset = Delta {
            dx: if dx > 0 { dx } else { 0 },
            dy: if dy > 0 { dy } else { 0 },
        };

        // Only offset existing layers if offset is positive
        if offset.dx > 0 || offset.dy > 0 {
            for existing_layer in &mut self.layers {
                existing_layer.offset(offset);
            }
        }

        // Now add the new layer
        self.layers.push(layer);
        /* if let Some(base_layer) = self.get_base_layer() {
            // Calculate offset based on size difference
            let offset = Coordinates {
                x: if layer.shape.width > base_layer.shape.width {
                    layer.shape.width - base_layer.shape.width - 1
                } else {
                    0
                },
                y: if layer.shape.height > base_layer.shape.height {
                    layer.shape.height - base_layer.shape.height - 1
                } else {
                    0
                },
            };

            // Offset existing layers to fit the new layer
            for existing_layer in &mut self.layers {
                existing_layer.offset(offset);
            }

            // Add the new layer
            self.layers.push(layer);

            // Remove old base layer
            self.layers.retain(|l| l.kind != LayerType::Base);

            // Recreate base layer from all non-base layers
            let base_layer = Layer::base(self.layers.clone());
            self.layers.push(base_layer);
        } else {
            // If no base layer, just add the layer and create base layer from all
            self.layers.push(layer);
            let base_layer = Layer::base(self.layers.clone());
            self.layers.push(base_layer);
        } */
    }

    /// Returns a map from layer names to their corresponding layers.
    pub fn layers_by_name(&self) -> IndexMap<String, Layer> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.clone()))
            .collect()
    }

    /// Merges another map into this map at a specified offset.
    ///
    /// Layers with the same name are merged by extending tiles and expanding shape.
    /// New layers are added directly.
    ///
    /// # Arguments
    /// * `other` - The other map to merge.
    /// * `top_left` - The offset coordinate where `other` map is placed relative to this map.
    /// * `spawn` - Optional new spawn coordinates to override this map's spawn.
    pub fn merge_at(&mut self, other: &Map, top_left: Coordinates, spawn: Option<Coordinates>) {
        // let mut layers_by_name = self.layers_by_name();

        for layer in &other.layers {
            let mut offset_layer = layer.clone();
            offset_layer.offset(top_left.to_delta());
            self.layers.push(offset_layer)

            // layers_by_name
            //     .entry(layer.name.clone())
            //     .and_modify(|existing| {
            //         existing.tiles.extend(&offset_layer.tiles);
            //         existing.shape.expand_to_include(top_left, layer.shape);
            //     })
            //     .or_insert(offset_layer);
        }

        // self.layers.push(); // layers_by_name.into_values().collect();

        // Optionally update spawn location
        self.spawn = spawn.unwrap_or(self.spawn);
    }

    /// Duplicates the map in a specified direction, expanding it by merging itself offset.
    ///
    /// # Arguments
    /// * `direction` - Direction to duplicate (`Up`, `Down`, `Left`, `Right`).
    /// * `spawn` - Optional spawn coordinate to override.
    pub fn duplicate_to_the(&mut self, direction: Direction, spawn: Option<Coordinates>) {
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
        self.merge_at(&self.clone(), top_left, spawn);
    }

    /// Checks if any layer blocks the tile at the given coordinates.
    ///
    /// # Arguments
    /// * `target` - Coordinates to check.
    ///
    /// # Returns
    /// `true` if any layer marks the tile as blocked.
    pub fn move_allowed(&self, target: Coordinates) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.get_tile_at(target).is_some())
            && self
                .layers
                .iter()
                .all(|layer| !layer.is_blocking_at(&target))
    }

    /// Returns the shape of the map, derived from the base layer.
    ///
    /// Returns a default shape if no base layer is found.
    pub fn get_shape(&self) -> Shape {
        let shapes: Vec<Shape> = self.layers.iter().map(|l| l.get_shape()).collect();
        Shape::bounding_shape(&shapes)
        // if let Some(base_layer) = self.get_base_layer() {
        //     base_layer.shape
        // } else {
        //     Shape::default()
        // }
    }

    /// Returns the first base layer, if present.
    // pub fn get_base_layer(&self) -> Option<Layer> {
    //     self.layers
    //         .iter()
    //         .find(|l| l.kind == LayerType::Base)
    //         .cloned()
    // }

    /// Returns all layers of a specific kind.
    ///
    /// # Arguments
    /// * `kind` - LayerType to filter.
    // pub fn get_layers_of_type(&self, kind: LayerType) -> Vec<Layer> {
    //     self.layers
    //         .iter()
    //         .filter(|l| l.kind == kind)
    //         .cloned()
    //         .collect()
    // }

    /// Gets the tile at the given coordinates from the base layer.
    ///
    /// # Arguments
    /// * `pointer` - Coordinates of the tile.
    ///
    /// # Returns
    /// An option with the tile if found.
    // pub fn get_base_tile(&self, pointer: Coordinates) -> Option<Tile> {
    //     self.get_base_layer()?.get_tile_at(pointer)
    // }

    /// Gets all tiles stacked at the given coordinates from all layers.
    ///
    /// # Arguments
    /// * `pointer` - Coordinates of the tiles.
    ///
    /// # Returns
    /// Vector of tiles found.
    pub fn get_tiles_at(&self, pointer: Coordinates) -> Vec<Tile> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer))
            .collect()
    }

    /// Gets all effects at the given coordinates from all layers.
    ///
    /// # Arguments
    /// * `pointer` - Coordinates to query.
    ///
    /// # Returns
    /// Vector of effects.
    pub fn get_effects_at(&self, pointer: Coordinates) -> Vec<Effect> {
        self.layers
            .iter()
            .flat_map(|layer| layer.get_tile_at(pointer).map(|tile| tile.effect))
            .collect()
    }

    /// Gets all action IDs present at the given coordinates in action layers.
    ///
    /// # Arguments
    /// * `pointer` - Coordinates to query.
    ///
    /// # Returns
    /// Vector of action IDs.
    pub fn get_actions_at(&self, pointer: Coordinates) -> Vec<u32> {
        self.layers
            .clone()
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
    use crate::{
        common::rect::Rect,
        prelude::{Effect, Shape},
    };

    /// Creates a dummy tile at the given coordinates.
    fn dummy_tile(x: u32, y: u32) -> Tile {
        Tile {
            area: Rect {
                origin: Coordinates { x, y },
                shape: Shape::from_square(1),
            },
            effect: Effect::default(),
        }
    }

    /// Creates a dummy layer with the specified name, kind, tiles, and shape.
    fn dummy_layer(name: &str) -> Layer {
        Layer {
            name: name.to_string(),
            masks: vec![],
            z: 1,
        }
    }

     #[test]
    fn creates_map_with_layers() {
        let tile = dummy_tile(0, 0);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(1));
        let map = Map::new(
            "TestMap".to_string(),
            vec![layer.clone()],
            Coordinates::default(),
        );

        assert_eq!(map.name, "TestMap");
        assert_eq!(map.layers.len(), 1);
        assert_eq!(map.get_base_layer().unwrap().name, "base");
    }

    #[test]
    fn gets_tile_from_base_layer() {
        let tile = dummy_tile(1, 2);
        let layer = dummy_layer("base", LayerType::Base, vec![tile], Shape::from_square(3));
        let map = Map::new("TileMap".to_string(), vec![layer], Coordinates::default());

        let result = map.get_base_tile(Coordinates { x: 1, y: 2 });
        assert!(result.is_some());
        assert_eq!(result.unwrap().pointer, Coordinates { x: 1, y: 2 });
    }

    #[test]
    fn detects_blocked_tile_across_layers() {
        let blocked_tile = Tile {
            area: Rect {
                origin: Coordinates { x: 0, y: 0 },
                shape: Shape::from_square(1),
            },
            effect: Effect {
                block: true,
                ..Default::default()
            },
        };
        let blocking_layer = dummy_layer("block");
        let map = Map::new(
            "BlockMap".to_string(),
            vec![blocking_layer],
            Coordinates::default(),
        );

        assert!(map.move_allowed(Coordinates { x: 0, y: 0 }));
        assert!(!map.move_allowed(Coordinates { x: 1, y: 1 }));
    }

    /* #[test]
    fn expands_map_with_offset_layer() {
        let tile = dummy_tile(0, 0);
        let shape = Shape::from_square(1);
        let mut base_map = Map::new(
            "Base".to_string(),
            vec![dummy_layer("base", LayerType::Base, vec![tile], shape)],
            Coordinates::default(),
        );

        let new_layer = dummy_layer(
            "action",
            LayerType::Action,
            vec![dummy_tile(2, 2)],
            Shape::from_square(3),
        );
        base_map.load_layer(new_layer);

        // Should have at least 2 layers: base + new action layer
        assert!(base_map.layers.len() >= 2);

        // Base layer shape should have been updated to accommodate new layer size
        let base_layer = base_map.get_base_layer().unwrap();
        assert!(base_layer.shape.width >= 3);
        assert!(base_layer.shape.height >= 3);
    }

    #[test]
    fn merges_maps_at_offset() {
        let layer1 = dummy_layer(
            "base",
            LayerType::Base,
            vec![dummy_tile(0, 0)],
            Shape::from_square(1),
        );
        let mut map1 = Map::new("Map1".to_string(), vec![layer1], Coordinates::default());

        let layer2 = dummy_layer(
            "base",
            LayerType::Base,
            vec![dummy_tile(1, 1)],
            Shape::from_square(2),
        );
        let map2 = Map::new("Map2".to_string(), vec![layer2], Coordinates::default());

        map1.merge_at(&map2, Coordinates { x: 2, y: 2 }, None);

        let base_layer = map1.get_base_layer().unwrap();
        assert!(base_layer.shape.width >= 3);
        assert!(base_layer.shape.height >= 3);

        let tiles = base_layer
            .tiles
            .iter()
            .map(|tile| tile.pointer)
            .collect::<Vec<_>>();
        assert!(tiles.contains(&Coordinates { x: 0, y: 0 }));
        assert!(tiles.contains(&Coordinates { x: 3, y: 3 })); // offset tile
    } */
}
*/
