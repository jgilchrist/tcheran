use chess::player::Player;

use crate::eval::Eval;

/// The standard `Eval` struct contains a value that is always from white's
/// perspective - i.e. positive if white is winning and negative if black is
/// winning.
/// When running a negamax search, in order for the same code to work for both
/// players, we need both players to try 'maximising' their score - so when black
/// is playing, we need positive values when black is winning.
/// However, when reporting results from Negamax, it can be confusing to see
/// evaluation numbers that are positive when black is winning.
/// `NegamaxEval` represents an evaluation from the perspective of a particular
/// player to avoid issues here. It can be easily constructed from an `Eval`
/// (by supplying the player whose perspective it will be from) and can easily
/// be turned back into an `Eval` for reporting or storage.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NegamaxEval(pub i32);

impl NegamaxEval {
    pub(crate) const MAX: Self = Self(i32::MAX);
    pub(crate) const MIN: Self = Self(i32::MIN);
    pub(crate) const DRAW: Self = Self(0);

    const MATE: i32 = 32000;

    pub fn mate_in(ply: u8) -> Self {
        Self(Self::MATE - i32::from(ply))
    }

    pub fn mated_in(ply: u8) -> Self {
        Self(-Self::MATE + i32::from(ply))
    }

    // TODO: We say 'any rating within 100 of 32000 is still mate in x moves'
    // Can we make this less arbitrary?
    pub fn is_mate_in_moves(self) -> Option<i32> {
        if self.0 > 32000 - 100 {
            return Some((Self::MATE - self.0 + 1) / 2);
        }

        if self.0 < -32000 + 100 {
            return Some((-Self::MATE - self.0) / 2);
        }

        None
    }

    // For negamax to work, the eval must be flipped for each side so that each side can
    // 'maximise' its score. However, this means when we report the eval, it will be reversed
    // if we're playing as black since we need the scores to be positive if black is winning.
    // When reporting this back, we'll want to normalise back to + = white winning.
    pub fn to_eval(self, player: Player) -> Eval {
        match player {
            Player::White => Eval(self.0),
            Player::Black => -Eval(self.0),
        }
    }

    pub fn from_eval(eval: Eval, player: Player) -> Self {
        match player {
            Player::White => Self(eval.0),
            Player::Black => Self((-eval).0),
        }
    }
}

impl std::ops::Add for NegamaxEval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for NegamaxEval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i32> for NegamaxEval {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Neg for NegamaxEval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.saturating_neg())
    }
}
