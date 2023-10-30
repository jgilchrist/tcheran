use crate::options::EngineOptions;
use chess::game::Game;

use crate::search;
use crate::transposition::transposition_table::SearchTranspositionTable;

use super::{Control, GoArgs, Reporter, Strategy};

pub struct MainStrategy {
    tt: SearchTranspositionTable,
}

impl Default for MainStrategy {
    fn default() -> Self {
        Self {
            tt: SearchTranspositionTable::new(),
        }
    }
}

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for MainStrategy {
    fn go(
        &mut self,
        game: &mut Game,
        args: &GoArgs,
        options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let (best_move, _eval) =
            search::search(game, &mut self.tt, args, options, &control, &reporter);

        reporter.best_move(best_move);
        control.stop();
    }
}
