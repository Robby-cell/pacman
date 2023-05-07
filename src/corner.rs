use rand::Rng;

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

    // simple function to calculate where the ghost turns at a corner, does not allow them to go back on themselves
    // and biased towards up, then right like original game
    pub fn next_dir(
        &self,
        targetx: &f64,
        targety: &f64,
        ghostx: &f64,
        ghosty: &f64,
        current_dir: &Direction,
    ) -> Direction {
        for &dir in self.directions.iter() {
            match dir {
                Direction::Up => {
                    if targety <= ghosty && dir != current_dir.reversed() {
                        return Direction::Up;
                    }
                }
                Direction::Right => {
                    if targetx >= ghostx && dir != current_dir.reversed() {
                        return Direction::Right;
                    }
                }
                Direction::Left => {
                    if targetx <= ghostx && dir != current_dir.reversed() {
                        return Direction::Left;
                    }
                }
                Direction::Down => {
                    if targety >= ghosty && dir != current_dir.reversed() {
                        return Direction::Down;
                    }
                }
            }
        }
        self.directions[0]
    }

    pub fn next_frightened(&self) -> Direction {
        self.directions[rand::thread_rng().gen_range(0..self.directions.len())]
    }
}
