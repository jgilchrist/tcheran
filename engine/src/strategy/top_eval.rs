use chess::{game::Game, moves::Move};

use crate::eval;

use super::Strategy;

#[derive(Default)]
pub struct TopEvalStrategy;

impl Strategy for TopEvalStrategy {
    fn next_move(&mut self, game: &Game) -> Move {
        let mut moves = game.legal_moves();
        moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

        let mv = *moves.first().expect("Could not find a legal move");
        mv
    }
}
