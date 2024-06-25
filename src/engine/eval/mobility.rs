use super::PhasedEval;
use crate::chess::game::Game;
use crate::chess::movegen::tables;
use crate::chess::player::Player;
use crate::engine::eval::params::{
    BISHOP_MOBILITY, KNIGHT_MOBILITY, QUEEN_MOBILITY, ROOK_MOBILITY,
};

fn mobility_for(game: &Game, player: Player) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;
    let blockers = game.board.occupancy();

    for p in game.board.knights(player) {
        let moves = tables::knight_attacks(p).count() as usize;
        eval += KNIGHT_MOBILITY[moves];
    }

    for p in game.board.bishops(player) {
        let moves = tables::bishop_attacks(p, blockers).count() as usize;
        eval += BISHOP_MOBILITY[moves];
    }

    for p in game.board.rooks(player) {
        let moves = tables::rook_attacks(p, blockers).count() as usize;
        eval += ROOK_MOBILITY[moves];
    }

    for p in game.board.queens(player) {
        let moves = (tables::bishop_attacks(p, blockers) | tables::rook_attacks(p, blockers))
            .count() as usize;
        eval += QUEEN_MOBILITY[moves];
    }

    eval
}

pub fn eval(game: &Game) -> PhasedEval {
    mobility_for(game, Player::White) - mobility_for(game, Player::Black)
}
