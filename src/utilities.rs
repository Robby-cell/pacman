// constants go here, and any universal utilities (enums, structs, traits that multiples mods will use)

pub const PLAYER_SIZE: f64 = 50_f64;
pub const ENEMY_SIZE: f64 = 40_f64;
pub const SCREEN_HEIGHT: f64 = 800_f64;
pub const SCREEN_WIDTH: f64 = 1500_f64;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
