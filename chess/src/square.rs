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
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::H => write!(f, "H"),
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::H => write!(f, "H"),
        }
    }
}

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
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R1 => write!(f, "1"),
            Self::R2 => write!(f, "2"),
            Self::R3 => write!(f, "3"),
            Self::R4 => write!(f, "4"),
            Self::R5 => write!(f, "5"),
            Self::R6 => write!(f, "6"),
            Self::R7 => write!(f, "7"),
            Self::R8 => write!(f, "8"),
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R1 => write!(f, "1"),
            Self::R2 => write!(f, "2"),
            Self::R3 => write!(f, "3"),
            Self::R4 => write!(f, "4"),
            Self::R5 => write!(f, "5"),
            Self::R6 => write!(f, "6"),
            Self::R7 => write!(f, "7"),
            Self::R8 => write!(f, "8"),
        }
    }
}

#[derive(Debug)]
pub struct Square(pub File, pub Rank);

impl Square {
    pub fn from_idx(file_idx: u8, rank_idx: u8) -> Square {
        Square(File::from_idx(file_idx), Rank::from_idx(rank_idx))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
