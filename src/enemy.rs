use std::path::Path;

use opengl_graphics::{Texture, TextureSettings};

pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

pub struct Ghost {
    x: f64,
    y: f64,
    speed: f64,
    ghost_texture: Box<Texture>,
}

impl Ghost {
    pub fn new(x: f64, y: f64, speed: f64, ghost_color: Color) -> Self {
        Self {
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
}
