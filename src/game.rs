use std::vec;

use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::Transformed;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use opengl_graphics::TextureSettings;
use piston::input::{Button, RenderArgs};

use crate::enemy::*;
use crate::map::Wall;
use crate::player;
use crate::points;
use crate::utilities::Direction;
use crate::utilities::ENEMY_SIZE;
use crate::utilities::PLAYER_SIZE;

use player::*;
use points::*;

pub struct Game {
    gl: GlGraphics,
    pub player: Player,
    pub score: i32,
    pub collects: Box<Vec<Option<Point>>>,
    pub ghosts: Box<[Option<Ghost>; 4]>,
    wall_gl: GlGraphics,
    walls: Box<[Wall]>,
    //glyph_cache: GlyphCache,
}

impl Game {
    pub fn new(gl: GlGraphics, x: f64, y: f64, health: i32) -> Self {
        Game {
            gl,
            player: Player::new(x, y, health),
            score: 0,
            collects: Box::new(vec![Some(Point::new(500., 700., 1, [1., 1., 1., 1.]))]),
            ghosts: Box::new([
                Some(Ghost::new(300., 300., 2., Color::Red)),
                None,
                None,
                None,
            ]),
            wall_gl: GlGraphics::new(OpenGL::V4_2),
            walls: Box::new([
                Wall::new((40., 40.), 360., 60., [0., 0., 1., 1.]),
                Wall::new((600., 200.), 60., 300., [0., 0., 1., 1.]),
            ]), //glyph_cache: GlyphCache::new(),
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        for ghost in self.ghosts.iter_mut() {
            if let Some(g) = ghost {
                for wall in self.walls.iter_mut() {
                    if g.x - ENEMY_SIZE >= wall.x0
                        && g.x <= wall.x1
                        && g.y - ENEMY_SIZE >= wall.y0
                        && g.y <= wall.y1
                    {
                        g.moving = false;
                        g.rethink();
                        if g.x <= wall.x0 && g.direction != Direction::Right
                            || g.x >= wall.x1 && g.direction != Direction::Left
                            || g.y >= wall.y1 && g.direction != Direction::Up
                            || g.y <= wall.y0 && g.direction != Direction::Down
                        {
                            g.moving = true;
                        }
                    }
                }
                g.update();
            }
        }

        for point in self.collects.iter_mut() {
            if let Some(p) = point {
                if p.x > self.player.x - p.size
                    && p.x < self.player.x + PLAYER_SIZE
                    && p.y > self.player.y - p.size
                    && p.y < self.player.y + PLAYER_SIZE
                {
                    self.score += p.worth;
                    *point = None;
                }
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0., 0., 0.1, 1.], gl);

            let texture_settings: TextureSettings = TextureSettings::new().filter(Filter::Nearest);
            let ref mut glyphs =
                GlyphCache::new("./assets/fonts/RoadPixel.ttf", (), texture_settings).unwrap();

            graphics::text(
                [1., 0., 0., 1.],
                22,
                format!("Score: {}", self.score).as_str(),
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

            for ghost in self.ghosts.iter_mut() {
                if let Some(g) = ghost {
                    g.render(args);
                }
            }
        });

        for wall in self.walls.iter_mut() {
            wall.render(args, &mut self.wall_gl);
        }
    }

    pub fn button_pressed(&mut self, btn: &Button) {
        match btn {
            _ => self.player.change_direction(btn),
        }
    }
}
