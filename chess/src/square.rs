#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub struct Square(pub File, pub Rank);

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Square(file, rank)
    }

    pub fn from_idx(idx: u8) -> Square {
        let file_idx = idx / 8;
        let rank_idx = idx % 8;
        Square(File::from_idx(file_idx), Rank::from_idx(rank_idx))
    }

    pub fn from_idxs(file_idx: u8, rank_idx: u8) -> Square {
        Square(File::from_idx(file_idx), Rank::from_idx(rank_idx))
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.0, self.1)
    }

    // For convenience
    pub const A1: Square = Square(File::A, Rank::R1);
    pub const A2: Square = Square(File::A, Rank::R2);
    pub const A3: Square = Square(File::A, Rank::R3);
    pub const A4: Square = Square(File::A, Rank::R4);
    pub const A5: Square = Square(File::A, Rank::R5);
    pub const A6: Square = Square(File::A, Rank::R6);
    pub const A7: Square = Square(File::A, Rank::R7);
    pub const A8: Square = Square(File::A, Rank::R8);

    pub const B1: Square = Square(File::B, Rank::R1);
    pub const B2: Square = Square(File::B, Rank::R2);
    pub const B3: Square = Square(File::B, Rank::R3);
    pub const B4: Square = Square(File::B, Rank::R4);
    pub const B5: Square = Square(File::B, Rank::R5);
    pub const B6: Square = Square(File::B, Rank::R6);
    pub const B7: Square = Square(File::B, Rank::R7);
    pub const B8: Square = Square(File::B, Rank::R8);

    pub const C1: Square = Square(File::C, Rank::R1);
    pub const C2: Square = Square(File::C, Rank::R2);
    pub const C3: Square = Square(File::C, Rank::R3);
    pub const C4: Square = Square(File::C, Rank::R4);
    pub const C5: Square = Square(File::C, Rank::R5);
    pub const C6: Square = Square(File::C, Rank::R6);
    pub const C7: Square = Square(File::C, Rank::R7);
    pub const C8: Square = Square(File::C, Rank::R8);

    pub const D1: Square = Square(File::D, Rank::R1);
    pub const D2: Square = Square(File::D, Rank::R2);
    pub const D3: Square = Square(File::D, Rank::R3);
    pub const D4: Square = Square(File::D, Rank::R4);
    pub const D5: Square = Square(File::D, Rank::R5);
    pub const D6: Square = Square(File::D, Rank::R6);
    pub const D7: Square = Square(File::D, Rank::R7);
    pub const D8: Square = Square(File::D, Rank::R8);

    pub const E1: Square = Square(File::E, Rank::R1);
    pub const E2: Square = Square(File::E, Rank::R2);
    pub const E3: Square = Square(File::E, Rank::R3);
    pub const E4: Square = Square(File::E, Rank::R4);
    pub const E5: Square = Square(File::E, Rank::R5);
    pub const E6: Square = Square(File::E, Rank::R6);
    pub const E7: Square = Square(File::E, Rank::R7);
    pub const E8: Square = Square(File::E, Rank::R8);

    pub const F1: Square = Square(File::F, Rank::R1);
    pub const F2: Square = Square(File::F, Rank::R2);
    pub const F3: Square = Square(File::F, Rank::R3);
    pub const F4: Square = Square(File::F, Rank::R4);
    pub const F5: Square = Square(File::F, Rank::R5);
    pub const F6: Square = Square(File::F, Rank::R6);
    pub const F7: Square = Square(File::F, Rank::R7);
    pub const F8: Square = Square(File::F, Rank::R8);

    pub const G1: Square = Square(File::G, Rank::R1);
    pub const G2: Square = Square(File::G, Rank::R2);
    pub const G3: Square = Square(File::G, Rank::R3);
    pub const G4: Square = Square(File::G, Rank::R4);
    pub const G5: Square = Square(File::G, Rank::R5);
    pub const G6: Square = Square(File::G, Rank::R6);
    pub const G7: Square = Square(File::G, Rank::R7);
    pub const G8: Square = Square(File::G, Rank::R8);

    pub const H1: Square = Square(File::H, Rank::R1);
    pub const H2: Square = Square(File::H, Rank::R2);
    pub const H3: Square = Square(File::H, Rank::R3);
    pub const H4: Square = Square(File::H, Rank::R4);
    pub const H5: Square = Square(File::H, Rank::R5);
    pub const H6: Square = Square(File::H, Rank::R6);
    pub const H7: Square = Square(File::H, Rank::R7);
    pub const H8: Square = Square(File::H, Rank::R8);
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
