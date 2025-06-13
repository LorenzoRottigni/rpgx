#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    pub fn invert(self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    pub fn is_zero(self) -> bool {
        self.dx == 0 && self.dy == 0
    }
}
