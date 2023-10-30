use crate::options::EngineOptions;
use chess::game::Game;
use rand::prelude::SliceRandom;

use super::{Control, Reporter, SearchRestrictions, Strategy, TimeControl};

#[derive(Default)]
pub struct RandomMoveStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for RandomMoveStrategy {
    fn go(
        &mut self,
        game: &mut Game,
        _time_control: &TimeControl,
        _restrictions: &SearchRestrictions,
        _options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let moves = game
            .pseudo_legal_moves()
            .into_iter()
            .filter(|m| {
                let player = game.player;
                game.make_move(m);
                let is_in_check = game.board.king_in_check(player);
                game.undo_move();
                !is_in_check
            })
            .collect::<Vec<_>>();

        let best_move = *moves.choose(&mut rand::thread_rng()).unwrap();

        reporter.best_move(best_move);
        control.stop();
    }
}
