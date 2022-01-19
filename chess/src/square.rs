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

pub struct Square(pub File, pub Rank);

impl Square {
    pub fn from_idx(file_idx: u8, rank_idx: u8) -> Square {
        Square(File::from_idx(file_idx), Rank::from_idx(rank_idx))
    }
}
