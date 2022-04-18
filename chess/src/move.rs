use crate::square::Square;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    src: Square,
    dst: Square,
}

impl Move {
    pub fn new(src: Square, dst: Square) -> Move {
        Move { src, dst }
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.src.notation(), self.dst.notation())
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}
