use crate::engine::game::EngineGame;
use crate::engine::options::EngineOptions;

use rand::prelude::SliceRandom;

use super::{Control, Reporter, SearchRestrictions, Strategy, TimeControl};

#[derive(Default)]
pub struct RandomMoveStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for RandomMoveStrategy {
    fn go(
        &mut self,
        game: &mut EngineGame,
        _time_control: &TimeControl,
        _restrictions: &SearchRestrictions,
        _options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let moves = game.moves();
        let best_move = *moves.choose(&mut rand::thread_rng()).unwrap();

        reporter.best_move(best_move);
        control.stop();
    }
}
