mod enemy;
mod game;
mod map;
mod player;
mod points;
mod utilities;

use game::Game;
use glutin_window::GlutinWindow;
use map::WallType;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::*,
    window::WindowSettings,
    EventLoop,
};
use utilities::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opengl = OpenGL::V4_2;
    let mut state = State::Play;

    let mut window: GlutinWindow = WindowSettings::new("Pacman!", (SCREEN_WIDTH, SCREEN_HEIGHT))
        .exit_on_esc(true)
        .build()
        .ok()
        .ok_or("Could not create Glutin Window")?;

    let mut game = Box::new(Game::new(GlGraphics::new(opengl), 500., 500., 3));
    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            match state {
                State::StartScreen => {
                    todo!("add start screen.");
                    () // do something. will do later
                }
                State::Play => {
                    game.render(&r);
                    game.update();
                }
                State::Menu => {
                    todo!("add menu state")
                }
            }
        }

        if let Some(k) = e.button_args() {
            match state {
                State::StartScreen => {
                    todo!("add start screen and implementations");
                    ()
                }
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
