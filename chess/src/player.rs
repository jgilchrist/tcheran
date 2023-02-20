#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

impl Player {
    #[must_use]
    pub const fn other(&self) -> Self {
        match *self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
