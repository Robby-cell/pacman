use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::Transformed;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use opengl_graphics::TextureSettings;
use piston::input::*;

use crate::enemy::*;
use crate::map::Wall;
use crate::map::WallType;
use crate::player;
use crate::points;

use player::*;
use points::*;

pub struct Game {
    gl: GlGraphics,
    pub player: Player,
    pub score: i32,
    pub collects: Box<Vec<Option<Point>>>,
    pub ghosts: Box<[Option<Ghost>; 4]>,
    walls: Box<[Wall]>,
    //glyph_cache: GlyphCache,
}

impl Game {
    pub fn new(gl: GlGraphics, x: f64, y: f64, health: i32) -> Self {
        Game {
            gl,
            player: Player::new(x, y, health),
            score: 0,
            collects: Box::new(vec![Some(Point::new(50., 70., 1))]),
            ghosts: Box::new([
                Some(Ghost::new(67., 56., 2., 1., Color::Red)),
                None,
                None,
                None,
            ]),
            walls: Box::new([Wall::new(
                OpenGL::V4_2,
                WallType::Horizontal,
                vec![(40., 40.), (60., 60.)],
                vec![60., 20.],
                vec![20., 40.],
                [0., 0., 1., 1.0],
            )]), //glyph_cache: GlyphCache::new(),
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0., 0., 0.1, 1.], gl);

            let texture_settings: TextureSettings = TextureSettings::new().filter(Filter::Nearest);
            let ref mut glyphs =
                GlyphCache::new("assets/RoadPixel.ttf", (), texture_settings).unwrap();

            graphics::text(
                [1., 0., 0., 1.],
                22,
                &format!("Score: {}", self.score),
                glyphs,
                c.transform.trans(50., 250.),
                gl,
            )
            .unwrap();

            for collect in self.collects.iter_mut() {
                if let Some(ref mut c) = collect {
                    c.render(args) // render the collectible points.
                }
            }

            self.player.render(gl, args);
        });

        for wall in self.walls.iter_mut() {
            wall.render(args);
        }
    }

    pub fn button_pressed(&mut self, btn: &Button) {
        match btn {
            _ => self.player.change_direction(btn),
        }
    }
}
