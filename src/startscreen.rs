use opengl_graphics::{GlGraphics, OpenGL};

pub struct StartScreen {
    pub gl: GlGraphics,
}

impl StartScreen {
    pub fn new(opengl: OpenGL) -> Self {
        Self {
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn render(&self) {
        println!("L")
    }
}
