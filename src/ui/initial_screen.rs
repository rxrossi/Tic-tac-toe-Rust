use super::{HandleEvent, Render};
use graphics::rectangle::square;
use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

pub struct InitialScreen {
    mouse_coords: [f64; 2],
}

impl InitialScreen {
    pub fn new() -> InitialScreen {
        InitialScreen {
            mouse_coords: [0.0, 0.0],
        }
    }
}

impl Render for InitialScreen {
    fn render(&mut self, gl: &mut opengl_graphics::GlGraphics, args: &piston::RenderArgs) -> () {
        use graphics::*;

        let image = Image::new().rect(square(0.0, 0.0, 501.0));
        let settings = TextureSettings::new();
        let texture = Texture::from_path(Path::new("assets/init-bg.png"), &settings).unwrap();

        gl.draw(args.viewport(), |ctx, gl| {
            const BACKGROUND_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            clear(BACKGROUND_COLOR, gl);

            image.draw(&texture, &DrawState::default(), ctx.transform, gl);
        });
    }
}

impl HandleEvent for InitialScreen {
    fn handle_event<E: piston::GenericEvent>(
        &mut self,
        e: E,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> () {
        if let Some(args) = e.render_args() {
            self.render(gl, &args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.mouse_coords = pos;
        }
    }
}
