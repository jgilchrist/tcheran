use crate::player::Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PromotionPieceKind {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPieceKind {
    pub const ALL: &'static [Self; 4] = &[Self::Knight, Self::Bishop, Self::Rook, Self::Queen];

    pub fn piece(&self) -> PieceKind {
        match self {
            PromotionPieceKind::Knight => PieceKind::Knight,
            PromotionPieceKind::Bishop => PieceKind::Bishop,
            PromotionPieceKind::Rook => PieceKind::Rook,
            PromotionPieceKind::Queen => PieceKind::Queen,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub player: Player,
}

impl Piece {
    pub const fn new(player: Player, kind: PieceKind) -> Piece {
        Piece { kind, player }
    }

    pub const fn white(kind: PieceKind) -> Piece {
        Self::new(Player::White, kind)
    }

    pub const fn black(kind: PieceKind) -> Piece {
        Self::new(Player::Black, kind)
    }
}

pub mod known {
    use super::{Piece, PieceKind};

    pub const WHITE_PAWN: Piece = Piece::white(PieceKind::Pawn);
    pub const WHITE_KNIGHT: Piece = Piece::white(PieceKind::Knight);
    pub const WHITE_BISHOP: Piece = Piece::white(PieceKind::Bishop);
    pub const WHITE_ROOK: Piece = Piece::white(PieceKind::Rook);
    pub const WHITE_QUEEN: Piece = Piece::white(PieceKind::Queen);
    pub const WHITE_KING: Piece = Piece::white(PieceKind::King);

    pub const BLACK_PAWN: Piece = Piece::black(PieceKind::Pawn);
    pub const BLACK_KNIGHT: Piece = Piece::black(PieceKind::Knight);
    pub const BLACK_BISHOP: Piece = Piece::black(PieceKind::Bishop);
    pub const BLACK_ROOK: Piece = Piece::black(PieceKind::Rook);
    pub const BLACK_QUEEN: Piece = Piece::black(PieceKind::Queen);
    pub const BLACK_KING: Piece = Piece::black(PieceKind::King);
}
