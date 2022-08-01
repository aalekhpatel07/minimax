// #[cfg(test)]

// mod tictactoe {

//     use crate::strategy::{AlphaBetaMiniMaxStrategy, Strategy};
//     use crate::tictactoe::TicTacToe;

//     #[test]
//     fn setup_game() {
//         let ttt = TicTacToe::create_game(5, None, None, None);
//         assert_eq!(ttt.is_game_complete(), false);
//         assert_eq!(ttt.is_game_tied(), false);
//         assert_eq!(ttt.get_winner(), '-');
//         assert_eq!(ttt.size, 5);

//         let ttt_regular = TicTacToe::create_game(5, Some('_'), Some('O'), Some('X'));
//         assert_eq!(ttt_regular.default_char, '_');
//         assert_eq!(ttt_regular.maximizer, 'O');
//         assert_eq!(ttt_regular.minimizer, 'X');
//     }

//     #[test]
//     fn test_get_best_move_3_by_3_depth_12() {
//         let mut ttt = TicTacToe::create_game(3, None, None, None);

//         ttt.play(&4, true);
//         ttt.play(&0, false);

//         let best = ttt.get_best_move(12 as i64, true);

//         // Since the game state is:
//         //
//         // x \_ \_
//         //
//         // \_ o \_
//         //
//         // \_ \_ \_
//         //
//         // and it is 'o' to move,
//         // any corner square is the best.
//         //
//         // So let's test that it is indeed the case.

//         assert_eq!(best % 2, 0);
//     }

//     #[test]
//     fn test_get_best_move_4_by_4_depth_6() {
//         let mut ttt = TicTacToe::create_game(4, None, None, None);
//         ttt.get_best_move(3 as i64, true);
//     }
//     #[test]
//     fn test_get_best_move_4_by_4_depth_8() {
//         let mut ttt = TicTacToe::create_game(4, None, None, None);
//         ttt.get_best_move(2 as i64, true);
//     }
//     #[test]
//     fn test_get_best_move_5_by_5_depth_6_after_5_moves_played() {
//         let mut ttt = TicTacToe::create_game(5, None, None, None);

//         ttt.play(&12, true);
//         ttt.play(&22, false);
//         ttt.play(&1, true);
//         ttt.play(&16, false);
//         ttt.play(&8, true);
//         ttt.get_best_move(6 as i64, false);
//     }
// }
