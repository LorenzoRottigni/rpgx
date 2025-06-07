use crate::prelude::WasmCoordinates;
use rpgx::prelude::Effect;
use wasm_bindgen::prelude::*;

/// A JS-friendly wrapper around the `Effect` struct.
#[wasm_bindgen]
pub struct WasmEffect {
    inner: Effect,
}

#[wasm_bindgen]
impl WasmEffect {
    /// Create a new `Effect`.
    #[wasm_bindgen(constructor)]
    pub fn new(
        texture_id: Option<u32>,
        action_id: Option<u32>,
        render_id: Option<u32>,
        block: bool,
        group: bool,
        shrink_start: Option<WasmCoordinates>,
        shrink_end: Option<WasmCoordinates>,
    ) -> WasmEffect {
        let shrink = match (shrink_start, shrink_end) {
            (Some(start), Some(end)) => Some((start.inner, end.inner)),
            _ => None,
        };

        WasmEffect {
            inner: Effect {
                action_id,
                texture_id,
                render_id,
                block,
                group,
                shrink,
            },
        }
    }

    /// Getters
    #[wasm_bindgen(getter, js_name = actionId)]
    pub fn action_id(&self) -> Option<u32> {
        self.inner.action_id
    }

    #[wasm_bindgen(getter, js_name = textureId)]
    pub fn texture_id(&self) -> Option<u32> {
        self.inner.texture_id
    }

    #[wasm_bindgen(getter)]
    pub fn block(&self) -> bool {
        self.inner.block
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> bool {
        self.inner.group
    }

    /// Returns the shrink selector if present as a 2-element array of `WasmCoordinates`
    #[wasm_bindgen(js_name = shrink)]
    pub fn shrink(&self) -> Option<Box<[WasmCoordinates]>> {
        self.inner.shrink.map(|(start, end)| {
            vec![
                WasmCoordinates { inner: start },
                WasmCoordinates { inner: end },
            ]
            .into_boxed_slice()
        })
    }

    /// Internal getter for the underlying Rust value
    pub(crate) fn into_inner(&self) -> Effect {
        self.inner
    }

    pub(crate) fn from_effect(effect: Effect) -> WasmEffect {
        WasmEffect { inner: effect }
    }
}
