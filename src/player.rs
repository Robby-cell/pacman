use std::path::Path;

use crate::game::Moveable;

use graphics::{self, rectangle::square, DrawState, Graphics, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::{input::RenderArgs, Button, Key};

pub const PLAYER_SIZE: f64 = 50_f64;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Stop,
}

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
    x: f64,
    y: f64,
    pub health: i32,
    closed: Box<Texture>,
    half: Box<Texture>,
    open: Box<Texture>,
    travel: f64,
}

impl Moveable for Player {
    fn move_o(&mut self) {
        match self.direction {
            Direction::Up => {
                self.travel += self.speed;
                self.y -= self.speed
            }
            Direction::Right => {
                self.travel += self.speed;
                self.x += self.speed
            }
            Direction::Down => {
                self.travel += self.speed;
                self.y += self.speed
            }
            Direction::Left => {
                self.travel += self.speed;
                self.x -= self.speed
            }

            Direction::Stop => (),
        }
    }
}

impl Player {
    pub fn new(x: f64, y: f64, health: i32) -> Self {
        Player {
            direction: Direction::Stop,
            state: PlayerState::Closed,
            speed: 4.,
            x,
            y,
            health,
            closed: Box::new(
                Texture::from_path(
                    Path::new(".\\assets\\closed.png"), // temporary
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            half: Box::new(
                Texture::from_path(
                    Path::new(".\\assets\\half.png"), // temporary
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            open: Box::new(
                Texture::from_path(
                    Path::new(".\\assets\\open.png"), // temporary
                    &TextureSettings::new(),
                )
                .unwrap(),
            ),
            travel: 0.,
        }
    }

    pub fn update(&mut self) {
        if self.travel < 32. {
            self.state = PlayerState::Closed
        } else if self.travel < 64. {
            self.state = PlayerState::HalfOpen;
        } else if self.travel < 96. {
            self.state = PlayerState::Open;
        } else if self.travel < 120. {
            self.state = PlayerState::HalfOpen;
        } else {
            self.travel = 0.;
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
            Direction::Stop => Direction::Stop,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let x = match self.direction {
            Direction::Right => self.x,                 // original image **
            Direction::Down => self.y,                  //90
            Direction::Left => -(self.x + PLAYER_SIZE), //180
            Direction::Up => -(self.y + PLAYER_SIZE),   //270
            Direction::Stop => self.x,
        };
        let y = match self.direction {
            Direction::Right => self.y,
            Direction::Down => -(self.x + PLAYER_SIZE),
            Direction::Left => -(self.y + PLAYER_SIZE),
            Direction::Up => self.x,
            Direction::Stop => self.y,
        };

        gl.draw(args.viewport(), |context, gl| {
            let transform = context.transform.rot_deg(match self.direction {
                Direction::Right => 0.,
                Direction::Down => 90.,
                Direction::Left => 180.,
                Direction::Up => -90.,
                Direction::Stop => 0.,
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
            &Button::Keyboard(Key::W) => self.direction = Direction::Up,
            &Button::Keyboard(Key::D) => self.direction = Direction::Right,
            &Button::Keyboard(Key::S) => self.direction = Direction::Down,
            &Button::Keyboard(Key::A) => self.direction = Direction::Left,

            _ => (),
        }
    }
}
