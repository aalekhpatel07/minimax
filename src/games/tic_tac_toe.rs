use crate::strategy::game_strategy::GameStrategy;

pub struct TicTacToe {
    pub board: Vec<char>,
    pub size: usize,
    pub default_char: char,
    pub maximizer: char,
    pub minimizer: char,
}

/// Implements all necessary
/// methods to operate a TicTacToe
/// game.
impl TicTacToe {
    /// Create a new game of TicTacToe
    /// with a fresh board.
    pub fn create_game(
        size: usize,
        default_char: Option<char>,
        maximizer: Option<char>,
        minimizer: Option<char>,
    ) -> TicTacToe {
        let board: Vec<char> = vec![default_char.unwrap_or('-'); (size * size) as usize];

        TicTacToe {
            size,
            board,
            maximizer: maximizer.unwrap_or('o'),
            minimizer: minimizer.unwrap_or('x'),
            default_char: default_char.unwrap_or('-'),
        }
    }

    /// Pretty print a TicTacToe board
    /// for visualizing a game state.
    pub fn print_board(&self) {
        for idx in 0..self.size {
            let start = self.size * idx;
            let end = self.size * (idx + 1);
            let sub: &[char] = &self.board[start as usize..end as usize];

            for &x in sub.iter() {
                print!("{}", x);
            }
            println!()
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