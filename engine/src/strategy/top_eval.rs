use chess::game::Game;

use crate::eval;
use crate::options::EngineOptions;

use super::{Control, GoArgs, Reporter, Strategy};

#[derive(Default)]
pub struct TopEvalStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for TopEvalStrategy {
    fn go(
        &mut self,
        game: &mut Game,
        _args: &GoArgs,
        _options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let mut moves = game.legal_moves();
        moves.sort_unstable_by_key(|m| {
            game.make_move(m);
            let result = eval::eval(game);
            game.undo_move();
            result
        });

        let mv = *moves.first().expect("Could not find a legal move");

        reporter.best_move(mv);
        control.stop();
    }
}
