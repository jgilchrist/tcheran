use chess::{debug, game::Game, moves::Move};

mod eval;
mod search;
pub mod uci;

fn run(game: &Game) -> Move {
    let best_move = search::search(game);

    let next_game = game.make_move(&best_move).unwrap();
    debug::log("eval", format!("{:?}", eval::eval(&next_game)));
    best_move
}
