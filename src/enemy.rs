use std::path::Path;

use graphics::{rectangle::square, DrawState, Image};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;
use rand::{thread_rng, Rng};

use crate::{
    corner::Corner,
    utilities::{
        pick_random_direction, Blinky,
        Direction::{self, *},
        Moveable, ENEMY_SIZE, GHOST_SPEED, SCREEN_WIDTH,
    },
};

pub enum Behave {
    Chase,
    Scatter,
    Frightened,
}

pub mod behavior {
    pub trait ChaseT {
        fn chase(&mut self);
    }

    pub trait ScatterT {
        fn scatter(&mut self);
    }

    pub trait FrightenedT {
        fn frightened(&mut self);
    }
}

pub enum Color {
    Red,
    Blue,
    Purple,
    Green,
}

#[allow(unused)]
pub struct Ghost {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    speed: f64,
    pub direction: Direction,
    behave: Behave,
    pub moving: bool,
    ghost_texture_right: Box<Texture>,
    ghost_texture_mid: Box<Texture>,
    ghost_texture_left: Box<Texture>,
    target: (f64, f64),
}

trait AI {
    fn new_speed(&mut self);
}

impl Ghost {
    pub fn new(x: f64, y: f64, speed: f64, ghost_color: Color, target: (f64, f64)) -> Self {
        Ghost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            behave: Behave::Scatter,
            moving: true,
            ghost_texture_right: match ghost_color {
                Color::Red => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\red-ghost-right.png"), // make these mid sprite
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Blue => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\blue-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Green => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\green-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Purple => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\purple-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
            },
            ghost_texture_mid: match ghost_color {
                Color::Red => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\red-ghost-right.png"), // make these mid sprite
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Blue => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\blue-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Green => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\green-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Purple => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\purple-ghost-right.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
            },
            ghost_texture_left: match ghost_color {
                Color::Red => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\red-ghost-left.png"), // make these mid sprite
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Blue => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\blue-ghost-left.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Green => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\green-ghost-left.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Purple => Box::new(
                    Texture::from_path(
                        Path::new(".\\assets\\ghosts\\purple-ghost-left.png"),
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
            },
            target,
        }
    }

    pub fn rethink(&mut self) {
        self.direction = Direction::randomize();
    }

    pub fn update(&mut self) {
        self.move_o();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gl| {
            Image::new().rect(square(self.x, self.y, ENEMY_SIZE)).draw(
                match self.direction {
                    Left => &*self.ghost_texture_left,
                    Right => &*self.ghost_texture_right,
                    _ => &*self.ghost_texture_mid,
                },
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }

    pub fn behavior_change(&mut self, new: Behave) {
        self.direction.reverse_direction();
        self.behave = new;
    }
}

impl Moveable for Ghost {
    fn move_o(&mut self) {
        if self.moving {
            match self.direction {
                Direction::Up => self.y -= self.speed,
                Direction::Down => self.y += self.speed,
                Direction::Right => self.x += self.speed,
                Direction::Left => self.x -= self.speed,
            }
        }
    }

    fn move_e(&mut self, corner: &Corner, px: f64, py: f64) {
        match self.behave {
            Behave::Scatter => (),
            Behave::Chase => (),
            Behave::Frightened => (),
        }
    }
}

impl AI for Ghost {
    fn new_speed(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed * speed_mult(5),
            Direction::Down => self.y += self.speed * speed_mult(5),
            Direction::Right => self.x += self.speed * speed_mult(5),
            Direction::Left => self.x -= self.speed * speed_mult(5),
        }
    }
}

fn speed_mult(factor: i32) -> f64 {
    thread_rng().gen_range(1..=factor) as f64
}

/*
// the aggressive ghost, aims for the player
pub trait Blinky {
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// this one aims for 4 "squares" in front of the player
pub trait Pinky {
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// the flank ghost
// aims for;
// vector from 2 "squares" in front of the player to Blinky rotated 180 deg
pub trait Inky {
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// this ghost is random and does his own thing
pub trait Clyde {
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}
*/

impl Blinky for Ghost {
    fn retarget_b(&mut self, px: f64, py: f64) {
        match self.behave {
            Behave::Chase => self.target = (px, py),
            Behave::Frightened => (),
            Behave::Scatter => self.target = (SCREEN_WIDTH, 0.),
        }
    }

    fn move_b(&mut self, corner: &Option<Corner>) {
        if let Some(c) = corner {
            self.direction = pick_random_direction(&c.directions);
        }

        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
        }
    }
}
