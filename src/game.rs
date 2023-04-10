use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::player;
use crate::points;

use player::*;
use points::*;

pub trait Moveable {
    fn move_o(&mut self);
}

pub trait Reset {
    fn reset(&mut self);
}

pub struct Game {
    gl: GlGraphics,
    pub player: Player,
    pub score: i32,
    pub collects: Vec<Option<Point>>,
}

impl Game {
    pub fn new(gl: GlGraphics, x: f64, y: f64, health: i32) -> Self {
        Game {
            gl,
            player: Player::new(x, y, health),
            score: 0,
            collects: vec![Some(Point::new(4., 65., 1))],
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear([0., 0., 0.1, 1.], gl);

            self.player.render(gl, args);
            for collect in self.collects.iter_mut() {
                if let Some(ref mut c) = collect {
                    c.render(args)
                }
            }
        })
    }

    pub fn button_pressed(&mut self, btn: &Button) {
        match btn {
            _ => self.player.change_direction(btn),
        }
    }
}
