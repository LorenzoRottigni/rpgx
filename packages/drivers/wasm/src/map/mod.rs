pub mod effect;
pub mod layer;
pub mod mask;
pub mod tile;

use effect::WasmEffect;
use layer::WasmLayer;
use tile::WasmTile;

use rpgx::prelude::Map;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmDirection, WasmShape};

#[wasm_bindgen(js_name = Map)]
pub struct WasmMap {
    inner: Map,
}

#[wasm_bindgen(js_class = Map)]
impl WasmMap {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, layers: Vec<WasmLayer>, spawn: &WasmCoordinates) -> WasmMap {
        let inner_layers = layers.into_iter().map(|l| l.into_inner()).collect();
        WasmMap {
            inner: Map::new(name, inner_layers, *spawn.inner()),
        }
    }

    /* TODO #[wasm_bindgen(js_name = compose)]
    pub fn compose(
        name: String,
        maps: Vec<JsValue>, // JS array of [WasmMap, WasmCoordinates]
        layers: Vec<WasmLayer>,
        spawn: WasmCoordinates,
    ) -> Result<WasmMap, JsValue> {
        let mut rust_maps: Vec<(Map, Coordinates)> = Vec::with_capacity(maps.len());

        for js_val in maps {
            // Convert JsValue into JS array (tuple expected)
            let arr = js_val
                .dyn_into::<Array>()
                .map_err(|_| JsValue::from_str("Each map entry must be a tuple (array)"))?;

            if arr.length() != 2 {
                return Err(JsValue::from_str(
                    "Each map entry must be a tuple of length 2",
                ));
            }

            // Extract first element: WasmMap
            let wasm_map = arr.get(0);

            // Extract second element: WasmCoordinates
            let wasm_coord = arr.get(1);

            rust_maps.push((wasm_map, wasm_coord));
        }

        let inner_layers = layers.into_iter().map(|l| l.into_inner()).collect();

        Ok(WasmMap {
            inner: Map::compose(name, rust_maps, inner_layers, *spawn.inner()),
        })
    }*/

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn spawn(&self) -> WasmCoordinates {
        WasmCoordinates::from_inner(self.inner.spawn)
    }

    #[wasm_bindgen(getter)]
    pub fn layers(&self) -> Vec<WasmLayer> {
        self.inner
            .layers
            .iter()
            .cloned()
            .map(WasmLayer::from_inner)
            .collect()
    }

    /// Load a new layer into the map.
    #[wasm_bindgen(js_name = loadLayer)]
    pub fn load_layer(&mut self, layer: WasmLayer) {
        self.inner.load_layer(layer.into_inner());
    }

    /// Returns a JS object mapping layer names to WasmLayer instances.
    #[wasm_bindgen(js_name = layersByName)]
    pub fn layers_by_name(&self) -> js_sys::Object {
        // Use the core Map method `layers_by_name` directly, which returns IndexMap<String, Layer>
        let core_map = self.inner.layers_by_name();

        let obj = js_sys::Object::new();
        for (name, layer) in core_map.into_iter() {
            let wasm_layer = WasmLayer::from_inner(layer);
            js_sys::Reflect::set(&obj, &JsValue::from_str(&name), &JsValue::from(wasm_layer))
                .unwrap_throw();
        }
        obj
    }

    /// Merge another map at a given offset; optionally update spawn.
    #[wasm_bindgen(js_name = mergeAt)]
    pub fn merge_at(
        &mut self,
        other: WasmMap,
        top_left: WasmCoordinates,
        spawn: Option<WasmCoordinates>,
    ) {
        let spawn_opt = spawn.map(|s| *s.inner());
        self.inner
            .merge_at(&other.inner, *top_left.inner(), spawn_opt);
    }

    /// Duplicate this map in a direction; optionally update spawn.
    #[wasm_bindgen(js_name = duplicateToThe)]
    pub fn duplicate_to_the(&mut self, direction: WasmDirection, spawn: Option<WasmCoordinates>) {
        let spawn_opt = spawn.map(|s| *s.inner());
        self.inner
            .duplicate_to_the(direction.into_inner(), spawn_opt);
    }

    /// Returns true if movement is allowed at the given coordinate.
    #[wasm_bindgen(js_name = moveAllowed)]
    pub fn move_allowed(&self, target: &WasmCoordinates) -> bool {
        self.inner.move_allowed(*target.inner())
    }

    /// Returns the bounding shape of the map.
    #[wasm_bindgen(js_name = getShape)]
    pub fn get_shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.get_shape())
    }

    /// Returns all tiles at a coordinate from all layers.
    #[wasm_bindgen(js_name = getTilesAt)]
    pub fn get_tiles_at(&self, pointer: &WasmCoordinates) -> Vec<WasmTile> {
        self.inner
            .get_tiles_at(*pointer.inner())
            .into_iter()
            .map(WasmTile::from_inner)
            .collect()
    }

    /// Returns all effects at a coordinate from all layers.
    #[wasm_bindgen(js_name = getEffectsAt)]
    pub fn get_effects_at(&self, pointer: &WasmCoordinates) -> Vec<WasmEffect> {
        self.inner
            .get_effects_at(*pointer.inner())
            .into_iter()
            .map(WasmEffect::from_inner)
            .collect()
    }

    /// Returns all action IDs at a coordinate from all layers.
    #[wasm_bindgen(js_name = getActionsAt)]
    pub fn get_actions_at(&self, pointer: &WasmCoordinates) -> Vec<u32> {
        self.inner.get_actions_at(*pointer.inner())
    }
}

impl WasmMap {
    /// Consume and get the inner Map
    pub fn into_inner(self) -> Map {
        self.inner
    }

    /// Create from an inner Map directly
    pub fn from_inner(inner: Map) -> WasmMap {
        WasmMap { inner }
    }
}
