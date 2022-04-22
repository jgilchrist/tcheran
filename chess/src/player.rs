#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn other(&self) -> Player {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}
