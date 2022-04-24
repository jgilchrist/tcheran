use chess::{game::Game, movegen::generate_moves, r#move::Move};
use rand::prelude::SliceRandom;

pub mod uci;

fn run(game: &Game) -> Move {
    let moves = generate_moves(game);
    *moves.choose(&mut rand::thread_rng()).unwrap()
}
