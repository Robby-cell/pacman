use crate::utilities::Direction;

// this struct represents an intersection (like a T junction or whatever)
// purpose of this is to make it easier to decide where the ghosts need to turn.
pub struct Corner {
    pub x: f64,
    pub y: f64,
    pub directions: Box<[Direction]>,
}

impl Corner {
    pub fn new(x: f64, y: f64, directions: Box<[Direction]>) -> Self {
        Self { x, y, directions }
    }
}
