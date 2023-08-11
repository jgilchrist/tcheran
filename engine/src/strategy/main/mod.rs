use chess::game::Game;

use crate::search;

use super::{Reporter, Strategy};

#[derive(Default)]
pub struct MainStrategy;

impl<T: Reporter> Strategy<T> for MainStrategy {
    fn go(&mut self, game: &Game, reporter: T) {
        let (best_move, _eval) = search::search(game, &reporter);

        reporter.best_move(best_move);
    }
}
