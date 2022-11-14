use graphics::{clear, text, DrawState, Transformed};
use opengl_graphics::{GlyphCache, TextureSettings};

use crate::ui::Render;

struct Score;

impl Render for Score {
    fn render(&mut self, gl: &mut opengl_graphics::GlGraphics, args: &piston::RenderArgs) -> () {
        let mut glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

        gl.draw(args.viewport(), |ctx, gl| {
            const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            clear(BACKGROUND_COLOR, gl);

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                .draw(
                    &format!("Player X: {}", 1).to_string(),
                    &mut glyph_cache,
                    &DrawState::default(),
                    ctx.transform.trans(40., 100.),
                    gl,
                )
                .unwrap();
        });
    }
}
