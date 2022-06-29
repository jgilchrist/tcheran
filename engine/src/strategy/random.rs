use chess::{game::Game, moves::Move};
use rand::prelude::SliceRandom;

use super::Strategy;

#[derive(Default)]
pub struct RandomMoveStrategy;

impl Strategy for RandomMoveStrategy {
    fn next_move(&mut self, game: &Game) -> Move {
        let moves = game.legal_moves();
        *moves.choose(&mut rand::thread_rng()).unwrap()
    }
}
