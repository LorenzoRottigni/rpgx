use crate::prelude::{WasmCoordinates, WasmDirection, WasmMap, WasmPawn, WasmTile};
use js_sys::{Array, Promise};
use rpgx::prelude::Engine;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

/// WASM wrapper for the Engine struct
#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(map: WasmMap, pawn: WasmPawn) -> WasmEngine {
        WasmEngine {
            inner: Engine::new(map.into_inner(), pawn.into_inner()),
        }
    }

    #[wasm_bindgen(js_name = walkTo)]
    pub fn walk_to(&mut self, target: &WasmCoordinates) -> Promise {
        let mut engine = self.inner.clone(); // clone so it can be moved into async block
        let target_coords = target.inner;
        future_to_promise(async move {
            match engine.walk_to(target_coords).await {
                Ok(tile) => Ok(WasmTile::from_inner(tile).into()),
                Err(e) => Err(JsValue::from_str(&format!("Move error: {:?}", e))),
            }
        })
    }

    #[wasm_bindgen(js_name = stepTo)]
    pub fn step_to(&mut self, direction: WasmDirection) -> Result<WasmTile, JsValue> {
        self.inner
            .step_to(direction.into_inner())
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Move error: {:?}", e)))
    }

    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&mut self, target: &WasmCoordinates) -> Result<WasmTile, JsValue> {
        self.inner
            .move_to(target.inner)
            .map(WasmTile::from_inner)
            .map_err(|e| JsValue::from_str(&format!("Move error: {:?}", e)))
    }

    #[wasm_bindgen(js_name = stepsTo)]
    pub fn steps_to(&self, target: &WasmCoordinates) -> Result<Array, JsValue> {
        self.inner
            .steps_to(target.inner)
            .map(|steps| {
                let arr = Array::new();
                for coord in steps {
                    let wasm_coord = WasmCoordinates::from_inner(coord);
                    arr.push(&wasm_coord.into());
                }
                arr
            })
            .map_err(|e| JsValue::from_str(&format!("Path error: {:?}", e)))
    }

    #[wasm_bindgen(getter)]
    pub fn map(&self) -> WasmMap {
        WasmMap::from_inner(self.inner.map.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn pawn(&self) -> WasmPawn {
        WasmPawn::from_inner(self.inner.pawn.clone())
    }
}
