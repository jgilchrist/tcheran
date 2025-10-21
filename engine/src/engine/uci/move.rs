use crate::chess::{moves::Move, piece::PromotionPieceKind, square::Square};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct UciMove {
    pub src: Square,
    pub dst: Square,
    pub promotion: Option<PromotionPieceKind>,
}

impl UciMove {
    pub fn notation(self) -> String {
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

impl std::fmt::Debug for UciMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl std::fmt::Display for UciMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl From<Move> for UciMove {
    fn from(value: Move) -> Self {
        Self {
            src: value.src(),
            dst: value.dst(),
            promotion: value.promotion(),
        }
    }
}
