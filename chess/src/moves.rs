use crate::{piece::PromotionPieceKind, square::Square};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub dst: Square,
    pub promotion: Option<PromotionPieceKind>,
}

impl Move {
    #[must_use]
    pub const fn new(src: Square, dst: Square) -> Self {
        Self {
            src,
            dst,
            promotion: None,
        }
    }

    #[must_use]
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

    #[must_use]
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

pub mod known {
    use super::Move;
    use crate::{player::Player, squares};

    #[must_use]
    pub const fn kingside_castle_move(player: Player) -> &'static Move {
        match player {
            Player::White => &WHITE_KINGSIDE_CASTLE_MOVE,
            Player::Black => &BLACK_KINGSIDE_CASTLE_MOVE,
        }
    }

    #[must_use]
    pub const fn queenside_castle_move(player: Player) -> &'static Move {
        match player {
            Player::White => &WHITE_QUEENSIDE_CASTLE_MOVE,
            Player::Black => &BLACK_QUEENSIDE_CASTLE_MOVE,
        }
    }

    pub const WHITE_KINGSIDE_CASTLE_MOVE: Move = Move::new(
        squares::INIT_WHITE_KING,
        squares::WHITE_KINGSIDE_CASTLE_SQUARE,
    );
    pub const WHITE_QUEENSIDE_CASTLE_MOVE: Move = Move::new(
        squares::INIT_WHITE_KING,
        squares::WHITE_QUEENSIDE_CASTLE_SQUARE,
    );
    pub const BLACK_KINGSIDE_CASTLE_MOVE: Move = Move::new(
        squares::INIT_BLACK_KING,
        squares::BLACK_KINGSIDE_CASTLE_SQUARE,
    );
    pub const BLACK_QUEENSIDE_CASTLE_MOVE: Move = Move::new(
        squares::INIT_BLACK_KING,
        squares::BLACK_QUEENSIDE_CASTLE_SQUARE,
    );
}
