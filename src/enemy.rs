use std::path::Path;

use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;

pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

pub struct Ghost {
    gl: GlGraphics,
    x: f64,
    y: f64,
    speed: f64,
    ghost_texture: Box<Texture>,
}

impl Ghost {
    pub fn new(x: f64, y: f64, speed: f64, ghost_color: Color) -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            ghost_texture: match ghost_color {
                Color::Red => Box::new(
                    Texture::from_path(
                        Path::new(
                            "C:\\Users\\Robert\\OneDrive\\Pictures\\Saved Pictures\\image.png",
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
        todo!("update movement etc")
    }

    pub fn render(&mut self, args: &RenderArgs) {
        todo!("render the enemy")
    }
}
