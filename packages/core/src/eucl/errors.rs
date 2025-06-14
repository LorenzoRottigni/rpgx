use crate::prelude::Rect;

#[derive(Debug)]
pub enum RectError {
    EmptyRectList,
    InvalidJoin(Vec<Rect>),
}
pub enum CoordinatesError {}

pub enum DeltaError {}

pub enum ShapeError {}
