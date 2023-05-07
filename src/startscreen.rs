use graphics::{Graphics, Transformed};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::RenderArgs;

use crate::utilities::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct StartScreen {
    pub gl: GlGraphics,
}

impl StartScreen {
    pub fn new(opengl: OpenGL) -> Self {
        Self {
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let texture_settings: TextureSettings = TextureSettings::new().filter(Filter::Nearest);
        let ref mut glyphs =
            GlyphCache::new("./assets/fonts/RoadPixel.ttf", (), texture_settings).unwrap();
        self.gl.draw(args.viewport(), |c, gl| {
            let font_size = 30;
            let text = "Press any button to start";
            graphics::text(
                [1., 0., 0., 1.],
                font_size,
                text,
                glyphs,
                c.transform.trans(
                    (SCREEN_WIDTH - font_size as f64 * text.len() as f64) / 2.,
                    SCREEN_HEIGHT / 2.,
                ),
                gl,
            )
            .unwrap();
        })
    }
}
