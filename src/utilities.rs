// constants go here, and any universal utilities (enums, structs, traits that multiples mods will use)

use graphics::glyph_cache::rusttype::GlyphCache;
use opengl_graphics::TextureSettings;

pub const PLAYER_SIZE: f64 = 50_f64;
pub const ENEMY_SIZE: f64 = 40_f64;
pub const SCREEN_HEIGHT: f64 = 800_f64;
pub const SCREEN_WIDTH: f64 = 1500_f64;
pub const GHOST_SPEED: f64 = 1_f64;
pub const WALL_SIZE: f64 = 50_f64;

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

pub enum State {
    Play,
    Menu,
    StartScreen,
}
