use crate::strategy::game_strategy::GameStrategy;

pub const INF: f64 = f64::INFINITY;
pub const NEG_INF: f64 = f64::NEG_INFINITY;


/// The behaviour required of any
/// minimax game engine.
pub trait AlphaBetaMiniMaxStrategy: GameStrategy {
    /// The ability to get the best move
    /// in the current state and for the
    /// current player.
    fn get_best_move(
        &mut self,
        max_depth: i64,
        is_maximizing: bool,
    ) -> <Self as GameStrategy>::Move;

    /// The ability to produce a best (good enough, sometimes)
    /// evaluation score possible over all
    /// possible moves at the current game state.
    fn minimax_score(
        &mut self,
        depth: i64,
        is_maximizing: bool,
        alpha: f64,
        beta: f64,
        max_depth: i64,
    ) -> f64;
}

/// Endow upon anything the ability to
/// use the AlphaBetaMiniMaxStrategy implementation
/// of the game engine as long as it understands
/// how to behave as Strategy.
impl<T: GameStrategy> AlphaBetaMiniMaxStrategy for T {
    fn get_best_move(
        &mut self,
        max_depth: i64,
        is_maximizing: bool,
    ) -> <Self as GameStrategy>::Move {
        let mut best_move: <Self as GameStrategy>::Move = self.get_a_sentinel_move();

        if self.is_game_complete() {
            return best_move;
        }

        let alpha = NEG_INF;
        let beta = INF;

        if is_maximizing {
            let mut best_move_val: f64 = INF;

            for mv in self.get_available_moves() {
                self.play(&mv, !is_maximizing);
                let value = self.minimax_score(max_depth, is_maximizing, alpha, beta, max_depth);
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
                self.play(&mv, !is_maximizing);
                let value = self.minimax_score(max_depth, is_maximizing, alpha, beta, max_depth);
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
        let avail: Vec<<T as GameStrategy>::Move> = self.get_available_moves();
        if depth == 0 || self.is_game_complete() || avail.is_empty() {
            return self.evaluate();
        }

        if is_maximizing {
            let mut value = NEG_INF;
            for idx in avail {
                self.play(&idx, true);
                let score = self.minimax_score(depth - 1, false, alpha, beta, max_depth);
                // if score >= value {
                //     value = score;
                // }
                value = value.max(score);
                alpha = alpha.max(score);
                // if score >= alpha {
                //     alpha = score;
                // }
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
                value = value.min(score);
                beta = beta.min(score);
                // if score <= value {
                //     value = score;
                // }
                // if score <= beta {
                //     beta = score;
                // }
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