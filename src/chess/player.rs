#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub const N: usize = 2;

    pub fn other(self) -> Self {
        !self
    }

    #[inline(always)]
    pub const fn array_idx(self) -> usize {
        self as usize
    }
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

pub trait PlayerT {
    type Other: PlayerT;

    const PLAYER: Player;
    const IS_WHITE: bool;

    const IDX: usize = Self::PLAYER.array_idx();
}

pub struct White;

impl PlayerT for White {
    type Other = Black;
    const PLAYER: Player = Player::White;
    const IS_WHITE: bool = true;
}

pub struct Black;

impl PlayerT for Black {
    type Other = White;
    const PLAYER: Player = Player::Black;
    const IS_WHITE: bool = false;
}
