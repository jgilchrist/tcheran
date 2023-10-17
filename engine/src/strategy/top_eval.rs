use chess::game::Game;

use crate::eval;
use crate::options::EngineOptions;

use super::{Control, Reporter, Strategy};

#[derive(Default)]
pub struct TopEvalStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for TopEvalStrategy {
    fn go(&mut self, game: &Game, _options: &EngineOptions, control: TCx, reporter: TRx) {
        let mut moves = game.legal_moves();
        moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

        let mv = *moves.first().expect("Could not find a legal move");

        reporter.best_move(mv);
        control.stop();
    }
}
