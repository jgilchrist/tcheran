use chess::{game::Game, moves::Move};

use crate::eval;

pub fn search(game: &Game) -> Move {
    let mut legal_moves = game.legal_moves();
    legal_moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

    let mv = *legal_moves.first().expect("Could not find a legal move");
    mv
}
