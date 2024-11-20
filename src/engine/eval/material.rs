use crate::chess::game::Game;
use crate::chess::player::Player;
use crate::engine::eval::PhasedEval;

const BISHOP_PAIR_BONUS: PhasedEval = PhasedEval::new(27, 65);

pub fn bishop_pair_eval(game: &Game) -> PhasedEval {
    let mut bishop_pair_bonuses = PhasedEval::ZERO;

    if game.board.bishops(Player::White).count() > 1 {
        bishop_pair_bonuses += BISHOP_PAIR_BONUS;
    }

    if game.board.bishops(Player::Black).count() > 1 {
        bishop_pair_bonuses -= BISHOP_PAIR_BONUS;
    }

    bishop_pair_bonuses
}

pub fn eval(game: &Game) -> PhasedEval {
    bishop_pair_eval(game)
}
