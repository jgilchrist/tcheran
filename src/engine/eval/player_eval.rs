use crate::chess::player::Player;

use crate::engine::eval::WhiteEval;

/// An evaluation from the active player's perspective
///
/// In algorithms like negamax, in order for the same code to work
/// for both players, we need to players to try maximising their score.
/// Therefore, we need a way to represent an evaluation of the board as
/// seen by the active player in any given game state.
///
/// This can be easily turned back into a 'classical' evaluation (i.e.
/// from white's perspective).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Eval(pub i16);

impl Eval {
    pub(crate) const MAX: Self = Self(i16::MAX);
    pub(crate) const MIN: Self = Self(i16::MIN);
    pub(crate) const DRAW: Self = Self(0);

    const MATE: i16 = 32000;
    const MATE_THRESHOLD: i16 = Self::MATE - 100;

    #[cfg(test)]
    pub fn mate_in(ply: u8) -> Self {
        Self(Self::MATE - i16::from(ply))
    }

    pub fn mated_in(ply: u8) -> Self {
        Self(-Self::MATE + i16::from(ply))
    }

    pub fn is_mate_in_moves(self) -> Option<i16> {
        if self.0 > Self::MATE_THRESHOLD {
            return Some((Self::MATE - self.0 + 1) / 2);
        }

        if self.0 < -Self::MATE_THRESHOLD {
            return Some((-Self::MATE - self.0) / 2);
        }

        None
    }

    pub fn to_white_eval(self, player: Player) -> WhiteEval {
        match player {
            Player::White => WhiteEval(self.0),
            Player::Black => -WhiteEval(self.0),
        }
    }

    pub fn from_white_eval(eval: WhiteEval, player: Player) -> Self {
        match player {
            Player::White => Self(eval.0),
            Player::Black => Self((-eval).0),
        }
    }
}

impl std::ops::Add for Eval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Eval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i16> for Eval {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Neg for Eval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.saturating_neg())
    }
}
