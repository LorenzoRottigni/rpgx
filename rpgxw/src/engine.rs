use std::cell::RefCell;

use js_sys::{Object, Promise, Reflect};
use rpgx::common::direction::Direction;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::coordinates::Coordinates;
use crate::map::Map;
use crate::pawn::Pawn;

#[wasm_bindgen]
pub struct WasmEngine {
    inner: RefCell<rpgx::engine::Engine>,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(map: Map, pawn: Pawn) -> WasmEngine {
        let engine = rpgx::engine::Engine::new(map.to_native(), pawn.to_native());
        WasmEngine {
            inner: RefCell::new(engine),
        }
    }

    pub fn get_map(&self) -> Map {
        Map::from_native(self.inner.borrow().map.clone())
    }

    /// Asynchronously walk to a target position (x, y)
    #[wasm_bindgen]
    pub fn walk_to(&self, x: i32, y: i32) -> Promise {
        let target = Coordinates::new(x, y);

        let inner = self.inner.clone(); // clone RefCell (cheap, it's a pointer)

        let fut = async move {
            // Borrow mutably inside the async block
            let mut engine = inner.borrow_mut();

            engine
                .walk_to(target.to_native())
                .await
                .map_err(|e| JsValue::from_str(&format!("walk_to failed: {:?}", e)))?;

            Ok(JsValue::undefined())
        };

        future_to_promise(fut)
    }

    /// Step in a direction (string): "Up", "Down", "Left", "Right"
    #[wasm_bindgen]
    pub fn step_to(&mut self, direction: String) -> Result<(), JsValue> {
        let dir = match direction.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return Err(JsValue::from_str("Invalid direction")),
        };
        self.inner
            .borrow_mut()
            .step_to(dir)
            .map_err(|e| JsValue::from_str(&format!("step_to failed: {:?}", e)))
    }

    /// Move directly to coordinates (x, y)
    #[wasm_bindgen]
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        let target = Coordinates::new(x, y);
        self.inner
            .borrow_mut()
            .move_to(target.to_native())
            .map_err(|e| JsValue::from_str(&format!("move_to failed: {:?}", e)))
    }

    /// Get pawn's current position as an object { x: i32, y: i32 }
    #[wasm_bindgen(getter)]
    pub fn pawn_position(&self) -> JsValue {
        let coords = &self.inner.borrow().pawn.tile.pointer;

        let obj = Object::new();

        // Assuming Coordinates has `x` and `y` fields (i32)
        Reflect::set(&obj, &JsValue::from_str("x"), &JsValue::from(coords.x)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("y"), &JsValue::from(coords.y)).unwrap();

        JsValue::from(obj)
    }
}
