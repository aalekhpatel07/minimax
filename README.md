# Implementation for a minimax game engine.

## Background

A Minimax game is any turn-based two player game where the objective of
one player is to maximize the evaluation score while the opponent
tries to minimize it. Examples of such games include Chess, Go, TicTacToe, Connect Four, etc.

In this crate, there is a generic implementation of the minimax game-playing engine that can be used for any such games. The minimax algorithm is notorious for being slow so we speed it up using a pruning method called Alpha-Beta pruning.

The minimax algorithm under-the-hood tries to simulate every possible "best" gameplays from either sides and based on the results, recommends a move that should be played to best improve the winning chances of the player to move.

There is a caveat, in that, the minimax algorithm is too slow (since it explores the search space almost uncleverly). One optimization is that of pruning the search space by the means of a clever approach in which we "rule out gameplay sequences which the opponent won't definitely let us improve in." This is achieved by a technique called Alpha-Beta pruning.

## Usage

The crate provides concrete implementations for TicTacToe, (note: other games are in works).

Use the `TicTacToe::get_best_move(depth, player)` method to compute the best move in this position for this player.

```rust
use minimax_alpha_beta::tictactoe::{TicTacToe};
use minimax_alpha_beta::strategy::*;

let mut ttt = TicTacToe::create_game(3, None, None, None);
ttt.print_board();

// The first argument takes a reference to the move position.
// The structure of the board is like [[0, 1, 2], [3, 4, 5], [6, 7, 8]].
// The second argument governs who plays this move;
// true means the first player, false means the second.

ttt.play(&4, true);
ttt.play(&0, false);

ttt.print_board();

// The first argument is the depth to explore.
// The higher the depth, the more the time it takes to compute
// the best move and that the chances of it being the best move increase.
// The lower the depth, the faster it takes to compute but the less
// the likelihood of being the best move.

// The second argument governs who plays this move;
// true means the first player, false means the second.
let best = ttt.get_best_move(6 as i64, true);

ttt.play(&best, true);

ttt.print_board();

```
