use crate::games::TicTacToe;
use crate::strategy::alpha_beta_minimax::AlphaBetaMiniMaxStrategy;
use crate::strategy::game_strategy::GameStrategy;

/// Read input.
fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

/// Play a game of any size in a REPL against the engine.
/// The default depth of 6 should make the
/// engine reasonably fast.
pub fn play_tic_tac_toe_against_computer(size: usize) {
    play_tic_tac_toe_against_computer_with_depth(size, 6)
}

/// Play a game of any size in a REPL against the engine.
/// The higher the depth, the longer it takes and
/// the more accurately the engine performs.
pub fn play_tic_tac_toe_against_computer_with_depth(size: usize, depth: i64) {
    let mut ttt = TicTacToe::new(size);
    loop {
        println!("Board:\n{}", ttt);
        println!("\n");

        if ttt.is_game_complete() {
            println!("Game is complete.");
            if ttt.is_game_tied() {
                println!("Game Tied!");
                break;
            } else {
                println!("{} wins!", ttt.get_winner().unwrap());
                break;
            }
        }

        let example_num: i64 = 7;
        println!(
            "Enter a move. (e.g. '{}' represents (row: {}, col: {}) : ",
            example_num,
            example_num as usize / size,
            example_num as usize % size
        );
        let s = get_input().trim().parse::<usize>();
        let n = s.unwrap_or(usize::MAX);

        if n == usize::MAX {
            break;
        }
        println!(
            "Move played by you: {} (i.e. {}, {})",
            n,
            n / size,
            n % size
        );
        ttt.play(&n, true);
        let move_found = ttt.get_best_move(depth as i64, true);
        if move_found > (ttt.size * ttt.size) {
            println!("Game is complete.");
            if ttt.is_game_tied() {
                println!("Game Tied!");
                break;
            } else {
                println!("{} wins!", ttt.get_winner().unwrap());
                break;
            }
        }
        println!(
            "Move played by AI: {} (i.e. {}, {})",
            move_found,
            move_found / size,
            move_found % size
        );
        ttt.play(&move_found, false);
    }
}