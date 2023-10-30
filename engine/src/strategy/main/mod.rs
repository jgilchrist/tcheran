use crate::options::EngineOptions;
use chess::game::Game;

use crate::search;
use crate::transposition::transposition_table::SearchTranspositionTable;

use super::{Control, Reporter, SearchRestrictions, Strategy, TimeControl};

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
        time_control: &TimeControl,
        restrictions: &SearchRestrictions,
        options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let (best_move, _eval) = search::search(
            game,
            &mut self.tt,
            time_control,
            restrictions,
            options,
            &control,
            &reporter,
        );

        reporter.best_move(best_move);
        control.stop();
    }
}
