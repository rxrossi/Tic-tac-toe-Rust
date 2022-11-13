use opengl_graphics::GlGraphics;
use piston::{Button, MouseButton, RenderArgs};

use self::board::{Board, BoardState};

use super::{HandleEvent, Render};

pub mod board;

pub struct BoardScene<'a> {
    mouse_coords: [f64; 2],
    board: Board<'a>,
}

impl BoardScene<'_> {
    pub fn new<'a>(board_state_controller: Box<dyn BoardState + 'a>) -> BoardScene<'a> {
        BoardScene {
            mouse_coords: [0., 0.],
            board: Board::new(board_state_controller),
        }
    }

    pub fn ended(&self) -> bool {
        self.board.has_game_finished()
    }
}

impl Render for BoardScene<'_> {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl.draw(args.viewport(), |_ctx, gl| {
            const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            clear(BACKGROUND_COLOR, gl);
        });

        self.board.render(gl, args);
    }
}

impl HandleEvent for BoardScene<'_> {
    fn handle_event<E: piston::GenericEvent>(&mut self, e: E, gl: &mut GlGraphics) -> () {

        if let Some(args) = e.render_args() {
            self.render(gl, &args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.mouse_coords = pos;
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                self.board.handle_left_mouse_click(self.mouse_coords);
            }
        }
    }
}
