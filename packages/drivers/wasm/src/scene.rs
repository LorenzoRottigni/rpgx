use rpgx::prelude::Scene;
use wasm_bindgen::prelude::*;

use crate::prelude::{WasmCoordinates, WasmDirection, WasmMap, WasmPawn, WasmTile};

#[wasm_bindgen]
pub struct WasmScene {
    inner: Scene,
}

impl WasmScene {
    pub fn from_inner(inner: Scene) -> Self {
        WasmScene { inner }
    }

    pub fn into_inner(self) -> Scene {
        self.inner
    }

    pub fn inner(&self) -> &Scene {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Scene {
        &mut self.inner
    }
}

#[wasm_bindgen]
impl WasmScene {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, map: WasmMap, pawn: WasmPawn) -> WasmScene {
        WasmScene {
            inner: Scene::new(name, map.into_inner(), pawn.into_inner()),
        }
    }

    #[wasm_bindgen(getter, js_name = WasmName)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter, js_name = WasmMap)]
    pub fn map(&self) -> WasmMap {
        WasmMap::from_inner(self.inner.map.clone())
    }

    #[wasm_bindgen(getter, js_name = WasmPawn)]
    pub fn pawn(&self) -> WasmPawn {
        WasmPawn::from_inner(self.inner.pawn.clone())
    }

    #[wasm_bindgen(js_name = WasmMoveTo)]
    pub fn move_to(&mut self, target: &WasmCoordinates) -> Result<WasmTile, JsValue> {
        self.inner
            .move_to(target.into_inner())
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Move failed: {e:?}")))
    }

    #[wasm_bindgen(js_name = WasmStepTo)]
    pub fn step_to(&mut self, direction: WasmDirection) -> Result<WasmTile, JsValue> {
        self.inner
            .step_to(direction.into_inner())
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Step failed: {e:?}")))
    }

    #[wasm_bindgen(js_name = WasmStepsTo)]
    pub fn steps_to(&self, target: &WasmCoordinates) -> Result<js_sys::Array, JsValue> {
        self.inner
            .steps_to(target.clone().into_inner())
            .map(|path| {
                path.into_iter()
                    .map(|c| JsValue::from(WasmCoordinates::from_inner(c)))
                    .collect::<js_sys::Array>()
            })
            .map_err(|e| JsValue::from_str(&format!("Pathfinding failed: {e:?}")))
    }

    #[wasm_bindgen(js_name = WasmWalkTo)]
    pub async fn walk_to(&mut self, target: &WasmCoordinates) -> Result<WasmTile, JsValue> {
        self.inner
            .walk_to(target.into_inner())
            .await
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Walk failed: {e:?}")))
    }
}
