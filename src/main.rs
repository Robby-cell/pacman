mod builder;
mod corner;
mod enemy;
mod game;
mod map;
mod player;
mod point;
mod startscreen;
mod utilities;

use game::Game;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::*,
    window::WindowSettings,
    EventLoop,
};
use point::Point;
use startscreen::StartScreen;
use utilities::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opengl = OpenGL::V4_2;
    let mut state = State::StartScreen;

    let mut window: GlutinWindow = WindowSettings::new("Pacman!", (SCREEN_WIDTH, SCREEN_HEIGHT))
        .exit_on_esc(true)
        .build()
        .ok()
        .ok_or("Could not create Window")?;

    let mut game = Box::new(Game::<Point>::new(GlGraphics::new(opengl), 500., 500., 3));
    let mut events = Events::new(EventSettings::new()).ups(60);
    let mut startscreen = Some(StartScreen::new(OpenGL::V4_2));

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            match state {
                State::StartScreen => {
                    game.render(&r);
                    ()
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
                State::StartScreen => match &k.button {
                    &Button::Mouse(_) => (),
                    _ => {
                        state = {
                            startscreen = None;
                            State::Play
                        }
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
