use crate::{bitboard::Bitboard, direction::Direction};

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
pub struct Square(pub u64);

impl Square {
    pub const fn new(file: File, rank: Rank) -> Self {
        Square::from_idxs(file.idx(), rank.idx())
    }

    pub const fn from_bitboard(bitboard: &Bitboard) -> Square {
        debug_assert!(bitboard.count() == 1);
        Square(bitboard.0)
    }

    pub const fn from_idxs(file_idx: u8, rank_idx: u8) -> Square {
        Square(1 << (rank_idx * 8 + file_idx))
    }

    #[inline(always)]
    fn rank(&self) -> Rank {
        Rank::from_idx((self.0.trailing_zeros() / 8) as u8)
    }

    #[inline(always)]
    fn file(&self) -> File {
        File::from_idx((self.0.trailing_zeros() % 8) as u8)
    }

    // TODO: Remove this
    pub fn bitboard(&self) -> Bitboard {
        Bitboard::new(self.0)
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.file(), self.rank())
    }

    #[inline(always)]
    pub fn in_direction(&self, direction: &Direction) -> Option<Square> {
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
    pub fn north(&self) -> Option<Square> {
        self.bitboard().north().to_square()
    }

    #[inline(always)]
    pub fn south(&self) -> Option<Square> {
        self.bitboard().south().to_square()
    }

    #[inline(always)]
    pub fn east(&self) -> Option<Square> {
        self.bitboard().east().to_square()
    }

    #[inline(always)]
    pub fn north_east(&self) -> Option<Square> {
        self.bitboard().north_east().to_square()
    }

    #[inline(always)]
    pub fn south_east(&self) -> Option<Square> {
        self.bitboard().south_east().to_square()
    }

    #[inline(always)]
    pub fn west(&self) -> Option<Square> {
        self.bitboard().west().to_square()
    }

    #[inline(always)]
    pub fn south_west(&self) -> Option<Square> {
        self.bitboard().south_west().to_square()
    }

    #[inline(always)]
    pub fn north_west(&self) -> Option<Square> {
        self.bitboard().north_west().to_square()
    }
}

pub mod squares {
    use crate::player::Player;

    use super::{File, Rank, Square};

    // For convenience
    pub const A1: Square = Square::new(File::A, Rank::R1);
    pub const A2: Square = Square::new(File::A, Rank::R2);
    pub const A3: Square = Square::new(File::A, Rank::R3);
    pub const A4: Square = Square::new(File::A, Rank::R4);
    pub const A5: Square = Square::new(File::A, Rank::R5);
    pub const A6: Square = Square::new(File::A, Rank::R6);
    pub const A7: Square = Square::new(File::A, Rank::R7);
    pub const A8: Square = Square::new(File::A, Rank::R8);

    pub const B1: Square = Square::new(File::B, Rank::R1);
    pub const B2: Square = Square::new(File::B, Rank::R2);
    pub const B3: Square = Square::new(File::B, Rank::R3);
    pub const B4: Square = Square::new(File::B, Rank::R4);
    pub const B5: Square = Square::new(File::B, Rank::R5);
    pub const B6: Square = Square::new(File::B, Rank::R6);
    pub const B7: Square = Square::new(File::B, Rank::R7);
    pub const B8: Square = Square::new(File::B, Rank::R8);

    pub const C1: Square = Square::new(File::C, Rank::R1);
    pub const C2: Square = Square::new(File::C, Rank::R2);
    pub const C3: Square = Square::new(File::C, Rank::R3);
    pub const C4: Square = Square::new(File::C, Rank::R4);
    pub const C5: Square = Square::new(File::C, Rank::R5);
    pub const C6: Square = Square::new(File::C, Rank::R6);
    pub const C7: Square = Square::new(File::C, Rank::R7);
    pub const C8: Square = Square::new(File::C, Rank::R8);

    pub const D1: Square = Square::new(File::D, Rank::R1);
    pub const D2: Square = Square::new(File::D, Rank::R2);
    pub const D3: Square = Square::new(File::D, Rank::R3);
    pub const D4: Square = Square::new(File::D, Rank::R4);
    pub const D5: Square = Square::new(File::D, Rank::R5);
    pub const D6: Square = Square::new(File::D, Rank::R6);
    pub const D7: Square = Square::new(File::D, Rank::R7);
    pub const D8: Square = Square::new(File::D, Rank::R8);

    pub const E1: Square = Square::new(File::E, Rank::R1);
    pub const E2: Square = Square::new(File::E, Rank::R2);
    pub const E3: Square = Square::new(File::E, Rank::R3);
    pub const E4: Square = Square::new(File::E, Rank::R4);
    pub const E5: Square = Square::new(File::E, Rank::R5);
    pub const E6: Square = Square::new(File::E, Rank::R6);
    pub const E7: Square = Square::new(File::E, Rank::R7);
    pub const E8: Square = Square::new(File::E, Rank::R8);

    pub const F1: Square = Square::new(File::F, Rank::R1);
    pub const F2: Square = Square::new(File::F, Rank::R2);
    pub const F3: Square = Square::new(File::F, Rank::R3);
    pub const F4: Square = Square::new(File::F, Rank::R4);
    pub const F5: Square = Square::new(File::F, Rank::R5);
    pub const F6: Square = Square::new(File::F, Rank::R6);
    pub const F7: Square = Square::new(File::F, Rank::R7);
    pub const F8: Square = Square::new(File::F, Rank::R8);

    pub const G1: Square = Square::new(File::G, Rank::R1);
    pub const G2: Square = Square::new(File::G, Rank::R2);
    pub const G3: Square = Square::new(File::G, Rank::R3);
    pub const G4: Square = Square::new(File::G, Rank::R4);
    pub const G5: Square = Square::new(File::G, Rank::R5);
    pub const G6: Square = Square::new(File::G, Rank::R6);
    pub const G7: Square = Square::new(File::G, Rank::R7);
    pub const G8: Square = Square::new(File::G, Rank::R8);

    pub const H1: Square = Square::new(File::H, Rank::R1);
    pub const H2: Square = Square::new(File::H, Rank::R2);
    pub const H3: Square = Square::new(File::H, Rank::R3);
    pub const H4: Square = Square::new(File::H, Rank::R4);
    pub const H5: Square = Square::new(File::H, Rank::R5);
    pub const H6: Square = Square::new(File::H, Rank::R6);
    pub const H7: Square = Square::new(File::H, Rank::R7);
    pub const H8: Square = Square::new(File::H, Rank::R8);

    pub fn king_start(player: &Player) -> &Square {
        player_square(player, &E1, &E8)
    }

    pub fn kingside_rook_start(player: &Player) -> &Square {
        player_square(player, &H1, &H8)
    }

    pub fn queenside_rook_start(player: &Player) -> &Square {
        player_square(player, &A1, &A8)
    }

    pub fn kingside_castle_dest(player: &Player) -> &Square {
        player_square(player, &G1, &G8)
    }

    pub fn queenside_castle_dest(player: &Player) -> &Square {
        player_square(player, &C1, &C8)
    }

    fn player_square<'a>(
        player: &Player,
        white_square: &'a Square,
        black_square: &'a Square,
    ) -> &'a Square {
        match player {
            Player::White => white_square,
            Player::Black => black_square,
        }
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
