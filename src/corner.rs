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

    pub fn next_dir(&self, playerx: &f64, playery: &f64, ghostx: &f64, ghosty: &f64, to_avoid: &Direction) -> Direction {
        for &dir in self.directions.iter() {
            match dir {
                Direction::Up => {
                    if playery < ghosty && dir != to_avoid.reversed() { return Direction::Up; }
                }
                Direction::Down => {
                    if playery > ghosty && dir != to_avoid.reversed() { return Direction::Down; }
                }
                Direction::Left => {
                    if playerx < ghostx && dir != to_avoid.reversed() { return Direction::Left; }
                }
                Direction::Right => {
                    if playerx > ghostx && dir != to_avoid.reversed() { return Direction::Right; }
                }
            }
        }
        self.directions[0]
    }
}
