use crate::engine::pawn::WasmPawn;
use crate::eucl::coordinates::WasmCoordinates;
use crate::eucl::direction::WasmDirection;
use crate::map::WasmMap;
use crate::traits::WasmWrapper;
use rpgx::prelude::Scene;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Scene)]
pub struct WasmScene {
    inner: Scene,
}

impl WasmWrapper<Scene> for WasmScene {
    fn into_inner(self) -> Scene {
        self.inner
    }

    fn from_inner(inner: Scene) -> WasmScene {
        WasmScene { inner }
    }

    fn inner(&self) -> &Scene {
        &self.inner
    }
}

#[wasm_bindgen(js_class = Scene)]
impl WasmScene {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, map: WasmMap, pawn: Option<WasmPawn>) -> WasmScene {
        let inner = Scene::new(name, map.into_inner(), pawn.map(|p| p.into_inner()));
        WasmScene { inner }
    }

    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(js_name = loadPawn)]
    pub fn load_pawn(&mut self, texture_id: u32) {
        self.inner.load_pawn(texture_id);
    }

    #[wasm_bindgen(js_name = loadPawnAt)]
    pub fn load_pawn_at(&mut self, pawn: WasmPawn) {
        self.inner.load_pawn_at(pawn.into_inner());
    }

    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&mut self, target: &WasmCoordinates) -> Result<WasmCoordinates, JsValue> {
        self.inner
            .move_to(target.clone().into_inner())
            .map(WasmCoordinates::from_inner)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = stepTo)]
    pub fn step_to(&mut self, direction: WasmDirection) -> Result<WasmCoordinates, JsValue> {
        self.inner
            .step_to(direction.into_inner())
            .map(WasmCoordinates::from_inner)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = stepsTo)]
    pub fn steps_to(&self, target: &WasmCoordinates) -> Result<js_sys::Array, JsValue> {
        self.inner
            .steps_to(target.clone().into_inner())
            .map(|steps| {
                steps
                    .into_iter()
                    .map(WasmCoordinates::from_inner)
                    .map(JsValue::from)
                    .collect()
            })
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = walkTo)]
    pub async fn walk_to(&mut self, target: WasmCoordinates) -> Result<WasmCoordinates, JsValue> {
        self.inner
            .walk_to(target.into_inner())
            .await
            .map(WasmCoordinates::from_inner)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = getMap)]
    pub fn get_map(&self) -> WasmMap {
        WasmMap::from_inner(self.inner.map.clone())
    }

    #[wasm_bindgen(js_name = getPawn)]
    pub fn get_pawn(&self) -> Option<WasmPawn> {
        self.inner.pawn.clone().map(WasmPawn::from_inner)
    }
}
