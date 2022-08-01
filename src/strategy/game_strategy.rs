/// Any two-player Minimax game must
/// have this behavior. In other words,
/// these functions should yield meaningful outputs
/// for any two-player games.


pub enum GameResult {
    Player1Win,
    Player2Win,
    Draw
}


pub trait GameStrategy {
    type Player;
    type Move;
    type Board;

    /// Ability to statically evaluate the current game state.
    fn evaluate(&self) -> f64;
    /// Identify a winner, if exists.
    fn get_winner(&self) -> Option<Self::Player>;
    /// Identify if the game is tied.
    fn is_game_tied(&self) -> bool;
    /// Identify if the game is in a completed state.
    fn is_game_complete(&self) -> bool;
    /// Ability to produce a collection of playable legal moves
    /// in the current position.
    fn get_available_moves(&self) -> Vec<Self::Move>;
    /// Modify the game state by playing a given move.
    fn play(&mut self, mv: &Self::Move, maximizer: bool);
    /// Modify the game state by resetting a given move.
    fn clear(&mut self, mv: &Self::Move);
    /// Get the current state of the board.
    fn get_board(&self) -> &Self::Board;
    /// Determine if a given move is valid.
    fn is_a_valid_move(&self, mv: &Self::Move) -> bool;
    /// Ability to produce a sentinel (not-playable) move.
    fn get_a_sentinel_move(&self) -> Self::Move;
}