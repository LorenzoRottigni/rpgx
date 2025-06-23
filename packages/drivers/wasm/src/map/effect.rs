use crate::prelude::{WasmDelta, WasmRect};
use crate::traits::WasmWrapper;
use rpgx::prelude::Effect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Effect)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WasmEffect {
    inner: Effect,
}

impl WasmWrapper<Effect> for WasmEffect {
    fn inner(&self) -> &Effect {
        &self.inner
    }

    fn into_inner(self) -> Effect {
        self.inner
    }

    fn from_inner(inner: Effect) -> Self {
        WasmEffect { inner }
    }
}

#[wasm_bindgen(js_class = Effect)]
impl WasmEffect {
    // === Constructors ===

    #[wasm_bindgen(js_name = none)]
    pub fn none() -> WasmEffect {
        WasmEffect {
            inner: Effect::None,
        }
    }

    #[wasm_bindgen(js_name = action)]
    pub fn action(id: u32) -> WasmEffect {
        WasmEffect {
            inner: Effect::Action(id),
        }
    }

    #[wasm_bindgen(js_name = texture)]
    pub fn texture(id: u32) -> WasmEffect {
        WasmEffect {
            inner: Effect::Texture(id),
        }
    }

    #[wasm_bindgen(js_name = render)]
    pub fn render(id: u32) -> WasmEffect {
        WasmEffect {
            inner: Effect::Render(id),
        }
    }

    #[wasm_bindgen(js_name = block)]
    pub fn block(rect: &WasmRect) -> WasmEffect {
        WasmEffect {
            inner: Effect::Block(*rect.inner()),
        }
    }

    // === Accessors ===

    #[wasm_bindgen(js_name = kind)]
    pub fn kind(&self) -> String {
        match self.inner {
            Effect::None => "None",
            Effect::Action(_) => "Action",
            Effect::Texture(_) => "Texture",
            Effect::Render(_) => "Render",
            Effect::Block(_) => "Block",
        }
        .to_string()
    }

    #[wasm_bindgen(js_name = asAction)]
    pub fn as_action(&self) -> Option<u32> {
        if let Effect::Action(id) = self.inner {
            Some(id)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = asTexture)]
    pub fn as_texture(&self) -> Option<u32> {
        if let Effect::Texture(id) = self.inner {
            Some(id)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = asRender)]
    pub fn as_render(&self) -> Option<u32> {
        if let Effect::Render(id) = self.inner {
            Some(id)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = asBlock)]
    pub fn as_block(&self) -> Option<WasmRect> {
        if let Effect::Block(rect) = self.inner {
            Some(WasmRect::from_inner(rect))
        } else {
            None
        }
    }

    // === Methods ===

    #[wasm_bindgen]
    pub fn offset(&mut self, delta: &WasmDelta) {
        self.inner.offset(*delta.inner());
    }
}
