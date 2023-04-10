use derivative::Derivative;
use opengl_graphics::*;
use piston::RenderArgs;

// this is a collectible item that can be picked up to increase the players score...
#[derive(Derivative)]
pub struct Point {
    gl: GlGraphics,
    #[derivative(Default)]
    x: f64,
    #[derivative(Default)]
    y: f64,
    #[derivative(Default)]
    worth: i32,
    #[derivative(Default)]
    size: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, worth: i32) -> Self {
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
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let rect = graphics::rectangle::square(self.x, self.y, self.size);

        self.gl.draw(args.viewport(), |c, g| {
            graphics::ellipse([1., 1., 1., 1.], rect, c.transform, g)
        })
    }
}
