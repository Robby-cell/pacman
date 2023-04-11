use std::path::Path;

use crate::game::Moveable;
use crate::utilities::*;

use graphics::{self, rectangle::square, DrawState, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::{input::RenderArgs, Button, Key};

pub enum PlayerState {
    Closed,
    HalfOpen,
    Open,
}

/*
impl PlayerState {
    pub fn update(&mut self) {
        *self = match self {
            Self::Closed => Self::HalfOpen,
            Self::HalfOpen => Self::Open,
            Self::Open => Self::Closed,
        }
    }
}
*/

pub struct Player {
    pub direction: Direction,
    pub state: PlayerState,
    speed: f64,
    pub x: f64,
    pub y: f64,
    pub health: i32,
    closed: Box<Texture>,
    half: Box<Texture>,
    open: Box<Texture>,
    travel: u8,
    pub moving: bool,
}

impl Moveable for Player {
    fn move_o(&mut self) {
        if (self.y <= 0. && self.direction == Direction::Up)
            || (self.x <= 0. && self.direction == Direction::Left)
            || (self.y >= SCREEN_HEIGHT - PLAYER_SIZE && self.direction == Direction::Down)
            || (self.x >= SCREEN_WIDTH - PLAYER_SIZE && self.direction == Direction::Right)
        {
            self.travel = 4;
            self.moving = false;
        } else if self.moving {
            self.travel += 1;
            match self.direction {
                Direction::Up => self.y -= self.speed,
                Direction::Right => self.x += self.speed,
                Direction::Down => self.y += self.speed,
                Direction::Left => self.x -= self.speed,
            }
        }
    }
}

impl Player {
    pub fn new(x: f64, y: f64, health: i32) -> Self {
        Player {
            direction: Direction::Up,
            state: PlayerState::Closed,
            speed: 4.,
            x,
            y,
            health,
            closed: Box::new(
                Texture::from_path(Path::new(".\\assets\\closed.png"), &TextureSettings::new())
                    .unwrap(),
            ),
            half: Box::new(
                Texture::from_path(Path::new(".\\assets\\half.png"), &TextureSettings::new())
                    .unwrap(),
            ),
            open: Box::new(
                Texture::from_path(Path::new(".\\assets\\open.png"), &TextureSettings::new())
                    .unwrap(),
            ),
            travel: 0,
            moving: false,
        }
    }

    pub fn update(&mut self) {
        if self.travel < 8 {
            self.state = PlayerState::Closed
        } else if self.travel < 16 {
            self.state = PlayerState::HalfOpen;
        } else if self.travel < 24 {
            self.state = PlayerState::Open;
        } else if self.travel < 30 {
            self.state = PlayerState::HalfOpen;
        } else {
            self.travel = 0;
        }
        self.move_o();
    }

    // probably not going to be used for now, but feature is here for now
    #[allow(unused)]
    pub fn reverse_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let x = match self.direction {
            Direction::Right => self.x,                 // original image **
            Direction::Down => self.y,                  //90
            Direction::Left => -(self.x + PLAYER_SIZE), //180
            Direction::Up => -(self.y + PLAYER_SIZE),   //270
        };
        let y = match self.direction {
            Direction::Right => self.y,
            Direction::Down => -(self.x + PLAYER_SIZE),
            Direction::Left => -(self.y + PLAYER_SIZE),
            Direction::Up => self.x,
        };

        gl.draw(args.viewport(), |context, gl| {
            let transform = context.transform.rot_deg(match self.direction {
                Direction::Right => 0.,
                Direction::Down => 90.,
                Direction::Left => 180.,
                Direction::Up => -90.,
            });
            Image::new().rect(square(x, y, PLAYER_SIZE)).draw(
                match self.state {
                    PlayerState::Closed => &*self.closed,
                    PlayerState::HalfOpen => &*self.half,
                    PlayerState::Open => &*self.open,
                },
                &DrawState::default(),
                transform,
                gl,
            )
        })
    }

    pub fn change_direction(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::W) => {
                self.moving = true;
                self.direction = Direction::Up
            }
            &Button::Keyboard(Key::D) => {
                self.moving = true;
                self.direction = Direction::Right
            }
            &Button::Keyboard(Key::S) => {
                self.moving = true;
                self.direction = Direction::Down
            }
            &Button::Keyboard(Key::A) => {
                self.moving = true;
                self.direction = Direction::Left
            }

            &Button::Keyboard(Key::Space) => {
                self.travel = 4;
                self.moving = false
            }

            _ => (),
        }
    }
}
