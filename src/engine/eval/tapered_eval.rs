use crate::chess::board::Board;
use crate::chess::piece::PieceKind;
use crate::chess::square::Square;
use crate::engine::eval::WhiteEval;

pub fn piece_phase_value_contribution(kind: PieceKind) -> i16 {
    match kind {
        PieceKind::Pawn | PieceKind::King => 0,
        PieceKind::Knight | PieceKind::Bishop => 1,
        PieceKind::Rook => 2,
        PieceKind::Queen => 4,
    }
}

pub fn phase_value(board: &Board) -> i16 {
    let mut v = 0;

    for idx in 0..Square::N {
        let maybe_piece = board.pieces[idx];

        if let Some(piece) = maybe_piece {
            v += piece_phase_value_contribution(piece.kind);
        }
    }

    v
}

const PHASE_COUNT_MAX: i64 = 24;

pub fn taper(phase_value: i16, midgame_eval: WhiteEval, endgame_eval: WhiteEval) -> WhiteEval {
    // Switch to 64 bit calculations to avoid overflow
    let phase_value = i64::from(phase_value);

    let midgame_phase_value = phase_value.min(PHASE_COUNT_MAX);
    let endgame_phase_value = PHASE_COUNT_MAX - phase_value;

    let midgame_eval = i64::from(midgame_eval.0);
    let endgame_eval = i64::from(endgame_eval.0);

    let eval = (midgame_eval * midgame_phase_value + endgame_eval * endgame_phase_value) / 24;
    WhiteEval(i16::try_from(eval).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_calculation_does_not_overflow() {
        let midgame_eval = WhiteEval(2456);
        let endgame_eval = WhiteEval(2393);
        let phase_value = 9;

        let eval = taper(phase_value, midgame_eval, endgame_eval);
        assert!(eval > WhiteEval(0));
    }
}
