use chess::game::Game;

use crate::search;

use super::{Control, Reporter, Strategy};

#[derive(Default)]
pub struct MainStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for MainStrategy {
    fn go(&mut self, game: &Game, control: TCx, reporter: TRx) {
        let (best_move, _eval) = search::search(game, &reporter);

        reporter.best_move(best_move);
        control.stop();
    }
}
