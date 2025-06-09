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
