use chess::game::Game;
use crate::options::EngineOptions;

use crate::search;

use super::{Control, Reporter, Strategy};

#[derive(Default)]
pub struct MainStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for MainStrategy {
    fn go(&mut self, game: &Game, options: &EngineOptions, control: TCx, reporter: TRx) {
        let (best_move, _eval) = search::search(game, options, &reporter);

        reporter.best_move(best_move);
        control.stop();
    }
}
