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
/// use minimax_alpha_beta::tictactoe::TicTacToe;
/// let mut tic_tac_toe = TicTacToe::create_game(3, None, None, None);
/// tic_tac_toe.print_board();
/// assert_eq!(tic_tac_toe.size, 3);
/// assert_eq!(tic_tac_toe.default_char, '-');
/// ```
// mod tests;
pub mod games;
pub mod strategy;
mod drivers;

pub use drivers::*;