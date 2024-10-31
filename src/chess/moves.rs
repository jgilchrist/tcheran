use crate::chess::{piece::PromotionPieceKind, square::Square};
use arrayvec::ArrayVec;
use std::num::NonZeroU16;

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
//  5 │      │ ─┘                                   xxxxxxxx ─── Unused
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
const CAPTURE_BIT_MASK: u16 = 0b0001_0000_0000_0000;
const PROMOTION_BIT_MASK: u16 = 0b0010_0000_0000_0000;
const FIRST_FLAG_MASK: u16 = 0b0100_0000_0000_0000;
const SECOND_FLAG_MASK: u16 = 0b1000_0000_0000_0000;
const FLAGS_MASK: u16 = FIRST_FLAG_MASK | SECOND_FLAG_MASK;

const DST_SHIFT: usize = 6;
const CAPTURE_BIT_SHIFT: usize = 12;
const PROMOTION_BIT_SHIFT: usize = 13;
const FIRST_FLAG_SHIFT: usize = 14;
const FLAGS_SHIFT: usize = 14;
const SECOND_FLAG_SHIFT: usize = 15;

#[inline]
const fn src_bits(src: Square) -> u16 {
    src.idx() as u16
}

#[inline]
const fn dst_bits(dst: Square) -> u16 {
    (dst.idx() as u16) << DST_SHIFT
}

#[inline]
const fn capture_bit() -> u16 {
    1u16 << CAPTURE_BIT_SHIFT
}

#[inline]
const fn promotion_bit() -> u16 {
    1u16 << PROMOTION_BIT_SHIFT
}

#[inline]
const fn flag_bits(f1: bool, f2: bool) -> u16 {
    (f1 as u16) << FIRST_FLAG_SHIFT | (f2 as u16) << SECOND_FLAG_SHIFT
}

#[inline]
const fn promotion_flag_bits(promotion_piece_kind: PromotionPieceKind) -> u16 {
    (match promotion_piece_kind {
        PromotionPieceKind::Queen => 0b00u16,
        PromotionPieceKind::Rook => 0b01u16,
        PromotionPieceKind::Knight => 0b10u16,
        PromotionPieceKind::Bishop => 0b11u16,
    }) << FLAGS_SHIFT
}

impl Move {
    #[inline]
    const fn new(data: u16) -> Self {
        // It's impossible for us to create a move with '0' data. In order to do that
        // we'd need both the source and destination squares to be A1 (0).
        Self(unsafe { NonZeroU16::new_unchecked(data) })
    }

    #[inline]
    pub const fn quiet(src: Square, dst: Square) -> Self {
        Self::new(src_bits(src) | dst_bits(dst))
    }

    #[inline]
    pub const fn capture(src: Square, dst: Square) -> Self {
        Self::new(src_bits(src) | dst_bits(dst) | capture_bit())
    }

    #[inline]
    pub const fn castles(src: Square, dst: Square) -> Self {
        Self::new(src_bits(src) | dst_bits(dst) | flag_bits(true, false))
    }

    #[inline]
    pub const fn en_passant(src: Square, dst: Square) -> Self {
        Self::new(src_bits(src) | dst_bits(dst) | capture_bit() | flag_bits(true, false))
    }

    #[inline]
    pub const fn quiet_promotion(src: Square, dst: Square, promotion: PromotionPieceKind) -> Self {
        Self::new(src_bits(src) | dst_bits(dst) | promotion_bit() | promotion_flag_bits(promotion))
    }

    #[inline]
    pub const fn capture_promotion(
        src: Square,
        dst: Square,
        promotion: PromotionPieceKind,
    ) -> Self {
        Self::new(
            src_bits(src)
                | dst_bits(dst)
                | capture_bit()
                | promotion_bit()
                | promotion_flag_bits(promotion),
        )
    }

    #[inline]
    fn data(&self) -> u16 {
        self.0.get()
    }

    #[inline]
    pub fn src(&self) -> Square {
        Square::from_index((self.data() & SRC_MASK) as u8)
    }

    #[inline]
    pub fn dst(&self) -> Square {
        Square::from_index(((self.data() & DST_MASK) >> DST_SHIFT) as u8)
    }

    #[inline]
    pub fn is_capture(&self) -> bool {
        (self.data() & CAPTURE_BIT_MASK) == CAPTURE_BIT_MASK
    }

    #[inline]
    pub fn is_promotion(&self) -> bool {
        (self.data() & PROMOTION_BIT_MASK) == PROMOTION_BIT_MASK
    }

    #[inline]
    pub fn promotion(&self) -> Option<PromotionPieceKind> {
        if !self.is_promotion() {
            return None;
        }

        let flag_bits = (self.data() & FLAGS_MASK) >> FLAGS_SHIFT;

        Some(match flag_bits {
            0b00 => PromotionPieceKind::Queen,
            0b01 => PromotionPieceKind::Rook,
            0b10 => PromotionPieceKind::Knight,
            0b11 => PromotionPieceKind::Bishop,
            _ => unreachable!(),
        })
    }

    #[inline]
    pub fn is_quiet(&self) -> bool {
        !self.is_promotion() && !self.is_capture()
    }

    #[inline]
    pub fn is_en_passant(&self) -> bool {
        self.is_capture()
            && !self.is_promotion()
            && (self.data() & FIRST_FLAG_MASK) == FIRST_FLAG_MASK
    }

    #[inline]
    pub fn is_castling(&self) -> bool {
        self.is_quiet() && (self.data() & FIRST_FLAG_MASK) == FIRST_FLAG_MASK
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
        assert!(mv.is_quiet());
        assert!(mv.promotion().is_none());
        assert!(!mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_quiet_promotion() {
        let mv = Move::quiet_promotion(A1, B1, PromotionPieceKind::Queen);
        assert!(!mv.is_quiet());
        assert_eq!(mv.promotion(), Some(PromotionPieceKind::Queen));
        assert!(!mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_capture() {
        let mv = Move::capture(A1, B1);
        assert!(!mv.is_quiet());
        assert!(mv.promotion().is_none());
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_capture_promotion() {
        let mv = Move::capture_promotion(A1, B1, PromotionPieceKind::Queen);
        assert!(!mv.is_quiet());
        assert_eq!(mv.promotion(), Some(PromotionPieceKind::Queen));
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_castles() {
        let mv = Move::castles(A1, B1);
        assert!(mv.is_quiet());
        assert!(mv.promotion().is_none());
        assert!(!mv.is_capture());
        assert!(mv.is_castling());
        assert!(!mv.is_en_passant());
    }

    #[test]
    fn test_en_passant() {
        let mv = Move::en_passant(A1, B1);
        assert!(!mv.is_quiet());
        assert!(mv.promotion().is_none());
        assert!(mv.is_capture());
        assert!(!mv.is_castling());
        assert!(mv.is_en_passant());
    }
}
