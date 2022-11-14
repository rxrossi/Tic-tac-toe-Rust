use graphics::color::hex;
use opengl_graphics::GlGraphics;
use piston::{Button, MouseButton, RenderArgs};

use self::board::{Board, GameState};

use super::{HandleEvent, Render};

pub mod board;
pub mod score;

pub struct BoardScene<'a> {
    mouse_coords: [f64; 2],
    board: Board,
    game_state_controller: Box<dyn GameState + 'a>, // TODO: remove pub
}

impl BoardScene<'_> {
    pub fn new<'a>(game_state_controller: Box<dyn GameState + 'a>) -> BoardScene<'a> {
        BoardScene {
            mouse_coords: [0., 0.],
            board: Board::new(),
            game_state_controller,
        }
    }

    pub fn ended(&self) -> bool {
        self.game_state_controller.has_game_finished()
    }
}

impl Render for BoardScene<'_> {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl.draw(args.viewport(), |_ctx, gl| {
            clear(hex("032E4E"), gl);
        });

        self.board.render(gl, args, self.game_state_controller.get_grid_spaces());

        // I need to execute the render method of Score here. To be able to do that,
        // I need the state info about the score, but the `Board` owns it. It owns it
        // because it needs the grid space click function and information about the
        // marks placed on the grids. If I could pass down that information instead of the state
        // controller, I could fix it
        //
        // 1. BoardScene should own the board_state_controller, which could be named
        //    game_state_controller instead
        // 2. Board render should take the grid spaces info
        // 3. Board handle_left_mouse_click should take a closure that can change the data
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
                let on_grid_space_click =
                    |(x, y)| self.game_state_controller.on_grid_space_click(x, y); //TODO: inverted here

                self.board
                    .handle_left_mouse_click(self.mouse_coords, on_grid_space_click);
            }
        }
    }
}
