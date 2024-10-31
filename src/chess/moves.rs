use crate::chess::{piece::PromotionPieceKind, square::Square};
use arrayvec::ArrayVec;

const MAX_LEGAL_MOVES: usize = 218;

pub type MoveList = ArrayVec<Move, MAX_LEGAL_MOVES>;

pub trait MoveListExt {
    fn expect_matching(
        &self,
        src: Square,
        dst: Square,
        promotion: Option<PromotionPieceKind>,
    ) -> Move;
}

impl MoveListExt for MoveList {
    fn expect_matching(
        &self,
        src: Square,
        dst: Square,
        promotion: Option<PromotionPieceKind>,
    ) -> Move {
        for i in 0..self.len() {
            let mv = *self.get(i).unwrap();

            if mv.src == src && mv.dst == dst && mv.promotion == promotion {
                return mv;
            }
        }

        panic!("Illegal move")
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub dst: Square,
    pub promotion: Option<PromotionPieceKind>,
}

impl Move {
    pub const fn new(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,
        }
    }

    pub const fn new_with_promotion(
        src: Square,
        dst: Square,
        promotion: PromotionPieceKind,
    ) -> Self {
        Self {
            src,
            dst,
            promotion: Some(promotion),
        }
    }
}

impl From<(Square, Square)> for Move {
    fn from((src, dst): (Square, Square)) -> Self {
        Self::new(src, dst)
    }
}

impl From<(Square, Square, PromotionPieceKind)> for Move {
    fn from((src, dst, promotion): (Square, Square, PromotionPieceKind)) -> Self {
        Self::new_with_promotion(src, dst, promotion)
    }
}

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
