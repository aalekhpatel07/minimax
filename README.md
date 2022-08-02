# Implementation for a minimax game engine.

## Background

A Minimax game is any turn-based two player game where the objective of
one player is to maximize the evaluation score while the opponent
tries to minimize it. Examples of such games include Chess, Go, TicTacToe, Connect Four, etc.

In this crate, there is a generic implementation of the minimax game-playing engine that can be used for any such games. The minimax algorithm is notorious for being slow so we speed it up using a pruning method called Alpha-Beta pruning.

The minimax algorithm under-the-hood tries to simulate every possible "best" gameplays from either sides and based on the results, recommends a move that should be played to best improve the winning chances of the player to move.

There is a caveat, in that, the minimax algorithm is too slow (since it explores the search space almost uncleverly). One optimization is that of pruning the search space by the means of a clever approach in which we "rule out gameplay sequences which the opponent won't definitely let us improve in." This is achieved by a technique called Alpha-Beta pruning.

## Usage

### As a binary

Install with:
```sh
cargo install minimax-alpha-beta
```

Get usage instructions with:
```sh
tic-tac-toe --help
```
```
minimax-alpha-beta 0.2.0
Aalekh Patel <aalekh.gwpeck.7998@icloud.com>
Play a game of Tic Tac Toe with a computer opponent that uses the Alpha-Beta Minimax Engine.

USAGE:
    tic-tac-toe [OPTIONS]

OPTIONS:
        --depth <DEPTH>    The depth of the search [default: 9]
    -h, --help             Print help information
        --size <SIZE>      The size of the board [default: 3]
    -V, --version          Print version information
```

For example, to play a regular 3x3 tic-tac-toe game just use `tic-tac-toe`.

Otherwise to play a 6x6 tic-tac-toe game with search depth of 5, use
```sh
tic-tac-toe --depth 5 --size 6
```

### As a library

The crate provides concrete implementations for TicTacToe, (note: other games are in works).

Use the `TicTacToe::get_best_move(depth, player)` method to compute the best move in this position for this player.

### To use a pre-written driver:

```rust

use minimax_alph_beta::drivers;

fn main() {
    let grid_size: usize = 3;
    let search_depth: i64 = 6;

    // Control the depth to trade running time and accuracy.
    // The higher the depth the slower the compute and higher the accuracy.
    // The lower the depth the faster the compute and lower the accuracy.
    drivers::play_tic_tac_toe_against_computer_with_depth(grid_size, search_depth);

    // Or simply use the default balance of `depth = 6`.
    drivers::play_tic_tac_toe_against_computer(grid_size);
}
```

### To write a driver yourself:

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

### To use the engine for a completely new minimax game (e.g. chess):

```rust

use minimax_alpha_beta::strategy::{Strategy, AlphaBetaMiniMaxStrategy};

/// Define the Chess structure.
pub struct Chess {
    /// The board could be represented by a vector of 64 characters.
    pub board: Vec<char>,
    /// The default char could be a '.'
    pub default_char: char,
    /// The maximizer is White.
    pub maximizer: char,
    /// The minimizer is Black.
    pub minimizer: char,
}

// Implement any basic methods on Chess.
// This should ideally allow us to work with
// Chess at a very low level.

impl Chess {
    // ...
}

// You'll likely need to have a new struct
// that represents a move in Chess.
pub struct ChessMove {
    // ...
}

// Implement all the higher level
// methods on Chess. This should ideally
// be compositions of the basic methods
// in the default impl of Chess.

impl Strategy for Chess {
    // ...
    type Move = ChessMove;
    // ...
}

// Once Strategy is implemented for Chess, the Minimax engine should be ready to use!

// Make sure there is a create_game in the default impl for Chess.
// Then create a game with parameters as necessary.
let mut chessboard = Chess::create_game()

let search_depth: i64 = 6;

// Play arbitrary number of moves depending on how've you defined it in the default impl.
chessboard.play(&your_move, true);

let best_move: ChessMove = chessboard.get_best_move(search_depth, true);

chessboard.play(&best_move, true);
chessboard.print_board();
```

## Show appreciation

I enjoyed creating this and is one of my first excursions in Rust.
If you found this library useful or maybe just cool, consider [awarding a star](https://www.github.com/aalekhpatel07/minimax).

## Questions

Please [create an issue](https://www.github.com/aalekhpatel07/minimax/issues).

## Contribution

All [pull requests](https://www.github.com/aalekhpatel07/minimax/pull) are welcome!


## Contact

Got any cool collaboration ideas? My github is [aalekhpatel07](https://www.github.com/aalekhpatel07) and you can reach me [here](mailto:itsme@aalekhpatel.com).
