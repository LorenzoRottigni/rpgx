pub mod pawn;
pub mod scene;

use crate::engine::scene::WasmScene;
use js_sys::Array;
use rpgx::prelude::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Engine)]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen(js_class = Engine)]
impl WasmEngine {
    /// Create a new engine from an initial scene
    #[wasm_bindgen(constructor)]
    pub fn new(scene: WasmScene) -> WasmEngine {
        WasmEngine {
            inner: Engine::new(scene.into_inner()),
        }
    }

    /// Get the active scene
    #[wasm_bindgen(js_name = getActiveScene)]
    pub fn get_active_scene(&self) -> Option<WasmScene> {
        self.inner
            .get_active_scene()
            .cloned()
            .map(WasmScene::from_inner)
    }

    /// Get the active scene mutably
    #[wasm_bindgen(js_name = getActiveSceneMut)]
    pub fn get_active_scene_mut(&mut self) -> Option<WasmScene> {
        self.inner
            .get_active_scene_mut()
            .cloned()
            .map(WasmScene::from_inner)
    }

    /// Push a new scene and set it active
    #[wasm_bindgen(js_name = pushScene)]
    pub fn push_scene(&mut self, scene: WasmScene) {
        self.inner.push_scene(scene.into_inner());
    }

    /// Pop the last scene if possible
    #[wasm_bindgen(js_name = popScene)]
    pub fn pop_scene(&mut self) {
        self.inner.pop_scene();
    }

    /// Rollback timeline to a given index
    #[wasm_bindgen(js_name = rollbackTo)]
    pub fn rollback_to(&mut self, index: usize) {
        self.inner.rollback_to(index);
    }

    /// Rewind to a specific index without truncating
    #[wasm_bindgen(js_name = rewindTo)]
    pub fn rewind_to(&mut self, index: usize) -> Result<(), JsValue> {
        self.inner
            .rewind_to(index)
            .map_err(|e| JsValue::from_str(e))
    }

    /// Get a scene at a specific index
    #[wasm_bindgen(js_name = getSceneAt)]
    pub fn get_scene_at(&self, index: usize) -> Option<WasmScene> {
        self.inner
            .get_scene_at(index)
            .cloned()
            .map(WasmScene::from_inner)
    }

    /// Get full timeline (cloned)
    #[wasm_bindgen(js_name = getTimeline)]
    pub fn get_timeline(&self) -> Array {
        self.inner
            .timeline
            .iter()
            .cloned()
            .map(WasmScene::from_inner)
            .map(JsValue::from)
            .collect()
    }

    /// Get current time index
    #[wasm_bindgen(js_name = getCurrentIndex)]
    pub fn get_current_index(&self) -> usize {
        self.inner.timenow
    }
}

impl WasmEngine {
    pub fn into_inner(self) -> Engine {
        self.inner
    }

    pub fn from_inner(inner: Engine) -> WasmEngine {
        WasmEngine { inner }
    }
}
