mod enemy;
mod game;
mod player;
mod points;

use game::Game;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::*,
    window::WindowSettings,
    EventLoop,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opengl = OpenGL::V4_2;

    let mut window: GlutinWindow = WindowSettings::new("Pacman!", (1500, 800))
        .exit_on_esc(true)
        .build()
        .ok()
        .ok_or("Could not create Glutin Window")?;

    let mut game = Box::new(Game::new(GlGraphics::new(opengl), 500., 500., 3));
    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
            game.update();
        }

        if let Some(k) = e.button_args() {
            game.button_pressed(&k.button);
        }
    }

    Ok(())
}
