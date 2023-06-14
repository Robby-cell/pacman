use std::vec;

use glutin_window::GlutinWindow;
use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::{DrawState, Transformed};
use piston::RenderEvent;
use piston::ButtonEvent;

use opengl_graphics::{Filter, GlGraphics, OpenGL, TextureSettings};

use piston::input::{Button, RenderArgs};
use piston::{WindowSettings, Events, EventSettings, EventLoop};

use crate::corner::Corner;

use crate::enemy::*;

use crate::map::Wall;

use crate::point::{PointType, Point};

use crate::startscreen::StartScreen;
use crate::utilities::*;

use crate::builder;
use crate::player::*;

use std::result::Result;
use std::error::Error;
pub fn mainloop() -> Result<(), Box<dyn Error>> {
    let opengl = OpenGL::V4_2;
    let mut state = State::StartScreen;

    let mut window: GlutinWindow =
        WindowSettings::new("Pacman!", (SCREEN_WIDTH, SCREEN_HEIGHT + BAND_SIZE))
            .exit_on_esc(true)
            .build()
            .ok()
            .ok_or("Could not create Window")?;

    let mut game = Game::<Point>::new(GlGraphics::new(opengl), 3);
    let mut events = Events::new(EventSettings::new()).ups(60);
    let mut startscreen = StartScreen::new(OpenGL::V4_2);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            match state {
                State::StartScreen => {
                    game.render(&r);

                    startscreen.render(&r);
                    ()
                }
                State::Play => {
                    game.update();
                    game.render(&r);
                }
                State::Menu => {
                    todo!("add menu state")
                }
            }
        }

        if let Some(k) = e.button_args() {
            match state {
                State::StartScreen => match &k.button {
                    &Button::Mouse(_) => (),
                    _ => {
                        state = State::Play;
                    }
                },
                State::Play => {
                    game.button_pressed(&k.button);
                }
                State::Menu => {
                    todo!("menu screen and its implementations");
                }
            }
        }
    }

    Ok(())
}

pub struct Game<T>
where
    T: Collectible,
{
    gl: GlGraphics,
    pub player: Player,
    pub score: i32,
    pub collects: Box<Vec<Option<T>>>,
    pub ghosts: Box<[Option<Box<dyn Ghost>>; 4]>,
    wall_gl: GlGraphics,
    walls: Vec<Wall>,
    corners: Vec<Corner>,
    // glyph_cache: GlyphCache,
}

impl<T> Game<T>
where
    T: Collectible,
{
    pub fn new(gl: GlGraphics, health: i32) -> Self {
        let sample_map = builder::builder::new_map(); // tuple of (all of the walls, all of the corners)

        let enemy_halfx = SCREEN_WIDTH / 2. - ENEMY_SIZE / 2.;
        let enemy_halfy = SCREEN_HEIGHT / 2. - ENEMY_SIZE / 2.;

        let pac_startx = SCREEN_WIDTH / 2. - PLAYER_SIZE / 2.;
        let pac_starty = SCREEN_HEIGHT - PLAYER_SIZE * 2.;

        Game {
            gl,
            player: Player::new(pac_startx, pac_starty, health),
            score: 0,
            collects: Box::new(vec![
                Some(T::new(500., 500., [1., 1., 1., 1.], PointType::Big)),
                None,
            ]),
            ghosts: Box::new([
                Some(Box::new(RedGhost::new(enemy_halfx, enemy_halfy-ENEMY_SIZE*2., GHOST_SPEED, (SCREEN_WIDTH, 0.)))),
                Some(Box::new(PurpleGhost::new(enemy_halfx, enemy_halfy, GHOST_SPEED, (0., 0.)))),
                Some(Box::new(GreenGhost::new(enemy_halfx, enemy_halfy, GHOST_SPEED, (0., SCREEN_HEIGHT)))),
                Some(Box::new(BlueGhost::new(enemy_halfx, enemy_halfy, GHOST_SPEED, (SCREEN_WIDTH, SCREEN_HEIGHT)))),
            ]),
            wall_gl: GlGraphics::new(OpenGL::V4_2),
            walls: /*Box::new([
                Wall::new((40., 40.), 360., 60., [0., 0., 1., 1.]),
                Wall::new((600., 200.), 60., 300., [0., 0., 1., 1.]),])*/
                sample_map.0,

            corners: sample_map.1,
            //glyph_cache: GlyphCache::new(),
        }
    }

    pub fn update(&mut self) {
        let (mut rx, mut ry): (f64, f64) = (0., 0.);
        for ghost in self.ghosts.iter_mut() {
            if let Some(g) = ghost {
                if g.is_red() {
                    (rx, ry) = (g.x(), g.y());
                }
                for wall in self.walls.iter() {
                    if (g.x() + ENEMY_SIZE >= wall.x0
                        && g.direction() == Direction::Right
                        && g.y() + ENEMY_SIZE < wall.y1
                        && g.y() > wall.y0
                        && g.x() <= wall.x1)
                        || (g.x() <= wall.x1
                            && g.direction() == Direction::Left
                            && g.y() + ENEMY_SIZE < wall.y1
                            && g.y() > wall.y0
                            && g.y() - ENEMY_SIZE >= wall.x0)
                        || (g.y() + ENEMY_SIZE >= wall.y0
                            && g.direction() == Direction::Down
                            && g.x() - ENEMY_SIZE >= wall.x0
                            && g.x() <= wall.x1
                            && g.y() <= wall.y1)
                        || (g.y() <= wall.y1
                            && g.direction() == Direction::Up
                            && g.x() - ENEMY_SIZE >= wall.x0
                            && g.x() <= wall.x1
                            && g.y() - ENEMY_SIZE >= wall.y0)
                    {
                        g.set_moving(false);
                        if (g.x() + ENEMY_SIZE <= wall.x0 + 0.5
                            && g.direction() != Direction::Right)
                            || (g.x() >= wall.x1 - 0.5 && g.direction() != Direction::Left)
                            || (g.y() >= wall.y1 - 0.5 && g.direction() != Direction::Up)
                            || (g.y() + ENEMY_SIZE <= wall.y0 + 0.5
                                && g.direction() != Direction::Down)
                        {
                            g.set_moving(true);
                        }
                    }
                }
                g.update(
                    self.player.x,
                    self.player.y,
                    &self.player.direction,
                    &rx,
                    &ry,
                );
            }
        }

        let mut moveplayer = true;
        for &wall in self.walls.iter() {
            if (self.player.x >= wall.x0 - PLAYER_SIZE
                && self.player.x < wall.x0 + 10. - PLAYER_SIZE
                && self.player.y > wall.y0 - PLAYER_SIZE + 1. // offsets so the player doesnt get stuck navigating 
                && self.player.y < wall.y1 - 1.
                && self.player.direction == Direction::Right)
                || (self.player.x <= wall.x1
                    && self.player.x > wall.x1 - 10.
                    && self.player.y > wall.y0 - PLAYER_SIZE + 1.
                    && self.player.y < wall.y1 - 1.
                    && self.player.direction == Direction::Left)
                || (self.player.y >= wall.y0 - PLAYER_SIZE
                    && self.player.y < wall.y0 + 10. - PLAYER_SIZE
                    && self.player.x > wall.x0 - PLAYER_SIZE + 1.
                    && self.player.x < wall.x1 - 1.
                    && self.player.direction == Direction::Down)
                || (self.player.y <= wall.y1
                    && self.player.y > wall.y1 - 10.
                    && self.player.x > wall.x0 - PLAYER_SIZE + 1.
                    && self.player.x < wall.x1 - 1.
                    && self.player.direction == Direction::Up)
            {
                moveplayer = false;
                break;
            }
        }
        if moveplayer {
            self.player.update();
        }

        for point in self.collects.iter_mut() {
            if let Some(p) = point {
                if p.x() > self.player.x - p.size()
                    && p.x() < self.player.x + PLAYER_SIZE
                    && p.y() > self.player.y - p.size()
                    && p.y() < self.player.y + PLAYER_SIZE
                {
                    self.score += p.collect();
                    *point = None;
                }
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        let position_y = SCREEN_HEIGHT + (BAND_SIZE + TEXT_SIZE as f64) / 2.;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0., 0., 0., 1.], gl);

            let texture_settings: TextureSettings = TextureSettings::new().filter(Filter::Nearest);
            let ref mut glyphs =
                GlyphCache::new("./assets/fonts/RoadPixel.ttf", (), texture_settings).unwrap();

            let line = graphics::line::Line::new([0., 1., 0., 1.], 1.);

            graphics::line::Line::draw(
                &line,
                [0., SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT],
                &DrawState::default(),
                c.transform,
                gl,
            );

            graphics::text(
                [1., 0., 0., 1.],
                TEXT_SIZE,
                format!("Score: {}", self.score).as_str(),
                glyphs,
                c.transform.trans(50., position_y),
                gl,
            )
            .unwrap();

            graphics::text(
                [1., 0., 0., 1.],
                TEXT_SIZE,
                format!("Lives: {}", self.player.health).as_str(),
                glyphs,
                c.transform.trans(SCREEN_WIDTH / 2., position_y),
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
        self.player.change_direction(btn)
    }
}
