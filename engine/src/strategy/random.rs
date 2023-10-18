use crate::options::EngineOptions;
use chess::game::Game;
use rand::prelude::SliceRandom;

use super::{Control, GoArgs, Reporter, Strategy};

#[derive(Default)]
pub struct RandomMoveStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for RandomMoveStrategy {
    fn go(
        &mut self,
        game: &Game,
        _args: &GoArgs,
        _options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let moves = game.legal_moves();
        let best_move = *moves.choose(&mut rand::thread_rng()).unwrap();

        reporter.best_move(best_move);
        control.stop();
    }
}
