use crate::prelude::{Coordinates, Direction};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RPGXError {
    TileNotWalkable(Coordinates),
    TileNotFound(Coordinates),
    PathNotFround { from: Coordinates, to: Coordinates },
    PawnNotFound,
    WalkFailed { from: Coordinates, to: Coordinates },
    StepFailed(Direction),
}
