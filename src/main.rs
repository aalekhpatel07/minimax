use minimax_alpha_beta::*;
use clap::{Parser};

#[derive(Parser, Debug, Clone)]
#[
    clap(
        author = "Aalekh Patel <aalekh.gwpeck.7998@icloud.com>",
        version = "0.2.0",
        about="Play a game of Tic Tac Toe with a computer opponent that uses the Alpha-Beta Minimax Engine.",
    )
]
pub struct Cli {
    /// The size of the board.
    #[clap(long, default_value_t = 3)]
    pub size: usize,
    /// The depth of the search.
    #[clap(long, default_value_t = 9)]
    pub depth: i64
}

fn main() {
    let cli = Cli::parse();
    play_tic_tac_toe_against_computer_with_depth(cli.size, cli.depth);
}
