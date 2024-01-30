use crate::chess::board::Board;
use crate::chess::piece::PieceKind;
use crate::chess::square::Square;
use crate::engine::eval::WhiteEval;

/// A midgame and endgame evaluation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhasedEval(i16, i16);

impl PhasedEval {
    pub const fn new(midgame: i16, endgame: i16) -> Self {
        Self(midgame, endgame)
    }

    pub fn midgame(self) -> WhiteEval {
        WhiteEval(self.0)
    }

    pub fn endgame(self) -> WhiteEval {
        WhiteEval(self.1)
    }
}

impl std::ops::Add for PhasedEval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for PhasedEval {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Sub for PhasedEval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign for PhasedEval {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl std::ops::Mul<i16> for PhasedEval {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Neg for PhasedEval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.saturating_neg(), self.1.saturating_neg())
    }
}

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

pub fn taper(phase_value: i16, eval: PhasedEval) -> WhiteEval {
    // Switch to 64 bit calculations to avoid overflow
    let phase_value = i64::from(phase_value);

    let midgame_phase_value = phase_value.min(PHASE_COUNT_MAX);
    let endgame_phase_value = PHASE_COUNT_MAX - phase_value;

    let midgame_eval = i64::from(eval.0);
    let endgame_eval = i64::from(eval.1);

    let eval = (midgame_eval * midgame_phase_value + endgame_eval * endgame_phase_value) / 24;
    WhiteEval(i16::try_from(eval).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_calculation_does_not_overflow() {
        let phased_eval = PhasedEval(2456, 2393);
        let phase_value = 9;

        let eval = taper(phase_value, phased_eval);
        assert!(eval > WhiteEval(0));
    }
}
