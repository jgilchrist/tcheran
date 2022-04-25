use chess::{debug, game::Game, moves::Move};

mod eval;
pub mod uci;

fn run(game: &Game) -> Move {
    let mut legal_moves = game.legal_moves();
    legal_moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

    let mv = *legal_moves.first().expect("Could not find a legal move");
    let next_game = game.make_move(&mv).unwrap();
    debug::log("eval", format!("{:?}", eval::eval(&next_game)));
    mv
}
