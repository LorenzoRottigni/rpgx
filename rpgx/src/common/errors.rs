use crate::common::coordinates::Coordinates;

#[derive(Debug)]
pub enum MoveError {
    TileBlocked,
    TileNotFound,
    TileOutOfBounds,
    PathNotFound,
    StepFailed(Coordinates),
}

#[derive(Debug)]
pub enum MapError {
    TileNotFound,
}
