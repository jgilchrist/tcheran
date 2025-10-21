use std::num::NonZeroU16;

use arrayvec::ArrayVec;

use crate::chess::{piece::PromotionPieceKind, square::Square};

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

            if mv.src() == src && mv.dst() == dst && mv.promotion() == promotion {
                return mv;
            }
        }

        panic!("Illegal move")
    }
}

//    Layout                                    Flags
//
//    ┌──────┐                                    Quiet move (Capture bit = 0, Promotion bit = 0)
//  0 │      │ ─┐
//    ├──────┤  │                                   ┌──────┐
//    ~~~~~~~~  ├── Source square                   │      │ ─── Castle bit
//    ├──────┤  │                                   ├──────┤
//  5 │      │ ─┘                                   │      │ ─── Double push bit
//    ├──────┤                                      └──────┘
//  6 │      │ ─┐
//    ├──────┤  │                                 Capture move (Capture bit = 1, Promotion bit = 0)
//    ~~~~~~~~  ├── Destination square
//    ├──────┤  │                                   ┌──────┐
// 11 │      │ ─┘                                   │      │ ─── En passant bit
//    ├──────┤                                      ├──────┤
// 12 │      │ ──── Capture bit                     xxxxxxxx ─── Unused
//    ├──────┤                                      └──────┘
// 13 │      │ ──── Promotion bit
//    ├──────┤                                    Promotion move (Capture bit = any, Promotion bit = 1)
// 14 │      │ ─┐
//    ├──────┤  ├── Additional flags                00 - Queen      10 - Knight
// 15 │      │ ─┘                                   01 - Rook       11 - Bishop
//    └──────┘

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move(NonZeroU16);

const SRC_MASK: u16 = 0b0000_0000_0011_1111;
const DST_MASK: u16 = 0b0000_1111_1100_0000;

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Flags {
    Quiet = 0b0000,
    Castle = flag_bits(true, false),
    DoublePush = flag_bits(false, true),
    Capture = CAPTURE_FLAG_BIT,
    EnPassant = CAPTURE_FLAG_BIT | flag_bits(true, false),
    PromoteToBishop = PROMOTION_FLAG_BIT | flag_bits(false, false),
    PromoteToKnight = PROMOTION_FLAG_BIT | flag_bits(false, true),
    PromoteToRook = PROMOTION_FLAG_BIT | flag_bits(true, false),
    PromoteToQueen = PROMOTION_FLAG_BIT | flag_bits(true, true),
    CaptureAndPromoteToBishop = CAPTURE_FLAG_BIT | PROMOTION_FLAG_BIT | flag_bits(false, false),
    CaptureAndPromoteToKnight = CAPTURE_FLAG_BIT | PROMOTION_FLAG_BIT | flag_bits(false, true),
    CaptureAndPromoteToRook = CAPTURE_FLAG_BIT | PROMOTION_FLAG_BIT | flag_bits(true, false),
    CaptureAndPromoteToQueen = CAPTURE_FLAG_BIT | PROMOTION_FLAG_BIT | flag_bits(true, true),
}

impl Flags {
    // Trick from Simbelmyne - rather than checking individual bits, we can transmute and check everything at once
    fn from_u8(flags: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(flags) }
    }
}

const CAPTURE_FLAG_BIT: u8 = 0b0001;
const PROMOTION_FLAG_BIT: u8 = 0b0010;

const CAPTURE_BIT_MASK: u16 = 0b0001_0000_0000_0000;
const PROMOTION_BIT_MASK: u16 = 0b0010_0000_0000_0000;

const fn flag_bits(f1: bool, f2: bool) -> u8 {
    ((f1 as u8) << 2) | ((f2 as u8) << 3)
}

const DST_SHIFT: usize = 6;
const FLAGS_SHIFT: usize = 12;

impl Move {
    #[inline]
    const fn new(src: Square, dst: Square, flags: Flags) -> Self {
        // It's impossible for us to create a move with '0' data. In order to do that
        // we'd need both the source and destination squares to be A1 (0).
        Self(unsafe {
            NonZeroU16::new_unchecked(
                (src.idx() as u16)
                    | (dst.idx() as u16) << DST_SHIFT
                    | ((flags as u16) << FLAGS_SHIFT),
            )
        })
    }

    #[inline]
    pub const fn quiet(src: Square, dst: Square) -> Self {
        Self::new(src, dst, Flags::Quiet)
    }

    #[inline]
    pub const fn double_push(src: Square, dst: Square) -> Self {
        Self::new(src, dst, Flags::DoublePush)
    }

    #[inline]
    pub const fn capture(src: Square, dst: Square) -> Self {
        Self::new(src, dst, Flags::Capture)
    }

    #[inline]
    pub const fn castles(src: Square, dst: Square) -> Self {
        Self::new(src, dst, Flags::Castle)
    }

    #[inline]
    pub const fn en_passant(src: Square, dst: Square) -> Self {
        Self::new(src, dst, Flags::EnPassant)
    }

    #[inline]
    pub const fn quiet_promotion(src: Square, dst: Square, promotion: PromotionPieceKind) -> Self {
        Self::new(
            src,
            dst,
            match promotion {
                PromotionPieceKind::Bishop => Flags::PromoteToBishop,
                PromotionPieceKind::Knight => Flags::PromoteToKnight,
                PromotionPieceKind::Rook => Flags::PromoteToRook,
                PromotionPieceKind::Queen => Flags::PromoteToQueen,
            },
        )
    }

    #[inline]
    pub const fn capture_promotion(
        src: Square,
        dst: Square,
        promotion: PromotionPieceKind,
    ) -> Self {
        Self::new(
            src,
            dst,
            match promotion {
                PromotionPieceKind::Bishop => Flags::CaptureAndPromoteToBishop,
                PromotionPieceKind::Knight => Flags::CaptureAndPromoteToKnight,
                PromotionPieceKind::Rook => Flags::CaptureAndPromoteToRook,
                PromotionPieceKind::Queen => Flags::CaptureAndPromoteToQueen,
            },
        )
    }

    #[inline]
    fn data(self) -> u16 {
        self.0.get()
    }

    #[inline]
    pub fn src(self) -> Square {
        Square::from_index((self.data() & SRC_MASK) as u8)
    }

    #[inline]
    pub fn dst(self) -> Square {
        Square::from_index(((self.data() & DST_MASK) >> DST_SHIFT) as u8)
    }

    #[inline]
    fn flags(self) -> Flags {
        Flags::from_u8((self.0.get() >> FLAGS_SHIFT) as u8)
    }

    #[inline]
    pub fn is_capture(self) -> bool {
        (self.data() & CAPTURE_BIT_MASK) == CAPTURE_BIT_MASK
    }

    #[inline]
    pub fn is_promotion(self) -> bool {
        (self.data() & PROMOTION_BIT_MASK) == PROMOTION_BIT_MASK
    }

    #[inline]
    pub fn promotion(self) -> Option<PromotionPieceKind> {
        use PromotionPieceKind::*;

        match self.flags() {
            Flags::PromoteToBishop | Flags::CaptureAndPromoteToBishop => Some(Bishop),
            Flags::PromoteToKnight | Flags::CaptureAndPromoteToKnight => Some(Knight),
            Flags::PromoteToRook | Flags::CaptureAndPromoteToRook => Some(Rook),
            Flags::PromoteToQueen | Flags::CaptureAndPromoteToQueen => Some(Queen),
            _ => None,
        }
    }

    #[inline]
    pub fn is_en_passant(self) -> bool {
        self.flags() == Flags::EnPassant
    }

    #[inline]
    pub fn is_double_push(self) -> bool {
        self.flags() == Flags::DoublePush
    }

    #[inline]
    pub fn is_castling(self) -> bool {
        self.flags() == Flags::Castle
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.src().notation(),
            self.dst().notation(),
            match self.promotion() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::square::squares::all::*;

    #[test]
    fn check_move_size() {
        assert_eq!(size_of::<Move>(), 2);
    }

    #[test]
    fn check_move_size_is_same_as_option_move_size() {
        assert_eq!(size_of::<Move>(), size_of::<Option<Move>>());
    }

    #[test]
    fn test_quiet() {
        let mv = Move::quiet(A1, B1);
        assert_eq!(mv.src(), A1);
        assert_eq!(mv.dst(), B1);
        assert!(mv.promotion().is_none());
        assert!(!mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_quiet_promotion() {
        let mv = Move::quiet_promotion(A1, B1, PromotionPieceKind::Queen);
        assert_eq!(mv.promotion(), Some(PromotionPieceKind::Queen));
        assert!(!mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_capture() {
        let mv = Move::capture(A1, B1);
        assert!(mv.promotion().is_none());
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_capture_promotion() {
        let mv = Move::capture_promotion(A1, B1, PromotionPieceKind::Queen);
        assert_eq!(mv.promotion(), Some(PromotionPieceKind::Queen));
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_castles() {
        let mv = Move::castles(A1, B1);
        assert!(mv.promotion().is_none());
        assert!(!mv.is_capture());
        assert!(mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_en_passant() {
        let mv = Move::en_passant(A1, B1);
        assert!(mv.promotion().is_none());
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(mv.is_en_passant());
    }
}
