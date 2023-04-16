use crate::utilities::WALL_SIZE;

use std::path::Path;

use graphics::{
    rectangle::{self, square},
    DrawState, Image, Rectangle,
};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;

#[derive(Clone)]
pub enum WallType {
    LRight,
    LLeft,
    Vertical,
    Horizontal,
    TFlatUp,
    TFlatLeft,
    TFlatRight,
    TFlatDown,
}

pub struct Wall {
    gl: GlGraphics,
    wall_type: WallType,
    //texture: Box<Texture>,
    rects: Vec<[f64; 4]>,
    color: [f32; 4],
}

impl Wall {
    pub fn new(
        opengl: OpenGL,
        wall_type: WallType,
        origin: Vec<(f64, f64)>,
        width: Vec<f64>,
        height: Vec<f64>,
        color: [f32; 4],
    ) -> Self {
        let mut rects: Vec<[f64; 4]> = Vec::new();
        for i in 0..origin.len() {
            rects.push(rectangle::rectangle_by_corners(
                origin[i].0,
                origin[i].1,
                origin[i].0 + width[i],
                origin[i].1 + height[i],
            ))
        }

        Self {
            gl: GlGraphics::new(opengl),
            wall_type: wall_type.clone(),
            // was thinking of doing it this way before adding textures for walls, but the idea is here and textures are missing from assets
            /*
            texture: Box::new(
                Texture::from_path(
                    Path::new(match wall_type {
                        WallType::Horizontal => "./assets/wall/Horizontal_wall.png",
                        WallType::Vertical => "./assets/wall/Vertical_wall.png",
                        WallType::LLeft => "./assets/wall/LLeft_wall.png",
                        WallType::LRight => "./assets/wall/LRight_wall.png",
                        WallType::TFlatUp => "./assets/wall/TFlatUp.png",
                        WallType::TFlatRight => "./assets/wall/TFlatRight.png",
                        WallType::TFlatDown => "./assets/wall/TFlatDown.png",
                        WallType::TFlatLeft => "./assets/wall/TFlatLeft.png",
                    }),
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            */
            rects,
            color,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        for rect in self.rects.iter() {
            self.gl.draw(args.viewport(), |context, gl| {
                graphics::rectangle(self.color, *rect, context.transform, gl);
            })
        }
    }

    pub fn touched(&self, x: &f64, y: &f64) -> bool {
        true
    }
}
