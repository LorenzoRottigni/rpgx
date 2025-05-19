use std::cell::RefCell;

use rpgx::common::direction::Direction;
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise, Reflect};
use wasm_bindgen_futures::future_to_promise;

// Coordinates
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Coordinates {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl Coordinates {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let x = Reflect::get(value, &JsValue::from_str("x"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.x must be a number"))? as i32;
        let y = Reflect::get(value, &JsValue::from_str("y"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Coordinates.y must be a number"))? as i32;
        Ok(Coordinates { x, y })
    }
}

impl Coordinates {
    fn to_native(&self) -> rpgx::common::coordinates::Coordinates {
        rpgx::common::coordinates::Coordinates { x: self.x, y: self.y }
    }
}


// Shape
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Shape {
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl Shape {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Shape {
        Shape { width, height }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

impl Shape {
    fn to_native(&self) -> rpgx::common::shape::Shape {
        rpgx::common::shape::Shape { width: self.width, height: self.height }
    }
}


// Effect
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Effect {
    texture: Option<String>,
    block: bool,
    group: bool,
}

impl Effect {
    fn to_native(&self) -> rpgx::prelude::Effect {
        rpgx::prelude::Effect {
            texture: self.texture.clone(),
            block: self.block,
            group: self.group,
            ..Default::default()
        }
    }
}


#[wasm_bindgen]
impl Effect {
    #[wasm_bindgen(constructor)]
    pub fn new(texture: Option<String>, block: bool, group: bool) -> Effect {
        Effect { texture, block, group }
    }

    #[wasm_bindgen(getter)]
    pub fn texture(&self) -> Option<String> {
        self.texture.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture(&mut self, texture: Option<String>) {
        self.texture = texture;
    }

    #[wasm_bindgen(getter)]
    pub fn block(&self) -> bool {
        self.block
    }

    #[wasm_bindgen(setter)]
    pub fn set_block(&mut self, block: bool) {
        self.block = block;
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> bool {
        self.group
    }

    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, group: bool) {
        self.group = group;
    }
}

impl Effect {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let texture = Reflect::get(value, &JsValue::from_str("texture"))?
            .as_string();
        let block = Reflect::get(value, &JsValue::from_str("block"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.block must be a bool"))?;
        let group = Reflect::get(value, &JsValue::from_str("group"))?
            .as_bool()
            .ok_or_else(|| JsValue::from_str("Effect.group must be a bool"))?;

        Ok(Effect { texture, block, group })
    }
}

// Selector
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Selector {
    start: Coordinates,
    end: Coordinates,
}

#[wasm_bindgen]
impl Selector {
    #[wasm_bindgen(constructor)]
    pub fn new_single(coord: Coordinates) -> Selector {
        Selector { start: coord, end: coord }
    }

    #[wasm_bindgen]
    pub fn new_block(start: Coordinates, end: Coordinates) -> Selector {
        Selector { start, end }
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Coordinates {
        self.start
    }

    #[wasm_bindgen(setter)]
    pub fn set_start(&mut self, start: Coordinates) {
        self.start = start;
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Coordinates {
        self.end
    }

    #[wasm_bindgen(setter)]
    pub fn set_end(&mut self, end: Coordinates) {
        self.end = end;
    }
}

impl Selector {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let start_js = Reflect::get(value, &JsValue::from_str("start"))?;
        let start = Coordinates::from_js_value(&start_js)?;
        let end_js = Reflect::get(value, &JsValue::from_str("end"))?;
        let end = Coordinates::from_js_value(&end_js)?;
        Ok(Selector { start, end })
    }
}

impl Selector {
    fn to_native(&self) -> rpgx::engine::map::selector::Selector {
        if self.start == self.end {
            rpgx::engine::map::selector::Selector::Single(self.start.to_native())
        } else {
            rpgx::engine::map::selector::Selector::Block((self.start.to_native(), self.end.to_native()))
        }
        // Filter variant can't be constructed from JS currently
    }
}

// Mask
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Mask {
    name: String,
    effect: Effect,
    selector: Selector,
}

#[wasm_bindgen]
impl Mask {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, effect: Effect, selector: Selector) -> Mask {
        Mask { name, effect, selector }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> Effect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn selector(&self) -> Selector {
        self.selector.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }
}

impl Mask {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Mask.name must be a string"))?;
        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = Effect::from_js_value(&effect_js)?;
        let selector_js = Reflect::get(value, &JsValue::from_str("selector"))?;
        let selector = Selector::from_js_value(&selector_js)?;

        Ok(Mask { name, effect, selector })
    }
}

impl Mask {
    fn to_native(&self) -> rpgx::prelude::Mask {
        rpgx::prelude::Mask {
            name: self.name.clone(),
            effect: self.effect.to_native(),
            selector: self.selector.to_native(),
        }
    }
}


// LayerType enum
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum LayerType {
    Default,
    Texture,
    Block,
    Action,
}

impl LayerType {
    fn to_native(&self) -> rpgx::prelude::LayerType {
        match self {
            LayerType::Default => rpgx::prelude::LayerType::Default,
            LayerType::Action => rpgx::prelude::LayerType::Action,
            LayerType::Texture => rpgx::prelude::LayerType::Texture,
            LayerType::Block => rpgx::prelude::LayerType::Block,
        }
    }
}

// Layer
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layer {
    name: String,
    kind: LayerType,
    shape: Shape,
    masks: Vec<Mask>,
}

#[wasm_bindgen]
impl Layer {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, kind: LayerType, shape: Shape, masks: &JsValue) -> Result<Layer, JsValue> {
        let masks_array = masks.dyn_ref::<Array>().ok_or_else(|| JsValue::from_str("Masks must be an Array"))?;
        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = Mask::from_js_value(&mask_js)?;
            masks_vec.push(mask);
        }
        Ok(Layer { name, kind, shape, masks: masks_vec })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> LayerType {
        self.kind
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> Vec<Mask> {
        self.masks.clone()
    }
}

impl Layer {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let name = Reflect::get(value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Layer.name must be a string"))?;

        let kind_js = Reflect::get(value, &JsValue::from_str("kind"))?;
        // For enums, you might pass numbers from JS or strings. Here, assume integer variant:
        let kind_num = kind_js.as_f64().ok_or_else(|| JsValue::from_str("Layer.kind must be a number"))?;
        let kind = match kind_num as u32 {
            0 => LayerType::Default,
            1 => LayerType::Texture,
            2 => LayerType::Block,
            3 => LayerType::Action,
            _ => return Err(JsValue::from_str("Invalid LayerType value")),
        };

        let shape_js = Reflect::get(value, &JsValue::from_str("shape"))?;
        let width = Reflect::get(&shape_js, &JsValue::from_str("width"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.width must be a number"))? as i32;
        let height = Reflect::get(&shape_js, &JsValue::from_str("height"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.height must be a number"))? as i32;
        let shape = Shape { width, height };

        let masks_js = Reflect::get(value, &JsValue::from_str("masks"))?;
        let masks_array = masks_js.dyn_ref::<Array>().ok_or_else(|| JsValue::from_str("Layer.masks must be an Array"))?;
        let mut masks_vec = Vec::with_capacity(masks_array.length() as usize);
        for i in 0..masks_array.length() {
            let mask_js = masks_array.get(i);
            let mask = Mask::from_js_value(&mask_js)?;
            masks_vec.push(mask);
        }

        Ok(Layer { name, kind, shape, masks: masks_vec })
    }
}

impl Layer {
    fn to_native(&self) -> rpgx::prelude::Layer {
        rpgx::prelude::Layer::new(self.name.clone(), self.kind.to_native(), self.shape.to_native(), self.masks.iter().map(|m| m.to_native()).collect())
    }
}


// Map
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Map {
    name: String,
    layers: Vec<Layer>,
}

#[wasm_bindgen]
impl Map {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, layers: &JsValue) -> Result<Map, JsValue> {
        let layers_array = layers.dyn_ref::<Array>().ok_or_else(|| JsValue::from_str("Layers must be an Array"))?;
        let mut layers_vec = Vec::with_capacity(layers_array.length() as usize);
        for i in 0..layers_array.length() {
            let layer_js = layers_array.get(i);
            let layer = Layer::from_js_value(&layer_js)?;
            layers_vec.push(layer);
        }
        Ok(Map { name, layers: layers_vec })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn layers(&self) -> Vec<Layer> {
        self.layers.clone()
    }
}

impl Map {
    fn to_native(&self) -> rpgx::prelude::Map {
        rpgx::prelude::Map {
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_native()).collect(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Tile {
    id: i32,
    effect: Effect,
    pointer: Coordinates,  // assuming SingleSelector is just Coordinates here; adjust if not
    shape: Shape,
}

#[wasm_bindgen]
impl Tile {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, effect: Effect, pointer: Coordinates, shape: Shape) -> Tile {
        Tile { id, effect, pointer, shape }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    #[wasm_bindgen(getter)]
    pub fn effect(&self) -> Effect {
        self.effect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> Coordinates {
        self.pointer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_pointer(&mut self, pointer: Coordinates) {
        self.pointer = pointer;
    }

    #[wasm_bindgen(getter)]
    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }
}

impl Tile {
    pub fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let id = Reflect::get(value, &JsValue::from_str("id"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Tile.id must be a number"))? as i32;

        let effect_js = Reflect::get(value, &JsValue::from_str("effect"))?;
        let effect = Effect::from_js_value(&effect_js)?;

        let pointer_js = Reflect::get(value, &JsValue::from_str("pointer"))?;
        let pointer = Coordinates::from_js_value(&pointer_js)?;

        let shape_js = Reflect::get(value, &JsValue::from_str("shape"))?;
        let width = Reflect::get(&shape_js, &JsValue::from_str("width"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.width must be a number"))? as i32;
        let height = Reflect::get(&shape_js, &JsValue::from_str("height"))?
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Shape.height must be a number"))? as i32;
        let shape = Shape { width, height };

        Ok(Tile { id, effect, pointer, shape })
    }

    pub fn to_native(&self) -> rpgx::engine::map::tile::Tile {
        rpgx::engine::map::tile::Tile {
            id: self.id,
            effect: self.effect.to_native(),
            pointer: rpgx::common::coordinates::Coordinates { x: self.pointer.x, y: self.pointer.y },  // adapt if SingleSelector is not Coordinates
            shape: self.shape.to_native(),
        }
    }
}


// Pawn wrapper
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pawn {
    tile: Tile,
    texture: String,
}

#[wasm_bindgen]
impl Pawn {
    #[wasm_bindgen(constructor)]
    pub fn new(tile: Tile, texture: String) -> Pawn {
        Pawn { tile, texture }
    }

    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> Tile {
        self.tile.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = tile;
    }

    #[wasm_bindgen(getter)]
    pub fn texture(&self) -> String {
        self.texture.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_texture(&mut self, texture: String) {
        self.texture = texture;
    }
}

impl Pawn {
    fn from_js_value(value: &JsValue) -> Result<Self, JsValue> {
        let tile_js = Reflect::get(value, &JsValue::from_str("tile"))?;
        let tile = Tile::from_js_value(&tile_js)?;

        let texture = Reflect::get(value, &JsValue::from_str("texture"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Pawn.texture must be a string"))?;

        Ok(Pawn { tile, texture })
    }

    fn to_native(&self) -> rpgx::prelude::Pawn {
        rpgx::prelude::Pawn {
            tile: self.tile.to_native(),
            texture: self.texture.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct WasmEngine {
    inner: RefCell<rpgx::engine::Engine>
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(map: Map, pawn: Pawn) -> WasmEngine {
        let engine = rpgx::engine::Engine::new(map.to_native(), pawn.to_native());
        WasmEngine { inner: RefCell::new(engine) }
    }

    /// Asynchronously walk to a target position (x, y)
   #[wasm_bindgen]
    pub fn walk_to(&self, x: i32, y: i32) -> Promise {
        let target = Coordinates { x, y };

        let inner = self.inner.clone();  // clone RefCell (cheap, it's a pointer)
        
        let fut = async move {
            // Borrow mutably inside the async block
            let mut engine = inner.borrow_mut();

            engine.walk_to(target.to_native()).await.map_err(|e| {
                JsValue::from_str(&format!("walk_to failed: {:?}", e))
            })?;

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
        self.inner.borrow_mut().step_to(dir).map_err(|e| JsValue::from_str(&format!("step_to failed: {:?}", e)))
    }

    /// Move directly to coordinates (x, y)
    #[wasm_bindgen]
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), JsValue> {
        let target = Coordinates { x, y };
        self.inner.borrow_mut().move_to(target.to_native()).map_err(|e| JsValue::from_str(&format!("move_to failed: {:?}", e)))
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