use tic_tac_toe_3::{
    game::{Game, Player},
    ui::ui,
};

fn main() {
    let mut game = Game::new(Player::Player1);
    ui(&mut game);
}
