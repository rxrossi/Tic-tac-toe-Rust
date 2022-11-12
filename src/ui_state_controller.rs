use crate::ui::{BoardState, Mark};

pub struct UiBoardStateController {
    state: [[Option<Mark>; 3]; 3],
}

impl UiBoardStateController {
    pub fn new() -> UiBoardStateController {
        UiBoardStateController {
            state: [
                [Some(Mark::RED), None, None],
                [Some(Mark::BLUE), None, None],
                [Some(Mark::RED), None, Some(Mark::BLUE)],
            ],
        }
    }
}

impl BoardState for UiBoardStateController {
    fn get_grid_spaces(&self) -> [[Option<Mark>; 3]; 3] {
        self.state.clone()
    }

    fn set_grid_space(&mut self, x: usize, y: usize) -> () {
        self.state[x][y] = Some(Mark::BLUE);
    }
}
