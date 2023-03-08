use crate::{bitboard::Bitboard, direction::Direction, squares::Squares};

pub const FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    #[must_use]
    pub fn from_idx(idx: u8) -> Self {
        debug_assert!(idx < 8);

        match idx {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn idx(&self) -> u8 {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
            Self::H => 7,
        }
    }

    #[must_use]
    pub const fn notation(&self) -> &str {
        match self {
            Self::A => "a",
            Self::B => "b",
            Self::C => "c",
            Self::D => "d",
            Self::E => "e",
            Self::F => "f",
            Self::G => "g",
            Self::H => "h",
        }
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

pub const RANKS: [Rank; 8] = [
    Rank::R1,
    Rank::R2,
    Rank::R3,
    Rank::R4,
    Rank::R5,
    Rank::R6,
    Rank::R7,
    Rank::R8,
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    #[must_use]
    pub fn from_idx(idx: u8) -> Self {
        debug_assert!(idx < 8);

        match idx {
            0 => Self::R1,
            1 => Self::R2,
            2 => Self::R3,
            3 => Self::R4,
            4 => Self::R5,
            5 => Self::R6,
            6 => Self::R7,
            7 => Self::R8,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn idx(&self) -> u8 {
        match self {
            Self::R1 => 0,
            Self::R2 => 1,
            Self::R3 => 2,
            Self::R4 => 3,
            Self::R5 => 4,
            Self::R6 => 5,
            Self::R7 => 6,
            Self::R8 => 7,
        }
    }

    #[must_use]
    pub const fn notation(&self) -> &str {
        match self {
            Self::R1 => "1",
            Self::R2 => "2",
            Self::R3 => "3",
            Self::R4 => "4",
            Self::R5 => "5",
            Self::R6 => "6",
            Self::R7 => "7",
            Self::R8 => "8",
        }
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Square(pub Bitboard);

impl Square {
    #[must_use]
    pub const fn from_file_and_rank(file: File, rank: Rank) -> Self {
        Self::from_idxs(file.idx(), rank.idx())
    }

    fn from_bitboard_maybe(bitboard: Bitboard) -> Option<Self> {
        match bitboard.count() {
            0 => None,
            1 => Some(Self(bitboard)),
            _ => panic!("Should have had 0 or 1 bits in bitboard."),
        }
    }

    #[must_use]
    pub const fn from_index(idx: u8) -> Self {
        Self(Bitboard::new(1 << idx))
    }

    #[must_use]
    pub const fn from_idxs(file_idx: u8, rank_idx: u8) -> Self {
        let idx = rank_idx * 8 + file_idx;
        Self::from_index(idx)
    }

    #[must_use]
    pub const fn idx(&self) -> u8 {
        self.0.trailing_zeros()
    }

    #[inline(always)]
    #[must_use]
    pub fn rank(self) -> Rank {
        Rank::from_idx(self.idx() / 8)
    }

    #[inline(always)]
    #[must_use]
    pub fn file(self) -> File {
        File::from_idx(self.idx() % 8)
    }

    #[must_use]
    pub fn notation(&self) -> String {
        format!("{}{}", self.file(), self.rank())
    }

    #[inline(always)]
    #[must_use]
    pub fn in_direction(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::North => self.north(),
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn north(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.north())
    }

    #[inline(always)]
    #[must_use]
    pub fn south(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south())
    }

    #[inline(always)]
    #[must_use]
    pub fn east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.east())
    }

    #[inline(always)]
    #[must_use]
    pub fn north_east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.north_east())
    }

    #[inline(always)]
    #[must_use]
    pub fn south_east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south_east())
    }

    #[inline(always)]
    #[must_use]
    pub fn west(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.west())
    }

    #[inline(always)]
    #[must_use]
    pub fn south_west(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south_west())
    }

    #[inline(always)]
    #[must_use]
    pub fn north_west(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.north_west())
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl const std::ops::BitOr for Square {
    type Output = Squares;

    fn bitor(self, rhs: Self) -> Self::Output {
        Squares::from_bitboard(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::squares::all::*;

    #[test]
    fn square_from_index() {
        assert_eq!(Square::from_index(0), A1);
        assert_eq!(Square::from_index(63), H8);
    }

    #[test]
    fn square_from_idxs() {
        assert_eq!(Square::from_idxs(0, 0), A1);
        assert_eq!(Square::from_idxs(7, 7), H8);
    }

    #[test]
    fn square_from_file_and_rank() {
        assert_eq!(Square::from_file_and_rank(File::A, Rank::R1), A1);
        assert_eq!(Square::from_file_and_rank(File::H, Rank::R8), H8);
    }

    #[test]
    fn square_size() {
        assert_eq!(
            std::mem::size_of::<Square>(),
            std::mem::size_of::<Bitboard>()
        );
    }
}
