use std::fmt::Display;

use crate::strategy::game_strategy::GameStrategy;

#[derive(Debug, Clone)]
pub struct TicTacToe {
    pub board: Vec<char>,
    pub size: usize,
    pub default_char: char,
    pub maximizer: char,
    pub minimizer: char,
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in 0..self.size {
            let start = self.size * idx;
            let end = self.size * (idx + 1);
            let sub: &[char] = &self.board[start as usize..end as usize];

            for &x in sub.iter() {
                write!(f, "{}", x)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe::new(3)
    }
}

/// Implements all necessary
/// methods to operate a TicTacToe
/// game.
impl TicTacToe {

    pub fn new(size: usize) -> Self {
        let board: Vec<char> = vec!['-'; (size * size) as usize];
        Self {
            board,
            size,
            default_char: '-',
            maximizer: 'o',
            minimizer: 'x'
        }
    }

    pub fn with_player_1(self, character: char) -> Self {
        Self {
            maximizer: character,
            ..self
        }
    }
    pub fn with_player_2(self, character: char) -> Self {
        Self {
            minimizer: character,
            ..self
        }
    }
    pub fn with_default_char(self, character: char) -> Self {
        Self {
            default_char: character,
            ..self
        }
    }

    /// Check the main and anti-diagonals
    /// for a winner.
    pub fn check_diagonals(&self) -> char {
        let mut winner = self.default_char;
        if self.check_diagonal(self.maximizer, true)
            || self.check_diagonal(self.maximizer, false)
        {
            winner = self.maximizer
        } else if self.check_diagonal(self.minimizer, true)
            || self.check_diagonal(self.minimizer, false)
        {
            winner = self.minimizer
        }
        winner
    }

    /// Check the rows of the grid for a winner.
    pub fn check_rows(&self) -> char {
        let mut winner = self.default_char;

        for row in 0..self.size as usize {
            if self.check_row(self.maximizer, row) {
                winner = self.maximizer;
                break;
            } else if self.check_row(self.minimizer, row) {
                winner = self.minimizer;
                break;
            }
        }
        winner
    }

    /// Check the columns of the grid for a winner.
    pub fn check_cols(&self) -> char {
        let mut winner = self.default_char;

        for col in 0..self.size as usize {
            if self.check_col(self.maximizer, col) {
                winner = self.maximizer;
                break;
            } else if self.check_col(self.minimizer, col) {
                winner = self.minimizer;
                break;
            }
        }
        winner
    }

    /// Check a given column if a given player has won.
    fn check_col(&self, ch: char, col_num: usize) -> bool {
        for row in 0..self.size as usize {
            if self.board[self.size as usize * row + col_num] != ch {
                return false;
            }
        }
        true
    }

    /// Check a given row if a given player has won.
    fn check_row(&self, ch: char, row_num: usize) -> bool {
        for col in 0..self.size as usize {
            if self.board[self.size as usize * row_num + col] != ch {
                return false;
            }
        }
        true
    }

    /// Check the main and anti diagonals if a
    /// given player has won.
    fn check_diagonal(&self, ch: char, diag: bool) -> bool {
        // main diagonal is represented by true.
        if diag {
            for idx in 0..self.size as usize {
                if self.board[(self.size as usize * idx as usize) + idx] != ch {
                    return false;
                }
            }
            true
        } else {
            for idx in 0..self.size as usize {
                if self.board
                    [(self.size as usize * (self.size as usize - 1 - idx as usize)) + idx]
                    != ch
                {
                    return false;
                }
            }
            true
        }
    }
}

/// Endow upon TicTacToe the ability to
/// play games.
impl GameStrategy for TicTacToe {
    /// The Player is a char.
    /// Usually one of 'o', 'O', 'x', 'X', '-'.
    type Player = char;

    /// The Move is a single number representing an
    /// index of the Board vector, i.e. in range
    /// `[0, (size * size) - 1]`.
    type Move = usize;

    /// The Board is a single vector of length `size * size`.
    type Board = Vec<char>;

    fn evaluate(&self) -> f64 {
        if self.is_game_tied() {
            0.
        } else {
            let _winner = self.get_winner().unwrap();
            if _winner == self.maximizer {
                1000.
            } else {
                -1000.
            }
        }
    }

    fn get_winner(&self) -> Option<Self::Player> {
        let mut winner = self.check_diagonals();

        if winner == self.default_char {
            winner = self.check_rows();
        }
        if winner == self.default_char {
            winner = self.check_cols();
        }
        Some(winner)
    }

    fn is_game_tied(&self) -> bool {
        let _winner = self.get_winner().unwrap();

        _winner == self.default_char && self.get_available_moves().is_empty()
    }

    fn is_game_complete(&self) -> bool {
        let _winner = self.get_winner();

        self.get_available_moves().is_empty() || _winner.unwrap() != '-'
    }

    fn get_available_moves(&self) -> Vec<Self::Move> {
        let mut moves: Vec<usize> = vec![];
        for idx in 0..(self.size * self.size) as usize {
            if self.board[idx] == '-' {
                moves.push(idx)
            }
        }
        moves
    }

    fn play(&mut self, &mv: &Self::Move, maximizer: bool) {
        // player: true means the maximizer's turn.

        if maximizer {
            self.board[mv] = self.maximizer;
        } else {
            self.board[mv] = self.minimizer;
        }
    }

    fn clear(&mut self, &mv: &Self::Move) {
        self.board[mv] = self.default_char
    }

    fn get_board(&self) -> &Self::Board {
        &self.board
    }

    fn is_a_valid_move(&self, &mv: &Self::Move) -> bool {
        self.board[mv] == self.default_char
    }

    fn get_a_sentinel_move(&self) -> Self::Move {
        self.size * self.size + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::alpha_beta_minimax::AlphaBetaMiniMaxStrategy;

    #[test]
    fn best_move_in_given_3_by_3() {
        let mut ttt = 
            TicTacToe::new(3)
            .with_player_1('o')
            .with_player_2('x')
            .with_default_char('-');
    
        ttt.play(&8, true);
        ttt.play(&7, false);
        ttt.play(&5, true);

        assert_eq!(ttt.get_best_move(9, false), 2);
    }

    #[test]
    fn test_should_always_tie_a_3_by_3_after_9_moves_at_depth_9() {
        let mut ttt = TicTacToe::new(3);
        for move_number in 0..=8 {
            let is_maximising = move_number%2 == 0;
            let i = ttt.get_best_move(9, is_maximising);
            ttt.play(&i, is_maximising);
            println!("{}", ttt);
            // ttt.print_board();
        }
        assert!(ttt.is_game_complete());
        assert!(ttt.is_game_tied());
    }
}