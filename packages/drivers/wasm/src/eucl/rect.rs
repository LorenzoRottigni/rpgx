use crate::{
    prelude::{WasmCoordinates, WasmShape},
    traits::WasmWrapper,
};
use rpgx::prelude::Rect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Rect)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmRect {
    inner: Rect,
}

impl WasmWrapper<Rect> for WasmRect {
    fn from_inner(inner: Rect) -> Self {
        WasmRect { inner }
    }

    fn inner(&self) -> &Rect {
        &self.inner
    }

    fn into_inner(self) -> Rect {
        self.inner
    }
}

#[wasm_bindgen(js_class = Rect)]
impl WasmRect {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: &WasmCoordinates, shape: &WasmShape) -> WasmRect {
        WasmRect {
            inner: Rect::new(origin.inner().clone(), *shape.inner()),
        }
    }

    #[wasm_bindgen(js_name = fromShape)]
    pub fn from_shape(shape: &WasmShape) -> WasmRect {
        WasmRect {
            inner: Rect::from_shape(*shape.inner()),
        }
    }

    #[wasm_bindgen(js_name = fromOrigin)]
    pub fn from_origin(origin: &WasmCoordinates) -> WasmRect {
        WasmRect {
            inner: Rect::from_origin(origin.inner().clone()),
        }
    }

    #[wasm_bindgen(js_name = fromXYWH)]
    pub fn from_xywh(x: u32, y: u32, width: u32, height: u32) -> WasmRect {
        WasmRect {
            inner: Rect::from_xywh(x, y, width, height),
        }
    }

    #[wasm_bindgen(js_name = fromMany)]
    pub fn from_many(rects: Vec<WasmRect>) -> Result<WasmRect, JsValue> {
        let inner_rects: Vec<Rect> = rects.into_iter().map(|r| r.inner).collect();
        match Rect::from_many(inner_rects) {
            Ok(rect) => Ok(WasmRect { inner: rect }),
            _ => Err(JsValue::from_str("RectError::EmptyRectList")),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> WasmCoordinates {
        WasmCoordinates::from_inner(self.inner.origin)
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> WasmShape {
        WasmShape::from_inner(self.inner.shape)
    }

    #[wasm_bindgen(js_name = asMany)]
    pub fn into_many(&self) -> Vec<WasmRect> {
        self.inner
            .into_many()
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asBlock)]
    pub fn into_single(&self) -> Vec<WasmRect> {
        self.inner
            .into_single()
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asPerimeter)]
    pub fn into_perimeter(&self, offset: u32, size: u32) -> Vec<WasmRect> {
        self.inner
            .into_perimeter(offset, size)
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asBisector)]
    pub fn into_bisector(&self, offset: u32, size: u32) -> Vec<WasmRect> {
        self.inner
            .into_bisector(offset, size)
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asCenter)]
    pub fn into_center(&self, offset: u32, size: u32) -> Vec<WasmRect> {
        self.inner
            .into_center(offset, size)
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asRhombus)]
    pub fn into_rhombus(&self, dial: u32) -> Vec<WasmRect> {
        self.inner
            .into_rhombus(dial)
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asCircle)]
    pub fn into_circle(&self) -> Vec<WasmRect> {
        self.inner
            .into_circle()
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asOdds)]
    pub fn into_odds(&self) -> Vec<WasmRect> {
        self.inner
            .into_odds()
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }

    #[wasm_bindgen(js_name = asEvens)]
    pub fn into_evens(&self) -> Vec<WasmRect> {
        self.inner
            .into_evens()
            .into_iter()
            .map(|r| WasmRect { inner: r })
            .collect()
    }
}
