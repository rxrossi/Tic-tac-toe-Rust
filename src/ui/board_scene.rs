use opengl_graphics::GlGraphics;
use piston::RenderArgs;


use self::board::Board;

use super::{HandleEvent, Render};

mod board;

pub struct BoardScene {
    mouse_coords: [f64; 2],
    board: Board,
}

impl BoardScene {
    pub fn new() -> BoardScene {
        BoardScene {
            mouse_coords: [0., 0.],
            board: Board::new(),
        }
    }
}

impl Render for BoardScene {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl.draw(args.viewport(), |_ctx, gl| {
            const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            clear(BACKGROUND_COLOR, gl);
        });

        self.board.render(gl, args);
    }
}

impl HandleEvent for BoardScene {
    fn handle_event<E: piston::GenericEvent>(&mut self, e: E, gl: &mut GlGraphics) -> () {
        // use piston::mouse::*;

        if let Some(args) = e.render_args() {
            self.render(gl, &args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.mouse_coords = pos;
        }

        // if let Some(button) = e.press_args() {
        //     if button == Button::Mouse(MouseButton::Left) {
        //         self.handle_left_mouse_click(self.mouse_coords);
        //     }
        // }
    }
}
