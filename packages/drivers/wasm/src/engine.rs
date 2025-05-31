use rpgx::prelude::Engine;
use wasm_bindgen::prelude::*;

use crate::prelude::WasmScene;

#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(scene: WasmScene) -> WasmEngine {
        WasmEngine {
            inner: Engine::new(scene.into_inner()),
        }
    }

    #[wasm_bindgen(js_name = WasmGetActiveScene)]
    pub fn get_active_scene(&mut self) -> Option<WasmScene> {
        self.inner
            .get_active_scene()
            .cloned()
            .map(WasmScene::from_inner)
    }

    #[wasm_bindgen(js_name = WasmPushScene)]
    pub fn push_scene(&mut self, scene: WasmScene) {
        self.inner.push_scene(scene.into_inner());
    }

    #[wasm_bindgen(js_name = WasmPopScene)]
    pub fn pop_scene(&mut self) {
        self.inner.pop_scene();
    }

    #[wasm_bindgen(js_name = WasmRollbackTo)]
    pub fn rollback_to(&mut self, index: usize) {
        self.inner.rollback_to(index);
    }

    #[wasm_bindgen(js_name = WasmRewindTo)]
    pub fn rewind_to(&mut self, index: usize) -> Result<(), JsValue> {
        self.inner
            .rewind_to(index)
            .map_err(|e| JsValue::from_str(e))
    }

    #[wasm_bindgen(js_name = WasmGetSceneAt)]
    pub fn get_scene_at(&self, index: usize) -> Option<WasmScene> {
        self.inner
            .get_scene_at(index)
            .cloned()
            .map(WasmScene::from_inner)
    }
}
