use chess::game::Game;

use crate::eval;

use super::{Reporter, Strategy};

#[derive(Default)]
pub struct TopEvalStrategy;

impl<T: Reporter> Strategy<T> for TopEvalStrategy {
    fn go(&mut self, game: &Game, reporter: T) {
        let mut moves = game.legal_moves();
        moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

        let mv = *moves.first().expect("Could not find a legal move");

        reporter.best_move(mv);
    }
}
