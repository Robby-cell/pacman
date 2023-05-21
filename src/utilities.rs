// constants go here, and any universal utilities (enums, structs, traits that multiples mods will use)

use piston::RenderArgs;
use rand::Rng;

use crate::{corner::Corner, point::PointType};

pub const PLAYER_SIZE: f64 = 40_f64;
pub const HALF_PLAYER: f64 = PLAYER_SIZE / 2_f64;
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const ENEMY_SIZE: f64 = 40_f64;
pub const SCREEN_HEIGHT: f64 = 10_f64 * PLAYER_SIZE;
pub const SCREEN_WIDTH: f64 = 2.5f64 * SCREEN_HEIGHT;
pub const GHOST_SPEED: f64 = 2_f64;
pub const WALL_SIZE: f64 = 50_f64;
pub const UNIT_SIZE: f64 = 20_f64;
pub const PLAYER_SPEED: f64 = 2_f64;

pub trait Reset {
    fn reset(&mut self);
}

pub trait Moveable {
    fn move_o(&mut self);
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64) {}
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn reverse_direction(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn reversed(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn randomize() -> Self {
        use Direction::*;
        match rand::thread_rng().gen_range(0..=3) {
            0 => Up,
            1 => Right,
            2 => Down,
            _ => Left,
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
