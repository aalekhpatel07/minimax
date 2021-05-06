//! Solve any Two-player Minimax game using the
//! Minimax algorithm with Alpha-Beta pruning.
//! Also, where possible, a parallel processing
//! implementation is provided.


/// Contains sruct and necessary implementations
/// for `TicTacToe`: a popular two-player game
/// where one player places a symbol - 'X' and another
/// player places a symbol - 'O' on a square grid
/// with the objective of creating a streak of same
/// symbols of length the size of the grid in any direction.
///
/// # Examples
///
/// ```
/// let mut tic_tac_toe = minimax::tictactoe::TicTacToe::create_game(3, None, None, None);
/// tic_tac_toe.print_board();
/// assert_eq!(tic_tac_toe.size, 3);
/// assert_eq!(tic_tac_toe.default_char, '-');
/// ```
pub mod tictactoe {
    pub struct TicTacToe {
        pub board: Vec<char>,
        pub size: usize,
        pub default_char: char,
        pub maximizer: char,
        pub minimizer: char,
    }

    impl TicTacToe {
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

        pub fn check_diagonals(&self) -> char {
            let mut winner = self.default_char;
            if self.check_diagonal(self.maximizer, true) || self.check_diagonal(self.maximizer, false) {
                winner = self.maximizer
            } else if self.check_diagonal(self.minimizer, true)
                || self.check_diagonal(self.minimizer, false)
            {
                winner = self.minimizer
            }
            winner
        }

        pub(crate) fn check_rows(&self) -> char {
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

        fn check_col(&self, ch: char, col_num: usize) -> bool {
            for row in 0..self.size as usize {
                if self.board[self.size as usize * row + col_num] != ch {
                    return false;
                }
            }
            true
        }

        fn check_row(&self, ch: char, row_num: usize) -> bool {
            for col in 0..self.size as usize {
                if self.board[self.size as usize * row_num + col] != ch {
                    return false;
                }
            }
            true
        }
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
                    if self.board[(self.size as usize * (self.size as usize - 1 - idx as usize)) + idx]
                        != ch
                    {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/// Contains the behaviours for
/// two player games.
///
pub mod strategy {

    pub trait Strategy {
        type Player;
        type Move;
        type Board;

        fn evaluate(&self) -> f64;
        fn get_winner(&self) -> Self::Player;
        fn is_game_tied(&self) -> bool;
        fn is_game_complete(&self) -> bool;
        fn get_available_moves(&self) -> Vec<Self::Move>;
        fn play(&mut self, mv: &Self::Move, maximizer: bool);
        fn clear(&mut self, mv: &Self::Move);
        fn get_board(&self) -> &Self::Board;
        fn is_a_valid_move(&self, mv: &Self::Move) -> bool;
        fn get_a_sentinel_move(&self) -> Self::Move;
    }

    pub trait AlphaBetaMiniMaxStrategy: Strategy {
        fn get_best_move(&mut self, max_depth: i64, is_maximizing: bool) -> <Self as Strategy>::Move;
        fn minimax_score(&mut self, depth: i64, is_maximizing: bool, alpha: f64, beta: f64, max_depth: i64) -> f64;
    }
}

pub mod body {

    use crate::strategy::*;
    use crate::tictactoe::*;


    impl Strategy for TicTacToe {
        type Player = char;
        type Move = usize;
        type Board = Vec<char>;

        fn evaluate(&self) -> f64 {
            if self.is_game_tied() {
                0.
            } else {
                let _winner = self.get_winner();
                if _winner == self.maximizer {
                    1000.
                } else {
                    -1000.
                }
            }
        }

        fn get_winner(&self) -> Self::Player {
            let mut winner = self.check_diagonals();

            if winner == self.default_char {
                winner = self.check_rows();
            }
            if winner == self.default_char {
                winner = self.check_cols();
            }
            winner
        }

        fn is_game_tied(&self) -> bool {
            let _winner = self.get_winner();

            _winner == self.default_char && self.get_available_moves().is_empty()
        }

        fn is_game_complete(&self) -> bool {
            let _winner = self.get_winner();

            self.get_available_moves().is_empty() || _winner != '-'
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





    pub const INF: f64 = f64::INFINITY;
    pub const NEG_INF: f64 = f64::NEG_INFINITY;

    impl<T: Strategy> AlphaBetaMiniMaxStrategy for T {
        fn get_best_move(&mut self, max_depth: i64, is_maximizing: bool) -> <Self as Strategy>::Move {
            let mut best_move: <Self as Strategy>::Move = self.get_a_sentinel_move();

            if self.is_game_complete() {
                return best_move;
            }

            let alpha = NEG_INF;
            let beta = INF;

            if is_maximizing {
                let mut best_move_val: f64 = INF;

                for mv in self.get_available_moves() {
                    self.play(&mv, false);
                    let value = self.minimax_score(max_depth, true, alpha, beta, max_depth);
                    self.clear(&mv);
                    if value <= best_move_val {
                        best_move_val = value;
                        best_move = mv;
                    }
                }

                best_move
            } else {
                let mut best_move_val: f64 = NEG_INF;

                for mv in self.get_available_moves() {
                    self.play(&mv, false);
                    let value = self.minimax_score(max_depth, false, alpha, beta, max_depth);
                    self.clear(&mv);
                    if value >= best_move_val {
                        best_move_val = value;
                        best_move = mv;
                    }
                }
                best_move
            }
        }

        fn minimax_score(
            &mut self,
            depth: i64,
            is_maximizing: bool,
            mut alpha: f64,
            mut beta: f64,
            max_depth: i64,
        ) -> f64 {

            let avail: Vec<<T as Strategy>::Move> = self.get_available_moves();
            if depth == 0 || self.is_game_complete() || avail.is_empty() {
                return self.evaluate();
            }

            if is_maximizing {
                let mut value = NEG_INF;
                for idx in avail {
                    self.play(&idx, true);
                    let score = self.minimax_score(depth - 1, false, alpha, beta, max_depth);
                    if score >= value {
                        value = score;
                    }
                    if score >= alpha {
                        alpha = score;
                    }
                    self.clear(&idx);
                    if beta <= alpha {
                        break;
                    }
                }
                if value != 0. {
                    return value - (max_depth - depth) as f64;
                }
                value
            } else {
                let mut value = INF;
                for idx in avail {
                    self.play(&idx, false);
                    let score = self.minimax_score(depth - 1, true, alpha, beta, max_depth);
                    if score <= value {
                        value = score;
                    }
                    if score <= beta {
                        beta = score;
                    }
                    self.clear(&idx);
                    if beta <= alpha {
                        break;
                    }
                }

                if value != 0. {
                    return value + (max_depth - depth) as f64;
                }
                value
            }
        }
    }
}