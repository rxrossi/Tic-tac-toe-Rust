use tic_tac_toe_3::game::{Game, Player};


#[test]
fn loses_as_player1() {
    let mut game = Game::new(Player::Player1);

    assert_eq!(game.has_anyone_won(), None);
    assert_eq!(game.score.player_1, 0);
    assert_eq!(game.score.player_2, 0);

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([0,0]);
    // X - -
    // - - -
    // - - -

    assert_eq!(game.is_my_turn(), false);
    game.add_mark_as_the_other_player([1,1]);
    // X - -
    // - O -
    // - - -

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([2,0]);
    // X - X
    // - O -
    // - - -

    assert_eq!(game.is_my_turn(), false);
    game.add_mark_as_the_other_player([1,0]);
    // X O X
    // - O -
    // - - -

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([0,1]);
    // X O X
    // X O -
    // - - -

    assert_eq!(game.is_my_turn(), false);
    game.add_mark_as_the_other_player([1,2]);
    // X O X
    // - O -
    // - O -

    assert_eq!(game.score.player_1, 0);
    assert_eq!(game.score.player_2, 1);
}

#[test]
fn wins_as_player1() {
    let mut game = Game::new(Player::Player1);

    assert_eq!(game.has_anyone_won(), None);
    assert_eq!(game.score.player_1, 0);
    assert_eq!(game.score.player_2, 0);

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([0,0]);
    // X - -
    // - - -
    // - - -

    assert_eq!(game.is_my_turn(), false);
    game.add_mark_as_the_other_player([1,1]);
    // X - -
    // - O -
    // - - -

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([2,0]);
    // X - X
    // - O -
    // - - -

    assert_eq!(game.is_my_turn(), false);
    game.add_mark_as_the_other_player([0,2]);
    // X - X
    // - O -
    // O - -

    assert_eq!(game.is_my_turn(), true);
    game.add_mark_at([1,0]);
    // X X X
    // - O -
    // O - -

    assert_eq!(game.score.player_1, 1);
    assert_eq!(game.score.player_2, 0);
}
