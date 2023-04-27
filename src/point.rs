use derivative::Derivative;
use opengl_graphics::*;
use piston::RenderArgs;

use crate::utilities::Collectible;

pub enum PointType {
    Big,
    Small,
}

// this is a collectible item that can be picked up to increase the players score...
#[derive(Derivative)]
pub struct Point {
    gl: GlGraphics,
    #[derivative(Default)]
    pub x: f64,
    #[derivative(Default)]
    pub y: f64,
    #[derivative(Default)]
    pub size: f64,
    color: [f32; 4],
    point_type: PointType,
}

impl Point {}

impl Collectible for Point {
    fn new(x: f64, y: f64, color: [f32; 4], point_type: PointType) -> Self {
        use PointType::*;
        Point {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            size: match point_type {
                Big => 12.,
                Small => 8.,
            },
            color,
            point_type,
        }
    }

    fn collect(&mut self) -> i32 {
        use PointType::*;
        match self.point_type {
            Big => 30,
            Small => 10,
        }
    }

    fn size(&self) -> f64 {
        self.size
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn render(&mut self, args: &RenderArgs) {
        let rect = graphics::rectangle::square(self.x, self.y, self.size);

        self.gl.draw(args.viewport(), |c, g| {
            graphics::ellipse(self.color, rect, c.transform, g)
        })
    }
}
