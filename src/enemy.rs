use std::path::Path;

use graphics::{rectangle::square, DrawState, Image};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;
use rand::{thread_rng, Rng};

use crate::utilities::{Direction, Moveable, ENEMY_SIZE, GHOST_SPEED};

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
    pub moving: bool,
    ghost_texture: Box<Texture>,
}

trait AI {
    fn new_speed(&mut self);
}

impl Ghost {
    pub fn new(x: f64, y: f64, speed: f64, ghost_color: Color) -> Self {
        Ghost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            moving: true,
            ghost_texture: match ghost_color {
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
                &*self.ghost_texture,
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }
}

impl Moveable for Ghost {
    fn move_o(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
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
