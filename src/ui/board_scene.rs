use graphics::color::hex;
use opengl_graphics::GlGraphics;
use piston::{Button, MouseButton, RenderArgs};

use self::{board::Board, score::ScoreUi};

use super::{HandleEvent, Mark, Render};

pub mod board;
pub mod score;

pub struct Score {
    pub player_1: usize,
    pub player_2: usize,
}

impl Score {}

//TODO: Move this
pub trait GameState {
    fn get_grid_spaces(&self) -> [[Option<Mark>; 3]; 3];
    fn on_grid_space_click(&mut self, x: usize, y: usize) -> ();
    fn get_score(&self) -> Score;
    fn has_game_finished(&self) -> bool;
}

pub struct BoardScene<'a> {
    mouse_coords: [f64; 2],
    board: Board,
    score: ScoreUi,
    game_state_controller: Box<dyn GameState + 'a>, // TODO: remove pub
}

impl BoardScene<'_> {
    pub fn new<'a>(game_state_controller: Box<dyn GameState + 'a>) -> BoardScene<'a> {
        BoardScene {
            mouse_coords: [0., 0.],
            score: ScoreUi,
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

        self.board
            .render(gl, args, self.game_state_controller.get_grid_spaces());

        self.score.render(gl, args, self.game_state_controller.get_score());
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
