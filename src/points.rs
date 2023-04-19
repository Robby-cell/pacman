use derivative::Derivative;
use opengl_graphics::*;
use piston::RenderArgs;

// this is a collectible item that can be picked up to increase the players score...
#[derive(Derivative)]
pub struct Point {
    gl: GlGraphics,
    #[derivative(Default)]
    pub x: f64,
    #[derivative(Default)]
    pub y: f64,
    #[derivative(Default)]
    pub worth: i32,
    #[derivative(Default)]
    pub size: f64,
    color: [f32; 4],
}

impl Point {
    pub fn new(x: f64, y: f64, worth: i32, color: [f32; 4]) -> Self {
        Point {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            worth,
            size: match worth {
                1 => 8.,
                3 => 13.,
                _ => 0.,
            },
            color,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let rect = graphics::rectangle::square(self.x, self.y, self.size);

        self.gl.draw(args.viewport(), |c, g| {
            graphics::ellipse(self.color, rect, c.transform, g)
        })
    }
}
