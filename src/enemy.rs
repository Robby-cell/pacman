use std::path::Path;

use graphics::Image;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;
use rand::{thread_rng, Rng};

use crate::utilities::GHOST_SPEED;

pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

#[allow(unused)]
pub struct Ghost {
    gl: GlGraphics,
    x: f64,
    y: f64,
    speedx: f64,
    speedy: f64,
    ghost_texture: Box<Texture>,
}

trait AI {
    fn new_speed(&mut self);
}

impl Ghost {
    pub fn new(x: f64, y: f64, speedx: f64, speedy: f64, ghost_color: Color) -> Self {
        Ghost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speedx,
            speedy,
            ghost_texture: match ghost_color {
                Color::Red => Box::new(
                    Texture::from_path(
                        Path::new(
                            "C:\\Users\\Robert\\OneDrive\\Pictures\\Saved Pictures\\image.png", // temp images
                        ), // temporary
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Blue => Box::new(
                    Texture::from_path(
                        Path::new(
                            "C:\\Users\\Robert\\OneDrive\\Pictures\\Saved Pictures\\image.png",
                        ), // temporary
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Green => Box::new(
                    Texture::from_path(
                        Path::new(
                            "C:\\Users\\Robert\\OneDrive\\Pictures\\Saved Pictures\\image.png",
                        ), // temporary
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
                Color::Yellow => Box::new(
                    Texture::from_path(
                        Path::new(
                            "C:\\Users\\Robert\\OneDrive\\Pictures\\Saved Pictures\\image.png",
                        ), // temporary
                        &TextureSettings::new(),
                    )
                    .unwrap(),
                ),
            },
        }
    }

    pub fn update(&mut self) {

        // some other updates and calculations to be done here
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            Image::new();
            //some stuff we will do later once we sort out  textures.
            todo!("todo")
        })
    }
}

impl AI for Ghost {
    fn new_speed(&mut self) {
        self.speedx = 0.5 * speed_mult(5);
        self.speedy = 0.5 * speed_mult(5);
    }
}

fn speed_mult(factor: i32) -> f64 {
    thread_rng().gen_range(1..=factor) as f64
}
