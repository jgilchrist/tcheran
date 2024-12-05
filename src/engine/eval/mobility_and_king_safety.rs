use super::PhasedEval;
use crate::chess::bitboard::Bitboard;
use crate::chess::game::Game;
use crate::chess::movegen::tables;
use crate::chess::player::Player;
use crate::engine::eval::params::{
    ATTACKED_KING_SQUARES, BISHOP_MOBILITY, KNIGHT_MOBILITY, QUEEN_MOBILITY, ROOK_MOBILITY,
};

fn mobility_and_opp_king_safety_for(game: &Game, player: Player) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;
    let blockers = game.board.occupancy();

    let their_pawns = game.board.pawns(player.other()).forward(player.other());
    let their_pawn_attacks = their_pawns.west() | their_pawns.east();
    let mobility_safe_squares = !their_pawn_attacks;

    let mut attacked_squares = Bitboard::EMPTY;

    for p in game.board.knights(player) {
        let moves = tables::knight_attacks(p);
        attacked_squares |= moves;

        let mobility_squares = (moves & mobility_safe_squares).count() as usize;
        eval += KNIGHT_MOBILITY[mobility_squares];
    }

    for p in game.board.bishops(player) {
        let moves = tables::bishop_attacks(p, blockers);
        attacked_squares |= moves;

        let mobility_squares = (moves & mobility_safe_squares).count() as usize;
        eval += BISHOP_MOBILITY[mobility_squares];
    }

    for p in game.board.rooks(player) {
        let moves = tables::rook_attacks(p, blockers);
        attacked_squares |= moves;

        let mobility_squares = (moves & mobility_safe_squares).count() as usize;
        eval += ROOK_MOBILITY[mobility_squares];
    }

    for p in game.board.queens(player) {
        let moves = tables::bishop_attacks(p, blockers) | tables::rook_attacks(p, blockers);
        attacked_squares |= moves;

        let mobility_squares = (moves & mobility_safe_squares).count() as usize;
        eval += QUEEN_MOBILITY[mobility_squares];
    }

    let enemy_king = game.board.king(player.other()).single();
    let enemy_king_surrounding_squares = tables::king_attacks(enemy_king);
    let attacks_on_enemy_king = attacked_squares & enemy_king_surrounding_squares;

    eval -= ATTACKED_KING_SQUARES[attacks_on_enemy_king.count() as usize];

    eval
}

pub fn eval(game: &Game) -> PhasedEval {
    mobility_and_opp_king_safety_for(game, Player::White)
        - mobility_and_opp_king_safety_for(game, Player::Black)
}
