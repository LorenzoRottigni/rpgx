use crate::prelude::{WasmBlockSelector, WasmCoordinates, WasmMask, WasmShape, WasmTile};
use wasm_bindgen::prelude::*;

use rpgx::prelude::{Layer, LayerType, Mask};

pub mod mask;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum WasmLayerType {
    Base,
    Action,
    Texture,
    Block,
}

impl From<WasmLayerType> for LayerType {
    fn from(w: WasmLayerType) -> Self {
        match w {
            WasmLayerType::Base => LayerType::Base,
            WasmLayerType::Action => LayerType::Action,
            WasmLayerType::Texture => LayerType::Texture,
            WasmLayerType::Block => LayerType::Block,
        }
    }
}

impl From<LayerType> for WasmLayerType {
    fn from(l: LayerType) -> Self {
        match l {
            LayerType::Base => WasmLayerType::Base,
            LayerType::Action => WasmLayerType::Action,
            LayerType::Texture => WasmLayerType::Texture,
            LayerType::Block => WasmLayerType::Block,
        }
    }
}

#[wasm_bindgen]
pub struct WasmLayer {
    inner: Layer,
}

#[wasm_bindgen]
impl WasmLayer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        kind: WasmLayerType,
        shape: &WasmShape,
        masks: Box<[WasmMask]>,
        z: u32,
    ) -> WasmLayer {
        let rust_masks: Vec<Mask> = masks.iter().map(|m| m.as_inner().clone()).collect();
        let inner = Layer::new(name, kind.into(), shape.inner.clone(), rust_masks, z);
        WasmLayer { inner }
    }

    #[wasm_bindgen(js_name = base)]
    pub fn base(layers: Box<[WasmLayer]>) -> WasmLayer {
        let rust_layers: Vec<Layer> = layers.iter().map(|l| l.inner.clone()).collect();
        WasmLayer {
            inner: Layer::base(rust_layers),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> WasmLayerType {
        self.inner.kind.into()
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> WasmShape {
        WasmShape {
            inner: self.inner.shape.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn tiles(&self) -> js_sys::Array {
        let arr = js_sys::Array::new_with_length(self.inner.tiles.len() as u32);
        for (i, tile) in self.inner.tiles.iter().enumerate() {
            arr.set(i as u32, WasmTile::from_inner(tile.clone()).into());
        }
        arr
    }

    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> js_sys::Array {
        let arr = js_sys::Array::new_with_length(self.inner.masks.len() as u32);
        for (i, mask) in self.inner.masks.iter().enumerate() {
            arr.set(i as u32, WasmMask::from_inner(mask.clone()).into());
        }
        arr
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> u32 {
        self.inner.z
    }

    pub fn reshape(&mut self, shape: &WasmShape) {
        self.inner.reshape(shape.inner.clone());
    }

    pub fn positive_reshape(&mut self, shape: &WasmShape) {
        self.inner.positive_reshape(shape.inner.clone());
    }

    pub fn get_tile_at(&self, pointer: &WasmCoordinates) -> Option<WasmTile> {
        self.inner
            .get_tile_at(pointer.inner)
            .map(WasmTile::from_inner)
    }

    // pub fn get_block_at(&self, pointer: &WasmBlockSelector) -> js_sys::Array {
    //     // Use as_inner() to get a reference, then deref to pass owned tuple
    //     let tiles = self.inner.get_block_at(pointer.clone().into_inner());
    //     let arr = js_sys::Array::new_with_length(tiles.len() as u32);
    //
    //     for (i, tile) in tiles.into_iter().enumerate() {
    //         arr.set(i as u32, WasmTile::from_inner(tile).into());
    //     }
    //
    //     arr
    // }

    pub fn is_blocking_at(&self, target: &WasmCoordinates) -> bool {
        self.inner.is_blocking_at(&target.inner)
    }

    pub fn offset(&mut self, delta: &WasmCoordinates) {
        self.inner.offset(delta.inner);
    }

    // Expose the inner Layer for internal use if needed
    pub(crate) fn inner(&self) -> &Layer {
        &self.inner
    }
}

impl WasmLayer {
    pub fn into_inner(self) -> Layer {
        self.inner
    }

    pub fn from_inner(inner: Layer) -> Self {
        WasmLayer { inner }
    }
}
