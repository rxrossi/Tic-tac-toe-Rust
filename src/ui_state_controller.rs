use crate::game::Game;
use crate::ui::{self, Score};
use crate::board;

pub struct UiBoardStateController<'a> {
    pub game: &'a mut Game,
}

impl UiBoardStateController<'_> {
    pub fn new(game: &mut Game) -> UiBoardStateController {
        UiBoardStateController { game }
    }

    fn game_logic_to_ui_grid_space(&self, coords: [usize; 2]) -> Option<ui::Mark> {
        match self.game.board.grid_spaces[coords[0]][coords[1]] {
            Some(board::Mark::X) => Some(ui::Mark::RED),
            Some(board::Mark::O) => Some(ui::Mark::BLUE),
            None => None,
        }
    }
}

impl ui::GameState for UiBoardStateController<'_> {
    fn get_score(&self) -> Score  {
        Score {
            player_1: self.game.score.player_1,
            player_2: self.game.score.player_2,
        }
    }

    fn get_grid_spaces(&self) -> [[Option<ui::Mark>; 3]; 3] {
        [
            [
                self.game_logic_to_ui_grid_space([0, 0]),
                self.game_logic_to_ui_grid_space([1, 0]),
                self.game_logic_to_ui_grid_space([2, 0]),
            ],
            [
                self.game_logic_to_ui_grid_space([0, 1]),
                self.game_logic_to_ui_grid_space([1, 1]),
                self.game_logic_to_ui_grid_space([2, 1]),
            ],
            [
                self.game_logic_to_ui_grid_space([0, 2]),
                self.game_logic_to_ui_grid_space([1, 2]),
                self.game_logic_to_ui_grid_space([2, 2]),
            ],
        ]
    }

    fn on_grid_space_click(&mut self, x: usize, y: usize) -> () {
        if self.game.has_anyone_won() != None {
        } else if self.game.is_my_turn() {
            self.game.add_mark_at([y, x]); //TODO: has been inverted
        } else {
            self.game.add_mark_as_the_other_player([y, x])
        }
    }

    fn has_game_finished(&self) -> bool {
        self.game.has_anyone_won() != None
    }
}
