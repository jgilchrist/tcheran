use crate::chess::{piece::PromotionPieceKind, square::Square};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub dst: Square,
    pub promotion: Option<PromotionPieceKind>,

    pub is_capture: bool,
    pub is_en_passant: bool,
    pub is_castling: bool,
}

impl Move {
    pub const fn quiet(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,

            is_capture: false,
            is_en_passant: false,
            is_castling: false,
        }
    }

    pub const fn capture(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,

            is_capture: true,

            is_en_passant: false,
            is_castling: false,
        }
    }

    pub const fn castles(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,

            is_castling: true,

            is_capture: false,
            is_en_passant: false,
        }
    }

    pub const fn en_passant(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,

            is_capture: true,
            is_en_passant: true,

            is_castling: false,
        }
    }

    pub const fn quiet_promotion(src: Square, dst: Square, promotion: PromotionPieceKind) -> Self {
        Self {
            src,
            dst,
            promotion: Some(promotion),

            is_capture: false,

            is_en_passant: false,
            is_castling: false,
        }
    }

    pub const fn capture_promotion(
        src: Square,
        dst: Square,
        promotion: PromotionPieceKind,
    ) -> Self {
        Self {
            src,
            dst,
            promotion: Some(promotion),

            is_capture: true,

            is_en_passant: false,
            is_castling: false,
        }
    }
}

// impl From<(Square, Square)> for Move {
//     fn from((src, dst): (Square, Square)) -> Self {
//         Self::new(src, dst)
//     }
// }

// impl From<(Square, Square, PromotionPieceKind)> for Move {
//     fn from((src, dst, promotion): (Square, Square, PromotionPieceKind)) -> Self {
//         Self::promotion(src, dst, promotion)
//     }
// }

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
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
