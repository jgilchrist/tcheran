use chess::game::Game;
use rand::prelude::SliceRandom;

use super::{Reporter, Strategy};

#[derive(Default)]
pub struct RandomMoveStrategy;

impl<T: Reporter> Strategy<T> for RandomMoveStrategy {
    fn go(&mut self, game: &Game, reporter: T) {
        let moves = game.legal_moves();
        let best_move = *moves.choose(&mut rand::thread_rng()).unwrap();

        reporter.best_move(best_move);
    }
}
