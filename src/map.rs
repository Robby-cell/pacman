use crate::utilities::WALL_SIZE;
use crate::utilities::TOLERANCE;
use crate::corner::Corner;
use crate::utilities::Direction;
use crate::enemy::Ghost;

use std::path::Path;

use graphics::{
    rectangle::{self, square},
    DrawState, Image, Rectangle,
};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;

#[derive(Debug)]
pub struct Map {
    walls: Vec<Wall>,
    corners: Vec<Corner>,
}

impl Map {
    pub fn new(map_args: (Vec<Wall>, Vec<Corner>)) -> Self {
        Self {
            walls: map_args.0,
            corners: map_args.1,
        }
    }
    
    pub fn can_turn(&self, x: f64, y: f64, next_dir: &Direction) -> bool {
        todo!("maybe try implementing this method?");
        true
    }
    
    #[no_mangle]
    pub fn ghost_check(&self, ghost: &mut dyn Ghost) {
        for corner in &self.corners {
            ();
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Wall {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    color: [f32; 4],
}

impl Wall {
    pub fn new((x0, y0): (f64, f64), width: f64, height: f64, color: [f32; 4]) -> Self {
        Self {
            x0,
            y0,
            x1: x0 + width,
            y1: y0 + height,
            color,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |context, g| {
            graphics::rectangle(
                self.color,
                rectangle::rectangle_by_corners(self.x0, self.y0, self.x1, self.y1),
                context.transform,
                g,
            );
        })
    }

    pub fn touched(&self, x: &f64, y: &f64) -> bool {
        false
    }
}
