use crate::prelude::WasmDelta;
use rpgx::prelude::Direction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Direction)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmDirection {
    inner: Direction,
}

#[wasm_bindgen(js_class = Direction)]
impl WasmDirection {
    /// Constructs a WasmDirection from a string like "Up", "Down", "Left", "Right".
    /// Returns None if the string does not match any direction.
    #[wasm_bindgen(constructor)]
    pub fn new(direction_str: String) -> WasmDirection {
        let dir = match direction_str.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => panic!("Invalid direction string"),
        };
        WasmDirection { inner: dir }
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        match self.inner {
            Direction::Up => "Up".to_string(),
            Direction::Down => "Down".to_string(),
            Direction::Left => "Left".to_string(),
            Direction::Right => "Right".to_string(),
        }
    }

    /// Creates a WasmDirection from a WasmDelta if possible.
    #[wasm_bindgen(js_name = fromDelta)]
    pub fn from_delta(delta: &WasmDelta) -> Option<WasmDirection> {
        Direction::from_delta(&delta.inner()).map(|d| WasmDirection { inner: d })
    }

    /// Converts the WasmDirection into a WasmDelta.
    #[wasm_bindgen(js_name = toDelta)]
    pub fn to_delta(&self) -> WasmDelta {
        WasmDelta::from_inner(self.inner.to_delta())
    }

    /// Returns true if this direction is equal to another.
    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: &WasmDirection) -> bool {
        self.inner == other.inner
    }
}

// Internal Rust API
impl WasmDirection {
    pub fn from_inner(inner: Direction) -> Self {
        WasmDirection { inner }
    }

    pub fn inner(&self) -> Direction {
        self.inner
    }

    pub fn into_inner(self) -> Direction {
        self.inner
    }
}
