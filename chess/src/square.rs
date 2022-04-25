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

    pub fn idx(&self) -> u8 {
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

    pub fn idx(&self) -> u8 {
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

// TODO: Change internal representation to u8 (index)
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Square { file, rank }
    }

    pub fn from_idx(idx: u8) -> Square {
        let file_idx = idx % 8;
        let rank_idx = idx / 8;

        Square {
            file: File::from_idx(file_idx),
            rank: Rank::from_idx(rank_idx),
        }
    }

    pub fn from_idxs(file_idx: u8, rank_idx: u8) -> Square {
        Square {
            file: File::from_idx(file_idx),
            rank: Rank::from_idx(rank_idx),
        }
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.file, self.rank)
    }

    // PERF: Continuous conversions to and from bitboard may have a perf impact?
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

    pub fn north(&self) -> Option<Square> {
        Bitboard::from_square(self).north().to_square()
    }

    pub fn south(&self) -> Option<Square> {
        Bitboard::from_square(self).south().to_square()
    }

    pub fn east(&self) -> Option<Square> {
        Bitboard::from_square(self).east().to_square()
    }

    pub fn north_east(&self) -> Option<Square> {
        Bitboard::from_square(self).north_east().to_square()
    }

    pub fn south_east(&self) -> Option<Square> {
        Bitboard::from_square(self).south_east().to_square()
    }

    pub fn west(&self) -> Option<Square> {
        Bitboard::from_square(self).west().to_square()
    }

    pub fn south_west(&self) -> Option<Square> {
        Bitboard::from_square(self).south_west().to_square()
    }

    pub fn north_west(&self) -> Option<Square> {
        Bitboard::from_square(self).north_west().to_square()
    }

    // For convenience
    pub const A1: Square = Square {
        file: File::A,
        rank: Rank::R1,
    };
    pub const A2: Square = Square {
        file: File::A,
        rank: Rank::R2,
    };
    pub const A3: Square = Square {
        file: File::A,
        rank: Rank::R3,
    };
    pub const A4: Square = Square {
        file: File::A,
        rank: Rank::R4,
    };
    pub const A5: Square = Square {
        file: File::A,
        rank: Rank::R5,
    };
    pub const A6: Square = Square {
        file: File::A,
        rank: Rank::R6,
    };
    pub const A7: Square = Square {
        file: File::A,
        rank: Rank::R7,
    };
    pub const A8: Square = Square {
        file: File::A,
        rank: Rank::R8,
    };

    pub const B1: Square = Square {
        file: File::B,
        rank: Rank::R1,
    };
    pub const B2: Square = Square {
        file: File::B,
        rank: Rank::R2,
    };
    pub const B3: Square = Square {
        file: File::B,
        rank: Rank::R3,
    };
    pub const B4: Square = Square {
        file: File::B,
        rank: Rank::R4,
    };
    pub const B5: Square = Square {
        file: File::B,
        rank: Rank::R5,
    };
    pub const B6: Square = Square {
        file: File::B,
        rank: Rank::R6,
    };
    pub const B7: Square = Square {
        file: File::B,
        rank: Rank::R7,
    };
    pub const B8: Square = Square {
        file: File::B,
        rank: Rank::R8,
    };

    pub const C1: Square = Square {
        file: File::C,
        rank: Rank::R1,
    };
    pub const C2: Square = Square {
        file: File::C,
        rank: Rank::R2,
    };
    pub const C3: Square = Square {
        file: File::C,
        rank: Rank::R3,
    };
    pub const C4: Square = Square {
        file: File::C,
        rank: Rank::R4,
    };
    pub const C5: Square = Square {
        file: File::C,
        rank: Rank::R5,
    };
    pub const C6: Square = Square {
        file: File::C,
        rank: Rank::R6,
    };
    pub const C7: Square = Square {
        file: File::C,
        rank: Rank::R7,
    };
    pub const C8: Square = Square {
        file: File::C,
        rank: Rank::R8,
    };

    pub const D1: Square = Square {
        file: File::D,
        rank: Rank::R1,
    };
    pub const D2: Square = Square {
        file: File::D,
        rank: Rank::R2,
    };
    pub const D3: Square = Square {
        file: File::D,
        rank: Rank::R3,
    };
    pub const D4: Square = Square {
        file: File::D,
        rank: Rank::R4,
    };
    pub const D5: Square = Square {
        file: File::D,
        rank: Rank::R5,
    };
    pub const D6: Square = Square {
        file: File::D,
        rank: Rank::R6,
    };
    pub const D7: Square = Square {
        file: File::D,
        rank: Rank::R7,
    };
    pub const D8: Square = Square {
        file: File::D,
        rank: Rank::R8,
    };

    pub const E1: Square = Square {
        file: File::E,
        rank: Rank::R1,
    };
    pub const E2: Square = Square {
        file: File::E,
        rank: Rank::R2,
    };
    pub const E3: Square = Square {
        file: File::E,
        rank: Rank::R3,
    };
    pub const E4: Square = Square {
        file: File::E,
        rank: Rank::R4,
    };
    pub const E5: Square = Square {
        file: File::E,
        rank: Rank::R5,
    };
    pub const E6: Square = Square {
        file: File::E,
        rank: Rank::R6,
    };
    pub const E7: Square = Square {
        file: File::E,
        rank: Rank::R7,
    };
    pub const E8: Square = Square {
        file: File::E,
        rank: Rank::R8,
    };

    pub const F1: Square = Square {
        file: File::F,
        rank: Rank::R1,
    };
    pub const F2: Square = Square {
        file: File::F,
        rank: Rank::R2,
    };
    pub const F3: Square = Square {
        file: File::F,
        rank: Rank::R3,
    };
    pub const F4: Square = Square {
        file: File::F,
        rank: Rank::R4,
    };
    pub const F5: Square = Square {
        file: File::F,
        rank: Rank::R5,
    };
    pub const F6: Square = Square {
        file: File::F,
        rank: Rank::R6,
    };
    pub const F7: Square = Square {
        file: File::F,
        rank: Rank::R7,
    };
    pub const F8: Square = Square {
        file: File::F,
        rank: Rank::R8,
    };

    pub const G1: Square = Square {
        file: File::G,
        rank: Rank::R1,
    };
    pub const G2: Square = Square {
        file: File::G,
        rank: Rank::R2,
    };
    pub const G3: Square = Square {
        file: File::G,
        rank: Rank::R3,
    };
    pub const G4: Square = Square {
        file: File::G,
        rank: Rank::R4,
    };
    pub const G5: Square = Square {
        file: File::G,
        rank: Rank::R5,
    };
    pub const G6: Square = Square {
        file: File::G,
        rank: Rank::R6,
    };
    pub const G7: Square = Square {
        file: File::G,
        rank: Rank::R7,
    };
    pub const G8: Square = Square {
        file: File::G,
        rank: Rank::R8,
    };

    pub const H1: Square = Square {
        file: File::H,
        rank: Rank::R1,
    };
    pub const H2: Square = Square {
        file: File::H,
        rank: Rank::R2,
    };
    pub const H3: Square = Square {
        file: File::H,
        rank: Rank::R3,
    };
    pub const H4: Square = Square {
        file: File::H,
        rank: Rank::R4,
    };
    pub const H5: Square = Square {
        file: File::H,
        rank: Rank::R5,
    };
    pub const H6: Square = Square {
        file: File::H,
        rank: Rank::R6,
    };
    pub const H7: Square = Square {
        file: File::H,
        rank: Rank::R7,
    };
    pub const H8: Square = Square {
        file: File::H,
        rank: Rank::R8,
    };
}

pub mod known {
    use super::Square;

    pub const WHITE_KING_START: &Square = &Square::E1;
    pub const BLACK_KING_START: &Square = &Square::E8;

    pub const WHITE_KINGSIDE_ROOK_START: &Square = &Square::H1;
    pub const BLACK_KINGSIDE_ROOK_START: &Square = &Square::H8;

    pub const WHITE_QUEENSIDE_ROOK_START: &Square = &Square::A1;
    pub const BLACK_QUEENSIDE_ROOK_START: &Square = &Square::A8;

    pub const WHITE_KINGSIDE_CASTLE: &Square = &Square::G1;
    pub const BLACK_KINGSIDE_CASTLE: &Square = &Square::G8;

    pub const WHITE_QUEENSIDE_CASTLE: &Square = &Square::C1;
    pub const BLACK_QUEENSIDE_CASTLE: &Square = &Square::C8;
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
