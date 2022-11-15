use graphics::{text, DrawState, Transformed, color::hex};
use opengl_graphics::{GlyphCache, TextureSettings};

use super::Score;

pub struct ScoreUi;

impl ScoreUi {
    pub fn render(&mut self, gl: &mut opengl_graphics::GlGraphics, args: &piston::RenderArgs, score: Score) -> () {
        let mut glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

        let orange = hex("FFB865");

        gl.draw(args.viewport(), |ctx, gl| {

            text::Text::new_color([1., 1., 1., 1.], 24)
                .draw(
                    "VS",
                    &mut glyph_cache,
                    &DrawState::default(),
                    ctx.transform.trans(234., 34.),
                    gl,
                )
                .unwrap();

            text::Text::new_color(orange, 24)
                .draw(
                    &format!("X: {}", score.player_1).to_string(),
                    &mut glyph_cache,
                    &DrawState::default(),
                    ctx.transform.trans(50., 34.),
                    gl,
                )
                .unwrap();

            text::Text::new_color(orange, 24)
                .draw(
                    &format!("O: {}", score.player_2).to_string(),
                    &mut glyph_cache,
                    &DrawState::default(),
                    ctx.transform.trans(398., 34.),
                    gl,
                )
                .unwrap();
        });
    }
}
