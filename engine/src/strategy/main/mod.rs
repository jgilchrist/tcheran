use crate::options::EngineOptions;
use chess::game::Game;

use crate::search;

use super::{Control, GoArgs, Reporter, Strategy};

#[derive(Default)]
pub struct MainStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for MainStrategy {
    fn go(
        &mut self,
        game: &Game,
        args: &GoArgs,
        options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let (best_move, _eval) = search::search(game, args, options, &control, &reporter);

        reporter.best_move(best_move);
        control.stop();
    }
}
