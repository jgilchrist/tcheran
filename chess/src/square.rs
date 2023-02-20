use crate::{bitboard::Bitboard, direction::Direction};

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
    pub fn from_idx(idx: u8) -> File {
        debug_assert!(idx < 8);

        match idx {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    pub const fn idx(&self) -> u8 {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }

    pub fn notation(&self) -> &str {
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
    pub fn from_idx(idx: u8) -> Rank {
        debug_assert!(idx < 8);

        match idx {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => unreachable!(),
        }
    }

    pub const fn idx(&self) -> u8 {
        match self {
            Rank::R1 => 0,
            Rank::R2 => 1,
            Rank::R3 => 2,
            Rank::R4 => 3,
            Rank::R5 => 4,
            Rank::R6 => 5,
            Rank::R7 => 6,
            Rank::R8 => 7,
        }
    }

    pub fn notation(&self) -> &str {
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
    pub const fn from_file_and_rank(file: File, rank: Rank) -> Self {
        Self::from_idxs(file.idx(), rank.idx())
    }

    pub fn from_bitboard(bitboard: Bitboard) -> Self {
        assert_eq!(bitboard.count(), 1);
        Self(bitboard)
    }

    fn from_bitboard_maybe(bitboard: Bitboard) -> Option<Self> {
        match bitboard.count() {
            0 => None,
            1 => Some(Self(bitboard)),
            _ => panic!("Should have had 0 or 1 bits in bitboard."),
        }
    }

    pub const fn from_index(idx: u8) -> Self {
        Self(Bitboard::new(1 << idx))
    }

    pub const fn from_idxs(file_idx: u8, rank_idx: u8) -> Self {
        let idx = rank_idx * 8 + file_idx;
        Self::from_index(idx)
    }

    #[inline(always)]
    fn rank(&self) -> Rank {
        Rank::from_idx(self.0.trailing_zeros() / 8)
    }

    #[inline(always)]
    fn file(&self) -> File {
        File::from_idx(self.0.trailing_zeros() % 8)
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.file(), self.rank())
    }

    #[inline(always)]
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
    pub fn north(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.north())
    }

    #[inline(always)]
    pub fn south(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south())
    }

    #[inline(always)]
    pub fn east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.east())
    }

    #[inline(always)]
    pub fn north_east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.north_east())
    }

    #[inline(always)]
    pub fn south_east(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south_east())
    }

    #[inline(always)]
    pub fn west(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.west())
    }

    #[inline(always)]
    pub fn south_west(&self) -> Option<Self> {
        Self::from_bitboard_maybe(self.0.south_west())
    }

    #[inline(always)]
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
}
