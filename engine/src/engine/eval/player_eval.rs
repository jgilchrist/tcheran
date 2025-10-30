use std::ops::Div;

use crate::{chess::player::Player, engine::eval::WhiteEval};

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
pub struct Eval(pub i32);

impl Eval {
    pub(crate) const MAX: Self = Self(i16::MAX as i32);
    pub(crate) const MIN: Self = Self(i16::MIN as i32 + 1);
    pub(crate) const NONE: Self = Self(i16::MIN as i32);
    pub(crate) const DRAW: Self = Self(0);

    const MATE: i32 = 32000;
    const MATED: i32 = -Self::MATE;
    const MATE_THRESHOLD: i32 = Self::MATE - 100;
    const MATED_THRESHOLD: i32 = -Self::MATE_THRESHOLD;

    pub const fn new(eval: i32) -> Self {
        Self(eval)
    }

    pub fn mate_in(ply: u8) -> Self {
        Self(Self::MATE - i32::from(ply))
    }

    pub fn mated_in(ply: u8) -> Self {
        Self(-Self::MATE + i32::from(ply))
    }

    #[inline]
    pub fn mating(self) -> bool {
        self.0 >= Self::MATE_THRESHOLD
    }

    #[inline]
    pub fn being_mated(self) -> bool {
        self.0 <= Self::MATED_THRESHOLD
    }

    pub fn is_mate_in_moves(self) -> Option<i32> {
        if self.mating() {
            return Some((Self::MATE - self.0 + 1) / 2);
        }

        if self.being_mated() {
            return Some((Self::MATED - self.0) / 2);
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

impl std::ops::AddAssign for Eval {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Eval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Eval {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<i32> for Eval {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Neg for Eval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.saturating_neg())
    }
}

impl Div<i32> for Eval {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}
