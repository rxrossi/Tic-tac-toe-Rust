use crate::board::{Board, Mark};

pub struct Score {
    pub player_1: usize,
    pub player_2: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    Player1,
    Player2,
}

pub struct Game {
    room: String,
    turn_of: Player,
    pub score: Score,
    who_am_i: Player,
    board: Board,
}

fn player_to_mark(player: &Player) -> Mark {
    match player {
        Player::Player1 => Mark::X,
        Player::Player2 => Mark::O,
    }
}

fn mark_to_player(mark: &Mark) -> Player {
    match mark {
        Mark::X => Player::Player1,
        Mark::O => Player::Player2,
    }
}

impl Game {
    pub fn new(as_player: Player) -> Game {
        Game {
            room: "TODO".to_string(),
            turn_of: Player::Player1,
            score: Score {
                player_1: 0,
                player_2: 0,
            },
            who_am_i: as_player,
            board: Board::new(),
        }
    }

    fn the_other_player(&self) -> Player {
        match self.who_am_i {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn add_mark_at(&mut self, coords: [usize; 2]) {
        self.board.set_mark(coords, player_to_mark(&self.who_am_i));

        match self.has_anyone_won() {
            None => self.turn_of = self.the_other_player(),
            Some(player) => self.on_win(player),
        }
    }

    /// Meant to be used by the the IO client to inform about how the other play has played
    pub fn add_mark_as_the_other_player(&mut self, coords: [usize; 2]) {
        self.board.set_mark(coords, crate::board::Mark::O);

        match self.has_anyone_won() {
            None => {
                self.turn_of = self.who_am_i.clone();
            }
            Some(player) => self.on_win(player),
        }
    }

    pub fn is_my_turn(&self) -> bool {
        if self.has_anyone_won() == None {
            self.turn_of == self.who_am_i
        } else {
            false
        }
    }

    pub fn has_anyone_won(&self) -> Option<Player> {
        match self.board.has_winner() {
            None => None,
            Some(mark) => Some(mark_to_player(&mark)),
        }
    }

    fn on_win(&mut self, winner: Player) {
        match winner {
            Player::Player1 => self.score.player_1 += 1,
            Player::Player2 => self.score.player_2 += 1,
        };
    }
}

#[cfg(test)]
mod player_turn {
    use super::*;

    #[test]
    fn can_play_if_it_is_my_turn() {
        let mut game = Game::new(Player::Player1);

        assert_eq!(game.is_my_turn(), true);

        game.add_mark_at([0, 0]);

        assert_eq!(game.is_my_turn(), false);

        game.add_mark_as_the_other_player([1, 1]);

        assert_eq!(game.is_my_turn(), true);
    }
}
