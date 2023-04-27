// constants go here, and any universal utilities (enums, structs, traits that multiples mods will use)

use piston::RenderArgs;
use rand::Rng;

use crate::{corner::Corner, point::PointType};

pub const PLAYER_SIZE: f64 = 40_f64;
pub const ENEMY_SIZE: f64 = 40_f64;
pub const SCREEN_HEIGHT: f64 = 800_f64;
pub const SCREEN_WIDTH: f64 = 1500_f64;
pub const GHOST_SPEED: f64 = 1_f64;
pub const WALL_SIZE: f64 = 50_f64;
pub const UNIT_SIZE: f64 = 20_f64;
pub const PLAYER_SPEED: f64 = 3_f64;

pub trait Reset {
    fn reset(&mut self);
}

pub trait Moveable {
    fn move_o(&mut self);
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64) {}
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

// following: we will have the ghosts change their target based on where the player is and their state

// the aggressive ghost, aims for the player
pub trait Blinky {
    fn retarget_b(&mut self, px: f64, py: f64);
    fn move_b(&mut self, corner: &Option<Corner>);
}

// this one aims for 4 "squares" in front of the player
pub trait Pinky {
    fn retarget_p(&mut self, px: f64, py: f64);
    fn move_p(&mut self, corner: &Option<Corner>);
}

// the flank ghost
// aims for;
// vector from 2 "squares" in front of the player to Blinky rotated 180 deg
pub trait Inky {
    fn retarget_i(&mut self, px: f64, py: f64);
    fn move_i(&mut self, corner: &Option<Corner>);
}

// this ghost is random and does his own thing
pub trait Clyde {
    fn retarget_c(&mut self, px: f64, py: f64);
    fn move_c(&mut self, corner: &Option<Corner>);
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn reverse_direction(&mut self) {
        *self = match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn randomize() -> Self {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        }
    }
}

pub enum State {
    Play,
    Menu,
    StartScreen,
}

pub trait Collectible {
    fn new(x: f64, y: f64, color: [f32; 4], point_type: PointType) -> Self;
    fn collect(&mut self) -> i32;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn size(&self) -> f64;
    fn render(&mut self, args: &RenderArgs);
}

pub fn pick_random_direction(directions: &Box<[Direction]>) -> Direction {
    let ind = rand::thread_rng().gen_range(0..directions.len());

    directions[ind]
}
