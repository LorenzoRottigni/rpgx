use rpgx::prelude::{Coordinates, Selector};
use wasm_bindgen::prelude::*;
/// Wrapper for Coordinates used as a SingleSelector
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmSingleSelector {
    inner: Coordinates,
}

#[wasm_bindgen]
impl WasmSingleSelector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> WasmSingleSelector {
        WasmSingleSelector {
            inner: Coordinates { x, y },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.inner.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.inner.y
    }
}

impl WasmSingleSelector {
    /// Create WasmSingleSelector from Coordinates (Rust side)
    pub fn from_inner(inner: Coordinates) -> Self {
        WasmSingleSelector { inner }
    }
}

/// Wrapper for BlockSelector (tuple of two Coordinates)
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmBlockSelector {
    start: Coordinates,
    end: Coordinates,
}

#[wasm_bindgen]
impl WasmBlockSelector {
    #[wasm_bindgen(constructor)]
    pub fn new(start: &WasmSingleSelector, end: &WasmSingleSelector) -> WasmBlockSelector {
        WasmBlockSelector {
            start: start.inner,
            end: end.inner,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> WasmSingleSelector {
        WasmSingleSelector { inner: self.start }
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> WasmSingleSelector {
        WasmSingleSelector { inner: self.end }
    }
}

impl WasmBlockSelector {
    pub(crate) fn into_inner(self) -> (Coordinates, Coordinates) {
        (self.start, self.end)
    }

    /// Create WasmBlockSelector from tuple of Coordinates (Rust side)
    pub fn from_inner(inner: (Coordinates, Coordinates)) -> Self {
        WasmBlockSelector {
            start: inner.0,
            end: inner.1,
        }
    }

    pub fn as_inner(&self) -> (&Coordinates, &Coordinates) {
        (&self.start, &self.end)
    }
}

/// The main Selector wrapper exposed to JS
#[wasm_bindgen]
pub struct WasmSelector {
    inner: Selector,
}

#[wasm_bindgen]
impl WasmSelector {
    /// Create a single tile selector
    #[wasm_bindgen(js_name = single)]
    pub fn new_single(coord: WasmSingleSelector) -> WasmSelector {
        WasmSelector {
            inner: Selector::Single(coord.inner),
        }
    }

    /// Create a block selector (rectangle)
    #[wasm_bindgen(js_name = block)]
    pub fn new_block(block: WasmBlockSelector) -> WasmSelector {
        WasmSelector {
            inner: Selector::Block((block.start, block.end)),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn is_single(&self) -> bool {
        matches!(self.inner, Selector::Single(_))
    }

    #[wasm_bindgen(getter)]
    pub fn is_block(&self) -> bool {
        matches!(self.inner, Selector::Block(_))
    }

    /// Returns the SingleSelector if this is a Single selector, else null.
    #[wasm_bindgen(js_name = asSingle)]
    pub fn as_single(&self) -> Option<WasmSingleSelector> {
        if let Selector::Single(coord) = self.inner {
            Some(WasmSingleSelector::from_inner(coord))
        } else {
            None
        }
    }

    /// Returns the BlockSelector if this is a Block selector, else null.
    #[wasm_bindgen(js_name = asBlock)]
    pub fn as_block(&self) -> Option<WasmBlockSelector> {
        if let Selector::Block((start, end)) = self.inner {
            Some(WasmBlockSelector::from_inner((start, end)))
        } else {
            None
        }
    }
}

impl WasmSelector {
    pub(crate) fn into_inner(self) -> Selector {
        self.inner
    }

    pub fn from_inner(inner: Selector) -> Self {
        WasmSelector { inner }
    }
}
