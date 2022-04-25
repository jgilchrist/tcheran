use crate::{piece::PromotionPieceKind, square::Square};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub dst: Square,
    pub promotion: Option<PromotionPieceKind>,
}

impl Move {
    pub fn new(src: Square, dst: Square) -> Move {
        Move {
            src,
            dst,
            promotion: None,
        }
    }

    pub fn new_with_promotion(src: Square, dst: Square, promotion: PromotionPieceKind) -> Move {
        Move {
            src,
            dst,
            promotion: Some(promotion),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        self.src.rank != self.dst.rank && self.src.file != self.dst.file
    }

    pub fn distance(&self) -> u8 {
        self.src.rank.idx().abs_diff(self.dst.rank.idx())
            + self.src.file.idx().abs_diff(self.dst.rank.idx())
    }

    pub fn notation(&self) -> String {
        format!(
            "{}{}{}",
            self.src.notation(),
            self.dst.notation(),
            match self.promotion {
                Some(piece) => match piece {
                    PromotionPieceKind::Knight => "n",
                    PromotionPieceKind::Bishop => "b",
                    PromotionPieceKind::Rook => "r",
                    PromotionPieceKind::Queen => "q",
                },
                None => "",
            }
        )
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
