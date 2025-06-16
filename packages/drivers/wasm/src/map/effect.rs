use rpgx::prelude::Effect;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmDelta, WasmRect};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WasmEffect {
    inner: Effect,
}

#[wasm_bindgen]
impl WasmEffect {
    #[wasm_bindgen(constructor)]
    pub fn new(
        action_id: Option<u32>,
        texture_id: Option<u32>,
        render_id: Option<u32>,
        block: Option<WasmRect>,
    ) -> WasmEffect {
        WasmEffect {
            inner: Effect {
                action_id,
                texture_id,
                render_id,
                block: block.map(|r| *r.inner()),
            },
        }
    }

    /// Getters and setters for action_id
    #[wasm_bindgen(getter, js_name = actionId)]
    pub fn action_id(&self) -> Option<u32> {
        self.inner.action_id
    }
    #[wasm_bindgen(setter, js_name = actionId)]
    pub fn set_action_id(&mut self, id: Option<u32>) {
        self.inner.action_id = id;
    }

    /// Getters and setters for texture_id
    #[wasm_bindgen(getter, js_name = textureId)]
    pub fn texture_id(&self) -> Option<u32> {
        self.inner.texture_id
    }
    #[wasm_bindgen(setter, js_name = textureId)]
    pub fn set_texture_id(&mut self, id: Option<u32>) {
        self.inner.texture_id = id;
    }

    /// Getters and setters for render_id
    #[wasm_bindgen(getter, js_name = renderId)]
    pub fn render_id(&self) -> Option<u32> {
        self.inner.render_id
    }
    #[wasm_bindgen(setter, js_name = renderId)]
    pub fn set_render_id(&mut self, id: Option<u32>) {
        self.inner.render_id = id;
    }

    /// Getters and setters for block rect
    #[wasm_bindgen(getter)]
    pub fn block(&self) -> Option<WasmRect> {
        self.inner.block.map(WasmRect::from_inner)
    }
    #[wasm_bindgen(setter)]
    pub fn set_block(&mut self, rect: Option<WasmRect>) {
        self.inner.block = rect.map(|r| *r.inner());
    }

    /// Offsets the block rect by delta (if any)
    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(*delta.inner());
    }
}

impl WasmEffect {
    // Internal Rust accessors
    pub fn from_inner(inner: Effect) -> Self {
        WasmEffect { inner }
    }

    pub fn inner(&self) -> &Effect {
        &self.inner
    }
}
