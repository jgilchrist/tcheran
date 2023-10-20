#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub const N: usize = 2;

    #[must_use]
    pub const fn other(&self) -> Self {
        match *self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    #[must_use]
    pub fn array_idx(&self) -> usize {
        *self as usize
    }
}
