use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::enemy::*;
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
    pub collects: Box<Vec<Option<Point>>>,
    pub ghosts: Box<[Option<Ghost>; 4]>,
}

impl Game {
    pub fn new(gl: GlGraphics, x: f64, y: f64, health: i32) -> Self {
        Game {
            gl,
            player: Player::new(x, y, health),
            score: 0,
            collects: Box::new(vec![Some(Point::new(50., 70., 1))]),
            ghosts: Box::new([Some(Ghost::new(67., 56., 2., Color::Red)), None, None, None]),
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear([0., 0., 0.1, 1.], gl);

            for collect in self.collects.iter_mut() {
                if let Some(ref mut c) = collect {
                    c.render(args) // render the collectible points.
                }
            }
            self.player.render(gl, args);
        })
    }

    pub fn button_pressed(&mut self, btn: &Button) {
        match btn {
            _ => self.player.change_direction(btn),
        }
    }
}
