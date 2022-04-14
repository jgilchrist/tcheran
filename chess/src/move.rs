use crate::square::Square;

#[derive(Debug, PartialEq, Eq)]
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
