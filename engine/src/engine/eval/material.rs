use crate::chess::board::Board;
use crate::chess::game::Game;
use crate::chess::piece::PieceKind;
use crate::chess::player::Player;
use crate::engine::eval::params::BISHOP_PAIR_BONUS;
use crate::engine::eval::{PhasedEval, Trace, TraceComponentIncr};

pub fn trace_psts_and_material(game: &Game, trace: &mut Trace) {
    trace_psts_and_material_for_player(&game.board, Player::White, trace);
    trace_psts_and_material_for_player(&game.board, Player::Black, trace);
}

fn trace_psts_and_material_for_player(board: &Board, player: Player, trace: &mut Trace) {
    for sq in board.pawns(player) {
        trace.material[PieceKind::Pawn.array_idx()].incr(player);
        trace.pawn_pst[sq.relative_for(player).array_idx()].incr(player);
    }

    for sq in board.knights(player) {
        trace.material[PieceKind::Knight.array_idx()].incr(player);
        trace.knight_pst[sq.relative_for(player).array_idx()].incr(player);
    }

    for sq in board.bishops(player) {
        trace.material[PieceKind::Bishop.array_idx()].incr(player);
        trace.bishop_pst[sq.relative_for(player).array_idx()].incr(player);
    }

    for sq in board.rooks(player) {
        trace.material[PieceKind::Rook.array_idx()].incr(player);
        trace.rook_pst[sq.relative_for(player).array_idx()].incr(player);
    }

    for sq in board.queens(player) {
        trace.material[PieceKind::Queen.array_idx()].incr(player);
        trace.queen_pst[sq.relative_for(player).array_idx()].incr(player);
    }

    for sq in board.king(player) {
        trace.material[PieceKind::King.array_idx()].incr(player);
        trace.king_pst[sq.relative_for(player).array_idx()].incr(player);
    }
}

pub fn bishop_pair_eval<const TRACE: bool>(game: &Game, trace: &mut Trace) -> PhasedEval {
    let mut bishop_pair_bonuses = PhasedEval::ZERO;

    if game.board.bishops(Player::White).count() > 1 {
        bishop_pair_bonuses += BISHOP_PAIR_BONUS;

        if TRACE {
            trace.bishop_pair.incr(Player::White);
        }
    }

    if game.board.bishops(Player::Black).count() > 1 {
        bishop_pair_bonuses -= BISHOP_PAIR_BONUS;

        if TRACE {
            trace.bishop_pair.incr(Player::Black);
        }
    }

    bishop_pair_bonuses
}

pub fn eval<const TRACE: bool>(game: &Game, trace: &mut Trace) -> PhasedEval {
    bishop_pair_eval::<TRACE>(game, trace)
}
