// constants go here, and any universal utilities (enums, structs, traits that multiples mods will use)

use rand::Rng;

pub const PLAYER_SIZE: f64 = 40_f64;
pub const ENEMY_SIZE: f64 = 40_f64;
pub const SCREEN_HEIGHT: f64 = 800_f64;
pub const SCREEN_WIDTH: f64 = 1500_f64;
pub const GHOST_SPEED: f64 = 1_f64;
pub const WALL_SIZE: f64 = 50_f64;
pub const UNIT_SIZE: f64 = 20_f64;

pub trait Reset {
    fn reset(&mut self);
}

pub trait Moveable {
    fn move_o(&mut self);
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn reverse_direction(&mut self) {
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
