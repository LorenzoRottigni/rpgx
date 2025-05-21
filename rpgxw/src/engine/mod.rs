use std::cell::RefCell;

pub mod map;
pub mod pawn;

use js_sys::{Object, Promise, Reflect};
use rpgx::{common::direction::Direction, prelude::Coordinates};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::prelude::{WasmCoordinates,WasmMap,WasmPawn,WasmTile};

/// Represents the RPGX engine, which handles the game logic and interactions.
#[wasm_bindgen]
pub struct WasmEngine {
    inner: RefCell<rpgx::engine::Engine>,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(map: WasmMap, pawn: WasmPawn) -> Self {
        let engine = rpgx::engine::Engine::new(map.to_native(), pawn.to_native());
        Self {
            inner: RefCell::new(engine),
        }
    }

    #[wasm_bindgen]
    pub fn map(&self) -> WasmMap {
        WasmMap::from_native(self.inner.borrow().map.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn pawn(&self) -> WasmPawn {
        let pawn = &self.inner.borrow().pawn;
        WasmPawn::from_native(pawn.clone())
        
    }

    /// Walk to coordinates (x, y)
    #[wasm_bindgen]
    pub fn walk_to(&self, pointer: WasmCoordinates) -> Promise {
        let inner = self.inner.clone();
        let fut = async move {
            let mut engine = inner.borrow_mut();

            engine
                .walk_to(pointer.to_native())
                .await
                .map_err(|e| JsValue::from_str(&format!("walk_to failed: {:?}", e)))?;

            Ok(JsValue::undefined())
        };

        future_to_promise(fut)
    }

    /// Step in a direction (string): "Up", "Down", "Left", "Right"
    #[wasm_bindgen]
    pub fn step_to(&mut self, direction: String) -> Result<WasmTile, JsValue> {
        let dir = match direction.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return Err(JsValue::from_str("Invalid direction")),
        };
        let tile = self.inner
            .borrow_mut()
            .step_to(dir)
            .map_err(|e| JsValue::from_str(&format!("step_to failed: {:?}", e)))?;

        Ok(WasmTile::from_native(tile))
    }

    /// Move directly to coordinates (x, y)
    #[wasm_bindgen]
    pub fn move_to(&mut self, pointer: WasmCoordinates) -> Result<WasmTile, JsValue> {
        let tile = self.inner
            .borrow_mut()
            .move_to(pointer.to_native())
            .map_err(|e| JsValue::from_str(&format!("move_to failed: {:?}", e)))?;
        Ok(WasmTile::from_native(tile))
    }

    /// Get steps to coordinates (x, y)
    #[wasm_bindgen]
    pub fn steps_to(
        &self,
        pointer: WasmCoordinates
    ) -> Result<Vec<WasmCoordinates>, JsValue> {
        let steps = self.inner
            .borrow()
            .steps_to(pointer.to_native())
            .map_err(|e| JsValue::from_str(&format!("steps_to failed: {:?}", e)))?;
        Ok(steps.into_iter().map(WasmCoordinates::from_native).collect())
    }
}
