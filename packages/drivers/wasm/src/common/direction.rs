use rpgx::prelude::Direction;
use wasm_bindgen::prelude::*;

use crate::prelude::WasmCoordinates;

#[wasm_bindgen(js_name = "WasmDirection")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasmDirection {
    Up,
    Down,
    Left,
    Right,
}

impl WasmDirection {
    pub fn from_inner(inner: Direction) -> WasmDirection {
        match inner {
            Direction::Up => WasmDirection::Up,
            Direction::Down => WasmDirection::Down,
            Direction::Left => WasmDirection::Left,
            Direction::Right => WasmDirection::Right,
        }
    }
    pub fn into_inner(self) -> Direction {
        match self {
            WasmDirection::Up => Direction::Up,
            WasmDirection::Down => Direction::Down,
            WasmDirection::Left => Direction::Left,
            WasmDirection::Right => Direction::Right,
        }
    }
}

#[wasm_bindgen(js_class = "WasmDirection2")]
impl WasmDirection {
    #[wasm_bindgen(js_name = toDelta)]
    pub fn to_delta(self) -> WasmCoordinates {
        WasmCoordinates {
            inner: self.clone().into_inner().to_delta(),
        }
    }
    #[wasm_bindgen(js_name = fromDelta)]
    pub fn from_delta(delta: &WasmCoordinates) -> Option<WasmDirection> {
        Direction::from_delta(&delta.inner).map(WasmDirection::from_inner)
    }
}
