pub mod nnue;
mod player_eval;
mod white_eval;

pub use player_eval::Eval;
pub use white_eval::WhiteEval;

use crate::chess::game::Game;

pub fn eval(game: &Game) -> Eval {
    game.nnue.evaluate(game.player)
}
