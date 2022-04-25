use chess::{game::Game, moves::Move};
use rand::prelude::SliceRandom;

pub mod uci;

fn run(game: &Game) -> Move {
    *game
        .legal_moves()
        .choose(&mut rand::thread_rng())
        .unwrap()
}
