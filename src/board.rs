use std::fmt::Debug;

#[derive(Debug)]
pub struct Board {
    grid_spaces: [[Option<Mark>; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid_spaces: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn set_mark(&mut self, coords: [usize; 2], mark: Mark) {
        if self.is_space_empty(coords) {
            self.grid_spaces[coords[0]][coords[1]] = Some(mark);
        }
    }

    pub fn is_space_empty(&self, coords: [usize; 2]) -> bool {
        self.grid_spaces[coords[0]][coords[1]] == None
    }

    pub fn has_winner(&self) -> Option<Mark> {
        for row in 0..3 {
            if !self.is_space_empty([0, row])
                && all_equal(vec![
                    self.grid_spaces[0][row].clone(),
                    self.grid_spaces[1][row].clone(),
                    self.grid_spaces[2][row].clone(),
                ])
            {
                return self.grid_spaces[0][row].clone();
            }
        }

        for col in 0..3 {
            if !self.is_space_empty([col, 0])
                && all_equal(vec![
                    self.grid_spaces[col][0].clone(),
                    self.grid_spaces[col][1].clone(),
                    self.grid_spaces[col][2].clone(),
                ])
            {
                return self.grid_spaces[col][0].clone();
            }
        }

        if !self.is_space_empty([1, 1])
            && (all_equal(vec![
                self.grid_spaces[0][0].clone(),
                self.grid_spaces[1][1].clone(),
                self.grid_spaces[2][2].clone(),
            ]) || (all_equal(vec![
                self.grid_spaces[2][0].clone(),
                self.grid_spaces[1][1].clone(),
                self.grid_spaces[0][2].clone(),
            ])))
        {
            return self.grid_spaces[1][1].clone();
        }

        None
    }
}

#[cfg(test)]
mod test_board_has_winner {
    use super::*;

    #[test]
    fn with_empty_board() {
        let board = Board::new();
        assert_eq!(board.has_winner(), None);
    }

    mod test_win_on_row {
        use super::*;

        #[test]
        fn with_first_row_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([0, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 0], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn with_second_row_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([0, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 1], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn with_third_row_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([0, 2], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 2], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 2], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn tie_on_third_row() {
            let mut board = Board::new();
            board.set_mark([0, 2], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 2], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 2], Mark::O);
            assert_eq!(board.has_winner(), None);
        }
    }

    mod win_on_col {
        use super::*;

        #[test]
        fn with_first_col_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([0, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([0, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([0, 2], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn with_second_col_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([1, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 2], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn with_third_col_filled_with_x() {
            let mut board = Board::new();
            board.set_mark([2, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 2], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn with_tie_on_middle_col() {
            let mut board = Board::new();
            board.set_mark([1, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 2], Mark::X);
            assert_eq!(board.has_winner(), None);
        }
    }

    mod win_on_diagonal {
        use super::*;

        #[test]
        fn win_on_first_diagonal() {
            let mut board = Board::new();
            board.set_mark([0, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 2], Mark::X);
            assert_eq!(board.has_winner(), Some(Mark::X));
        }

        #[test]
        fn win_on_second_diagonal() {
            let mut board = Board::new();
            board.set_mark([2, 0], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([0, 2], Mark::O);
            assert_eq!(board.has_winner(), Some(Mark::O));
        }


        #[test]
        fn tie_on_first_diagonal() {
            let mut board = Board::new();
            board.set_mark([0, 0], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([2, 2], Mark::X);
            assert_eq!(board.has_winner(), None);
        }

        #[test]
        fn tie_on_second_diagonal() {
            let mut board = Board::new();
            board.set_mark([2, 0], Mark::O);
            assert_eq!(board.has_winner(), None);
            board.set_mark([1, 1], Mark::X);
            assert_eq!(board.has_winner(), None);
            board.set_mark([0, 2], Mark::O);
            assert_eq!(board.has_winner(), None);
        }
    }
}

// TODO: move this out
fn all_equal<T>(vec: Vec<T>) -> bool
where
    T: PartialEq<T> + Debug,
{
    vec.iter().all(|it| *it == vec[0])
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mark {
    X,
    O,
}

#[cfg(test)]
mod test_marking_a_space {

    use super::*;

    #[test]
    fn test_all_equal() {
        let v = vec!["a", "a", "a"];
        assert!(all_equal(v));

        let v2 = vec!["c", "a", "c"];
        assert_eq!(all_equal(v2), false);
    }

    #[test]
    fn starts_with_empty_board() {
        let board = Board::new();

        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(board.grid_spaces[i][j], None);
            }
        }
    }

    #[test]
    fn can_mark_a_empty_space() {
        let mut board = Board::new();

        assert_eq!(board.grid_spaces[0][1], None);

        board.set_mark([0, 1], Mark::X);

        assert_eq!(board.grid_spaces[0][1], Some(Mark::X));
    }

    #[test]
    fn cant_replace_a_mark() {
        let mut board = Board::new();

        board.set_mark([0, 1], Mark::O);

        assert_eq!(board.grid_spaces[0][1], Some(Mark::O));

        board.set_mark([0, 1], Mark::X);

        assert_eq!(board.grid_spaces[0][1], Some(Mark::O));
    }
}
