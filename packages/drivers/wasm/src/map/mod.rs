use crate::prelude::{WasmCoordinates, WasmDirection, WasmLayer, WasmShape, WasmTile};
use effect::WasmEffect;
use rpgx::prelude::Map;
use wasm_bindgen::prelude::*;

pub mod effect;
pub mod layer;
pub mod selector;
pub mod tile;

/// WASM wrapper for the Map struct.
#[wasm_bindgen]
pub struct WasmMap {
    inner: Map,
}

#[wasm_bindgen]
impl WasmMap {
    /// Creates a new map with a name and optional array of WasmLayer
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, layers: Option<Box<[WasmLayer]>>) -> WasmMap {
        let layers_vec = layers
            .map(|layers_box| {
                layers_box
                    .into_vec()
                    .into_iter()
                    .map(|wl| wl.into_inner())
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        let map = Map::new(name, layers_vec);

        WasmMap { inner: map }
    }

    /// Compose a map from multiple maps and layers.
    /// maps: Array of tuples (WasmMap, WasmSingleSelector)
    /// layers: Array of WasmLayer

    // #[wasm_bindgen(js_name = compose)]
    // pub fn compose(
    //     name: String,
    //     map_tuples: Box<[WasmMapSelectorTuple]>,
    //     layers: Option<Box<[WasmLayer]>>,
    // ) -> WasmMap {
    //     let rust_maps: Vec<(Map, SingleSelector)> = map_tuples
    //         .into_vec()
    //         .into_iter()
    //         .map(|tuple| (tuple.map.inner.clone(), tuple.selector.inner.clone()))
    //         .collect();
    //
    //     let layers_vec = layers
    //         .map(|layers_box| {
    //             layers_box
    //                 .into_vec()
    //                 .into_iter()
    //                 .map(|wl| wl.into_inner())
    //                 .collect()
    //         })
    //         .unwrap_or_else(Vec::new);
    //
    //     let map = Map::compose(name, rust_maps, layers_vec);
    //
    //     WasmMap { inner: map }
    // }

    /// Returns the name of the map
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Returns the base layer if any
    #[wasm_bindgen(js_name = getBaseLayer)]
    pub fn get_base_layer(&self) -> Option<WasmLayer> {
        self.inner.get_base_layer().map(WasmLayer::from_inner)
    }

    /// Returns the shape of the map (shape of the base layer)
    #[wasm_bindgen(js_name = getShape)]
    pub fn get_shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.get_shape())
    }

    /// Returns true if any layer blocks the tile at pointer
    #[wasm_bindgen(js_name = isBlockingAt)]
    pub fn is_blocking_at(&self, pointer: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(pointer.inner)
    }

    /// Returns all layers as an array of WasmLayer
    #[wasm_bindgen(js_name = getLayers)]
    pub fn get_layers(&self) -> Box<[WasmLayer]> {
        self.inner
            .layers
            .iter()
            .cloned()
            .map(WasmLayer::from_inner)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Loads a layer (adds or reshapes base layer)
    #[wasm_bindgen(js_name = loadLayer)]
    pub fn load_layer(&mut self, layer: &WasmLayer) {
        self.inner.load_layer(layer.inner().clone());
    }

    /// Merges another map into this one at top_left coordinates
    #[wasm_bindgen(js_name = mergeAt)]
    pub fn merge_at(&mut self, other: &WasmMap, top_left: &WasmCoordinates) {
        self.inner.merge_at(&other.inner, top_left.inner);
    }

    /// Duplicates the map in the given direction, expanding it
    #[wasm_bindgen(js_name = duplicateToThe)]
    pub fn duplicate_to_the(&mut self, direction: WasmDirection) {
        self.inner.duplicate_to_the(direction.into_inner());
    }

    /// Returns tile at pointer from base layer if any
    #[wasm_bindgen(js_name = getBaseTile)]
    pub fn get_base_tile(&self, pointer: &WasmCoordinates) -> Option<WasmTile> {
        self.inner
            .get_base_tile(pointer.inner)
            .map(WasmTile::from_inner)
    }

    /// Returns stacked tiles at pointer from all layers
    #[wasm_bindgen(js_name = getTilesAt)]
    pub fn get_tiles_at(&self, pointer: &WasmCoordinates) -> Box<[WasmTile]> {
        self.inner
            .get_tiles_at(pointer.inner)
            .into_iter()
            .map(WasmTile::from_inner)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Returns all effects at pointer
    #[wasm_bindgen(js_name = getEffectsAt)]
    pub fn get_effects_at(&self, pointer: &WasmCoordinates) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for effect in self.inner.get_effects_at(pointer.inner) {
            let wasm_effect = WasmEffect::from_effect(effect);
            arr.push(&wasm_effect.into());
        }
        arr
    }

    /// Returns all action IDs present at pointer
    #[wasm_bindgen(js_name = getActionsAt)]
    pub fn get_actions_at(&self, pointer: &WasmCoordinates) -> Box<[u32]> {
        self.inner.get_actions_at(pointer.inner).into_boxed_slice()
    }
}

impl WasmMap {
    pub fn into_inner(self) -> Map {
        self.inner
    }

    pub fn from_inner(inner: Map) -> WasmMap {
        WasmMap { inner }
    }

    pub fn inner(&self) -> &Map {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Map {
        &mut self.inner
    }
}
