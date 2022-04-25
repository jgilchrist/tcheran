use chess::{game::Game, movegen::generate_moves, r#move::Move};
use rand::prelude::SliceRandom;

pub mod uci;

fn run(game: &Game) -> Move {
    let moves = generate_moves(game);

    let legal_moves: Vec<Move> = moves
        .into_iter()
        .filter(|m| {
            let next_state = game.make_move(m).unwrap();
            !next_state.king_in_check(&game.player)
        })
        .collect();

    *legal_moves.choose(&mut rand::thread_rng()).unwrap()
}
