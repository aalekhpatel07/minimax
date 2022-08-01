use std::{ops::{Deref, DerefMut}, error::Error, io::ErrorKind};
use crate::strategy::game_strategy::GameStrategy;

#[cfg(feature = "chess")]
pub use shakmaty::Chess as ShakmatyChess;
use shakmaty::Position;

#[derive(Debug, Clone)]
pub struct Chess {
    pub inner: ShakmatyChess,
    pub moves_played: shakmaty::MoveList
}

impl Default for Chess {
    fn default() -> Self {
        Self {
            inner: ShakmatyChess::default(),
            moves_played: shakmaty::MoveList::default()
        }
    }
}

impl Deref for Chess {
    type Target = ShakmatyChess;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Chess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Chess {

    pub fn new() -> Self {
        Self::default()
    }

    fn _undo(&self, _move: shakmaty::Move) -> Result<(), Box<dyn Error>>{
        todo!("Implement undo for Chess moves.");
    }

    pub fn undo(&mut self) -> Result<(), Box<dyn Error>>{
        if let Some(prev_move) = self.moves_played.pop() {
            self._undo(prev_move)
        } else {
            Err(Box::new(Error::new(ErrorKind::Other, "No moves to undo.")))
        }
    }

    fn _play(&mut self, _move: shakmaty::Move) {
        self.inner.play_unchecked(&_move);
        self.moves_played.push(_move);
    }
}

impl GameStrategy for Chess {
    type Player = shakmaty::Color;
    type Move = Option<shakmaty::Move>;
    type Board = shakmaty::Board;

    fn get_a_sentinel_move(&self) -> Self::Move {
        None
    }

    fn is_a_valid_move(&self, mv: &Self::Move) -> bool {
        mv.is_some()
    }

    fn play(&mut self, mv: &Self::Move, maximizer: bool) {
        if let Some(_mv) = mv {
            if maximizer {
                assert!(self.inner.turn() == shakmaty::Color::White);
                // self.inner.play(&_mv);
                self._play(_mv.clone());
                self.moves_played.push(_mv.clone());
            } else {
                assert!(self.inner.turn() == shakmaty::Color::Black);
                // self.inner.play(&mv);
                self._play(_mv.clone());
                self.moves_played.push(_mv.clone());
            }
        } else {
            panic!("Invalid move. Sentinel?");
        }
    }

    fn evaluate(&self) -> f64 {
        todo!("Implement a static evaluation of a chess position.")
    }

    fn clear(&mut self, mv: &Self::Move) {
        if mv.is_none() {
            panic!("Invalid move. Sentinel?");
        }   
        let prev_move = self.moves_played.pop();

        if prev_move.is_none() {
            panic!("Invalid move. Sentinel?");
        }
        let _mv = prev_move.unwrap();
        self._undo(_mv);

    }

    fn get_available_moves(&self) -> Vec<Self::Move> {
        self
        .legal_moves()
        .iter()
        .map(|mv| Some(mv.clone()))
        .collect()
    }

    fn get_board(&self) -> &Self::Board {
        &self.inner.board()
    }
    fn get_winner(&self) -> Option<Self::Player> {
        if let Some(outcome) = self.outcome() {
            match outcome {
                shakmaty::Outcome::Draw => None,
                shakmaty::Outcome::Decisive { winner } => Some(winner)
            }
        } else {
            None
        }
    }

    fn is_game_complete(&self) -> bool {
        self.outcome().is_some()
    }

    fn is_game_tied(&self) -> bool {
        if let Some(outcome) = self.outcome() {
            match outcome {
                shakmaty::Outcome::Draw => true,
                _ => false
            }
        } else {
            false
        }
    }

}

#[cfg(test)]
pub mod tests {
    pub use super::Chess;
    pub use crate::strategy::game_strategy::GameStrategy;
    use shakmaty::{Chess as ChessGame, Board, Square, Piece, Color, Role, Position, Setup, FromSetup, CastlingMode};

    #[test]
    fn test_chess_new() {
        let chess = Chess::new();
        assert_eq!(chess.turn(), shakmaty::Color::White);
    }

    #[test]
    fn test_chess_evaluate() {
        let mut chess = Chess::new();
        assert_eq!(chess.evaluate(), 0.);
    }

    #[test]
    fn test_chess_available_moves() {
        let mut chess = Chess::new();
        let moves = chess.get_available_moves();
        assert_eq!(moves.len(), chess.legal_moves().len());

        println!("{:?}", moves);
    }

    #[test]
    fn test_chess_available_moves_capture() {
        let mut chess_setup = Setup::default();

        let mut board = chess_setup.board;
        board.set_piece_at(Square::E4, Piece { color: Color::White, role: Role::Pawn });
        board.remove_piece_at(Square::E2).unwrap();
        board.remove_piece_at(Square::D7).unwrap();
        board.set_piece_at(Square::D5, Piece { color: Color::Black, role: Role::Pawn });

        chess_setup.board = board;

        let mut chess = ChessGame::from_setup(chess_setup, CastlingMode::Standard).unwrap();

        let moves = chess.capture_moves();
        println!("{moves:#?}");
    }

}