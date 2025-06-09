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
    pub fn new(name: String, map: WasmMap, pawn: Option<WasmPawn>) -> WasmScene {
        let inner_pawn = if let Some(pawn) = pawn {
            Some(pawn.into_inner())
        } else {
            None
        };
        WasmScene {
            inner: Scene::new(name, map.into_inner(), inner_pawn),
        }
    }

    #[wasm_bindgen(getter, js_name = name)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter, js_name = map)]
    pub fn map(&self) -> WasmMap {
        WasmMap::from_inner(self.inner.map.clone())
    }

    #[wasm_bindgen(getter, js_name = pawn)]
    pub fn pawn(&self) -> Option<WasmPawn> {
        if let Some(pawn) = &self.inner.pawn {
            Some(WasmPawn::from_inner(pawn.clone()))
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&mut self, target: &WasmCoordinates) -> Result<WasmTile, JsValue> {
        self.inner
            .move_to(target.into_inner())
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Move failed: {e:?}")))
    }

    #[wasm_bindgen(js_name = stepTo)]
    pub fn step_to(&mut self, direction: WasmDirection) -> Result<WasmTile, JsValue> {
        self.inner
            .step_to(direction.into_inner())
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Step failed: {e:?}")))
    }

    #[wasm_bindgen(js_name = stepsTo)]
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

    #[wasm_bindgen(js_name = walkTo)]
    pub async fn walk_to(&mut self, target: &WasmCoordinates) -> Result<WasmTile, JsValue> {
        self.inner
            .walk_to(target.into_inner())
            .await
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Walk failed: {e:?}")))
    }
}
