use tic_tac_toe_3::{ui::ui, ui_state_controller::UiBoardStateController};

fn main() {
    let board_state_controller = Box::new(UiBoardStateController::new());

    ui(board_state_controller);
}
