use std::path::Path;

use graphics::{rectangle::square, DrawState, Image};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::RenderArgs;

use crate::{
    corner::Corner,
    utilities::{
        Direction::{self, *},
        ENEMY_SIZE, PLAYER_SIZE,
    },
};

pub enum Behave {
    Chase,
    Scatter,
    Frightened,
}

pub trait Behavior {
    // new chase target
    fn new_chase(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64);
    // new scatter target
    fn new_scatter(&mut self);
    // new frightened target (we will ignore since frightened does not have a target)
    fn new_frightened(&mut self);
    // basic movement (updating x and y positions)
    fn basic_movement(&mut self);
    // what to do at a corner
    fn at_corner(&mut self, corner: &Corner);
    // state change (scatter -> chase etc.)
    fn change(&mut self, new: Behave);
}

pub trait Ghost: Behavior {
    // updating...
    fn update(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64);
    // rendering...
    fn render(&mut self, args: &RenderArgs);
    // returning x and y values
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    // returns direction
    fn direction(&self) -> Direction;
    // returns whether ghost is moving or not
    fn moving(&self) -> bool;
    // set moving status
    fn set_moving(&mut self, moving: bool);
    // whether ghost is the red one
    fn is_red(&self) -> bool;
}

// todo: struct for each color, remove color enum, and implement
// each behavior with 1 single trait, Box<dyn Behavior + Ghost> or something similar
#[allow(unused)]
pub struct RedGhost {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    speed: f64,
    pub direction: Direction,
    behave: Behave,
    pub moving: bool,
    ghost_texture_right: Box<Texture>,
    ghost_texture_mid: Box<Texture>,
    ghost_texture_left: Box<Texture>,
    target: (f64, f64),
    default_target: (f64, f64),
}

pub struct PurpleGhost {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    speed: f64,
    pub direction: Direction,
    behave: Behave,
    pub moving: bool,
    ghost_texture_right: Box<Texture>,
    ghost_texture_mid: Box<Texture>,
    ghost_texture_left: Box<Texture>,
    target: (f64, f64),
    default_target: (f64, f64),
}

pub struct GreenGhost {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    speed: f64,
    pub direction: Direction,
    behave: Behave,
    pub moving: bool,
    ghost_texture_right: Box<Texture>,
    ghost_texture_mid: Box<Texture>,
    ghost_texture_left: Box<Texture>,
    target: (f64, f64),
    default_target: (f64, f64),
}

pub struct BlueGhost {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    speed: f64,
    pub direction: Direction,
    behave: Behave,
    pub moving: bool,
    ghost_texture_right: Box<Texture>,
    ghost_texture_mid: Box<Texture>,
    ghost_texture_left: Box<Texture>,
    target: (f64, f64),
    default_target: (f64, f64),
}

impl RedGhost {
    pub fn new(x: f64, y: f64, speed: f64, default_target: (f64, f64)) -> Self {
        RedGhost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            behave: Behave::Scatter,
            moving: true,
            ghost_texture_right: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/red-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_mid: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/red-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_left: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/red-ghost-left.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            target: default_target,
            default_target,
        }
    }
}
impl PurpleGhost {
    pub fn new(x: f64, y: f64, speed: f64, default_target: (f64, f64)) -> Self {
        PurpleGhost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            behave: Behave::Scatter,
            moving: true,
            ghost_texture_right: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/purple-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_mid: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/purple-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_left: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/purple-ghost-left.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            target: default_target,
            default_target,
        }
    }
}
impl GreenGhost {
    pub fn new(x: f64, y: f64, speed: f64, default_target: (f64, f64)) -> Self {
        GreenGhost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            behave: Behave::Scatter,
            moving: true,
            ghost_texture_right: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/green-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_mid: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/green-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_left: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/green-ghost-left.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            target: default_target,
            default_target,
        }
    }
}
impl BlueGhost {
    pub fn new(x: f64, y: f64, speed: f64, default_target: (f64, f64)) -> Self {
        BlueGhost {
            gl: GlGraphics::new(OpenGL::V4_2),
            x,
            y,
            speed,
            direction: Direction::Right,
            behave: Behave::Scatter,
            moving: true,
            ghost_texture_right: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/blue-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_mid: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/blue-ghost-right.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            ghost_texture_left: Box::new(
                Texture::from_path(
                    Path::new("./assets/ghosts/blue-ghost-left.png"), // make these mid sprite
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            target: default_target,
            default_target,
        }
    }
}

impl Ghost for RedGhost {
    fn update(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        if self.moving {
            self.basic_movement();
        }

        match self.behave {
            Behave::Chase => self.new_chase(px, py, dir, _redx, _redy),
            Behave::Frightened => self.new_frightened(),
            Behave::Scatter => self.new_scatter(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gl| {
            Image::new().rect(square(self.x, self.y, ENEMY_SIZE)).draw(
                match self.direction {
                    Left => &*self.ghost_texture_left,
                    Right => &*self.ghost_texture_right,
                    _ => &*self.ghost_texture_mid,
                },
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn direction(&self) -> Direction {
        self.direction
    }
    fn moving(&self) -> bool {
        self.moving
    }
    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_red(&self) -> bool {
        true
    }
}

impl Ghost for PurpleGhost {
    fn update(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        if self.moving {
            self.basic_movement();
        }

        match self.behave {
            Behave::Chase => self.new_chase(px, py, dir, _redx, _redy),
            Behave::Frightened => self.new_frightened(),
            Behave::Scatter => self.new_scatter(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gl| {
            Image::new().rect(square(self.x, self.y, ENEMY_SIZE)).draw(
                match self.direction {
                    Left => &*self.ghost_texture_left,
                    Right => &*self.ghost_texture_right,
                    _ => &*self.ghost_texture_mid,
                },
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn direction(&self) -> Direction {
        self.direction
    }
    fn moving(&self) -> bool {
        self.moving
    }
    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_red(&self) -> bool {
        false
    }
}

impl Ghost for GreenGhost {
    fn update(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        if self.moving {
            self.basic_movement();
        }
        match self.behave {
            Behave::Chase => self.new_chase(px, py, dir, _redx, _redy),
            Behave::Frightened => self.new_frightened(),
            Behave::Scatter => self.new_scatter(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gl| {
            Image::new().rect(square(self.x, self.y, ENEMY_SIZE)).draw(
                match self.direction {
                    Left => &*self.ghost_texture_left,
                    Right => &*self.ghost_texture_right,
                    _ => &*self.ghost_texture_mid,
                },
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn direction(&self) -> Direction {
        self.direction
    }
    fn moving(&self) -> bool {
        self.moving
    }
    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_red(&self) -> bool {
        false
    }
}

impl Ghost for BlueGhost {
    fn update(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        if self.moving {
            self.basic_movement();
        }

        match self.behave {
            Behave::Chase => self.new_chase(px, py, dir, _redx, _redy),
            Behave::Frightened => self.new_frightened(),
            Behave::Scatter => self.new_scatter(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gl| {
            Image::new().rect(square(self.x, self.y, ENEMY_SIZE)).draw(
                match self.direction {
                    Left => &*self.ghost_texture_left,
                    Right => &*self.ghost_texture_right,
                    _ => &*self.ghost_texture_mid,
                },
                &DrawState::default(),
                context.transform,
                gl,
            )
        })
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn direction(&self) -> Direction {
        self.direction
    }
    fn moving(&self) -> bool {
        self.moving
    }
    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_red(&self) -> bool {
        false
    }
}

impl Behavior for RedGhost {
    fn change(&mut self, new: Behave) {
        self.direction.reverse_direction();
        self.behave = new
    }
    fn at_corner(&mut self, corner: &Corner) {
        self.direction = corner.next_dir(
            &self.target.0,
            &self.target.1,
            &self.x,
            &self.y,
            &self.direction,
        )
    }
    fn basic_movement(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
        }
    }
    fn new_chase(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        self.target = (px, py)
    }
    fn new_frightened(&mut self) {}
    fn new_scatter(&mut self) {}
}

impl Behavior for PurpleGhost {
    fn change(&mut self, new: Behave) {
        self.direction.reverse_direction();
        self.behave = new
    }
    fn at_corner(&mut self, corner: &Corner) {}
    fn basic_movement(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
        }
    }
    fn new_chase(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        self.target = match dir {
            &Direction::Left => (px - PLAYER_SIZE * 4., py),
            &Direction::Right => (px + PLAYER_SIZE * 4., py),
            &Direction::Up => (px - PLAYER_SIZE * 4., py - PLAYER_SIZE * 4.),
            &Direction::Down => (px, py + PLAYER_SIZE * 4.),
        }
    }
    fn new_frightened(&mut self) {}
    fn new_scatter(&mut self) {}
}

impl Behavior for GreenGhost {
    fn change(&mut self, new: Behave) {
        self.direction.reverse_direction();
        self.behave = new
    }
    fn at_corner(&mut self, corner: &Corner) {}
    fn basic_movement(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
        }
    }
    fn new_chase(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        if (px + py).sqrt() > PLAYER_SIZE * 8. {
            self.target = (px, py)
        } else {
            self.target = self.default_target
        }
    }
    fn new_frightened(&mut self) {}
    fn new_scatter(&mut self) {}
}
impl Behavior for BlueGhost {
    fn change(&mut self, new: Behave) {
        self.direction.reverse_direction();
        self.behave = new
    }
    fn at_corner(&mut self, corner: &Corner) {}
    fn basic_movement(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Right => self.x += self.speed,
            Direction::Left => self.x -= self.speed,
        }
    }
    fn new_chase(&mut self, px: f64, py: f64, dir: &Direction, _redx: &f64, _redy: &f64) {
        let neutral = match dir {
            &Direction::Left => (px - PLAYER_SIZE * 2., py),
            &Direction::Right => (px + PLAYER_SIZE * 2., py),
            &Direction::Up => (px - PLAYER_SIZE * 2., py - PLAYER_SIZE * 2.),
            &Direction::Down => (px, py + PLAYER_SIZE * 2.),
        };

        self.target = (
            neutral.0 - (_redx - neutral.0),
            neutral.1 - (_redy - neutral.1),
        )
    }
    fn new_frightened(&mut self) {}
    fn new_scatter(&mut self) {}
}

/*
// the aggressive ghost, aims for the player
pub trait Blinky {
    // red, top right
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// this one aims for 4 "squares" in front of the player
pub trait Pinky {
    // purple, top left
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// the flank ghost
// aims for;
// vector from 2 "squares" in front of the player to Blinky rotated 180 deg
pub trait Inky {
    // blue, bottom right
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}

// this ghost is random and does his own thing
pub trait Clyde {
    // our green ghost, stays in bottom left
    fn move_e(&mut self, corner: &Corner, px: f64, py: f64);
}
*/
