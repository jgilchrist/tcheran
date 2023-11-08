use crate::square::Square;
use crate::{
    direction::Direction,
    square::{File, Rank},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    const NOT_A_FILE: Self = bitboards::A_FILE.invert();
    const NOT_H_FILE: Self = bitboards::H_FILE.invert();

    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(u64::MAX);

    #[inline(always)]
    #[must_use]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    #[inline(always)]
    #[must_use]
    pub const fn all_except(square: Square) -> Self {
        square.0.invert()
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    #[must_use]
    pub const fn any(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    #[must_use]
    pub fn contains(&self, square: Square) -> bool {
        (*self & square.0).any()
    }

    #[inline(always)]
    #[must_use]
    pub fn single(&self) -> Square {
        debug_assert_eq!(self.count(), 1);
        Square(*self)
    }

    #[inline(always)]
    #[must_use]
    pub const fn invert(&self) -> Self {
        Self(!self.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn lsb(&self) -> Self {
        Self((1_u64).wrapping_shl(self.0.trailing_zeros()))
    }

    #[inline(always)]
    #[must_use]
    pub fn pop_lsb_inplace(&mut self) -> Self {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    #[inline(always)]
    pub fn set_inplace(&mut self, square: Square) {
        self.0 |= square.0 .0;
    }

    #[inline(always)]
    pub fn unset_inplace(&mut self, square: Square) {
        self.0 &= square.0.invert().0;
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn trailing_zeros(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    #[inline(always)]
    #[must_use]
    pub fn in_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => self.north(),
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn north(&self) -> Self {
        Self(self.0 << 8)
    }

    #[inline(always)]
    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 8)
    }

    #[inline(always)]
    #[must_use]
    pub fn east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self(self.0 << 1) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub fn north_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self(self.0 << 9) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub fn south_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self(self.0 >> 7) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub fn west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 >> 1) & Self::NOT_H_FILE
    }

    #[inline(always)]
    #[must_use]
    pub fn south_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 >> 9) & Self::NOT_H_FILE
    }

    #[inline(always)]
    #[must_use]
    pub fn north_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 << 7) & Self::NOT_H_FILE
    }
}

pub struct SquareIterator(Bitboard);

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(Square(self.0.pop_lsb_inplace()))
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator(self)
    }
}

impl<'a> IntoIterator for &'a Bitboard {
    type Item = Square;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator(*self)
    }
}

impl std::ops::Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd<Square> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Square) -> Self::Output {
        self & rhs.0
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitAndAssign<Square> for Bitboard {
    fn bitand_assign(&mut self, rhs: Square) {
        self.0 = self.0 & (rhs.0 .0);
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<Square> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Square) -> Self::Output {
        self | rhs.0
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::ops::BitOrAssign<Square> for Bitboard {
    fn bitor_assign(&mut self, rhs: Square) {
        self.0 = self.0 | (rhs.0 .0);
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.invert()
    }
}

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
            (0..Rank::N)
                .rev()
                .map(|rank| {
                    (0..File::N)
                        .map(|file| match self.0 & (1 << (rank * 8 + file)) {
                            0 => ".",
                            _ => "*",
                        })
                        .collect::<Vec<&str>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..8)
                .rev()
                .map(|rank| {
                    (0..8)
                        .map(|file| match self.0 & (1 << (rank * 8 + file)) {
                            0 => ".",
                            _ => "*",
                        })
                        .collect::<Vec<&str>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[rustfmt::skip]
pub mod bitboards {
    use super::*;
    use crate::player::Player;
    use crate::square::squares;
    use crate::square::squares::all::*;

    #[must_use]
    pub const fn kingside_required_empty_and_not_attacked_squares(player: Player) -> Bitboard {
        match player {
            Player::White => WHITE_KINGSIDE_CASTLE_REQUIRED_EMPTY_AND_NOT_ATTACKED_SQUARES,
            Player::Black => BLACK_KINGSIDE_CASTLE_REQUIRED_EMPTY_AND_NOT_ATTACKED_SQUARES,
        }
    }

    #[must_use]
    pub const fn queenside_required_empty_squares(player: Player) -> Bitboard {
        match player {
            Player::White => WHITE_QUEENSIDE_CASTLE_REQUIRED_EMPTY_SQUARES,
            Player::Black => BLACK_QUEENSIDE_CASTLE_REQUIRED_EMPTY_SQUARES,
        }
    }

    #[must_use]
    pub const fn queenside_required_not_attacked_squares(player: Player) -> Bitboard {
        match player {
            Player::White => WHITE_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
            Player::Black => BLACK_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
        }
    }

    #[must_use]
    pub const fn pawn_back_rank(player: Player) -> Bitboard {
        match player {
            Player::White => RANK_2,
            Player::Black => RANK_7,
        }
    }

    #[must_use]
    pub const fn pawn_double_push_rank(player: Player) -> Bitboard {
        match player {
            Player::White => RANK_4,
            Player::Black => RANK_5,
        }
    }


    // TODO: Once const traits are stabilised, all of this logic can be moved to BitOr and BitAnd impls directly
    pub const A_FILE: Bitboard = Bitboard::new(A1.0.0 | A2.0.0 | A3.0.0 | A4.0.0 | A5.0.0 | A6.0.0 | A7.0.0 | A8.0.0);
    pub const B_FILE: Bitboard = Bitboard::new(B1.0.0 | B2.0.0 | B3.0.0 | B4.0.0 | B5.0.0 | B6.0.0 | B7.0.0 | B8.0.0);
    pub const C_FILE: Bitboard = Bitboard::new(C1.0.0 | C2.0.0 | C3.0.0 | C4.0.0 | C5.0.0 | C6.0.0 | C7.0.0 | C8.0.0);
    pub const D_FILE: Bitboard = Bitboard::new(D1.0.0 | D2.0.0 | D3.0.0 | D4.0.0 | D5.0.0 | D6.0.0 | D7.0.0 | D8.0.0);
    pub const E_FILE: Bitboard = Bitboard::new(E1.0.0 | E2.0.0 | E3.0.0 | E4.0.0 | E5.0.0 | E6.0.0 | E7.0.0 | E8.0.0);
    pub const F_FILE: Bitboard = Bitboard::new(F1.0.0 | F2.0.0 | F3.0.0 | F4.0.0 | F5.0.0 | F6.0.0 | F7.0.0 | F8.0.0);
    pub const G_FILE: Bitboard = Bitboard::new(G1.0.0 | G2.0.0 | G3.0.0 | G4.0.0 | G5.0.0 | G6.0.0 | G7.0.0 | G8.0.0);
    pub const H_FILE: Bitboard = Bitboard::new(H1.0.0 | H2.0.0 | H3.0.0 | H4.0.0 | H5.0.0 | H6.0.0 | H7.0.0 | H8.0.0);

    pub const RANK_1: Bitboard = Bitboard::new(A1.0.0 | B1.0.0 | C1.0.0 | D1.0.0 | E1.0.0 | F1.0.0 | G1.0.0 | H1.0.0);
    pub const RANK_2: Bitboard = Bitboard::new(A2.0.0 | B2.0.0 | C2.0.0 | D2.0.0 | E2.0.0 | F2.0.0 | G2.0.0 | H2.0.0);
    pub const RANK_3: Bitboard = Bitboard::new(A3.0.0 | B3.0.0 | C3.0.0 | D3.0.0 | E3.0.0 | F3.0.0 | G3.0.0 | H3.0.0);
    pub const RANK_4: Bitboard = Bitboard::new(A4.0.0 | B4.0.0 | C4.0.0 | D4.0.0 | E4.0.0 | F4.0.0 | G4.0.0 | H4.0.0);
    pub const RANK_5: Bitboard = Bitboard::new(A5.0.0 | B5.0.0 | C5.0.0 | D5.0.0 | E5.0.0 | F5.0.0 | G5.0.0 | H5.0.0);
    pub const RANK_6: Bitboard = Bitboard::new(A6.0.0 | B6.0.0 | C6.0.0 | D6.0.0 | E6.0.0 | F6.0.0 | G6.0.0 | H6.0.0);
    pub const RANK_7: Bitboard = Bitboard::new(A7.0.0 | B7.0.0 | C7.0.0 | D7.0.0 | E7.0.0 | F7.0.0 | G7.0.0 | H7.0.0);
    pub const RANK_8: Bitboard = Bitboard::new(A8.0.0 | B8.0.0 | C8.0.0 | D8.0.0 | E8.0.0 | F8.0.0 | G8.0.0 | H8.0.0);

    pub const UP_DIAGONAL: Bitboard = Bitboard::new(A1.0.0 | B2.0.0 | C3.0.0 | D4.0.0 | E5.0.0 | F6.0.0 | G7.0.0 | H8.0.0);
    pub const DOWN_DIAGONAL: Bitboard = Bitboard::new(A8.0.0 | B7.0.0 | C6.0.0 | D5.0.0 | E4.0.0 | F3.0.0 | G2.0.0 | H1.0.0);

    pub const LIGHT_SQUARES: Bitboard = Bitboard::new(
        A8.0.0 | C8.0.0 | E8.0.0 | G8.0.0 |
        B7.0.0 | D7.0.0 | F7.0.0 | H7.0.0 |
        A6.0.0 | C6.0.0 | E6.0.0 | G6.0.0 |
        B5.0.0 | D5.0.0 | F5.0.0 | H5.0.0 |
        A4.0.0 | C4.0.0 | E4.0.0 | G4.0.0 |
        B3.0.0 | D3.0.0 | F3.0.0 | H3.0.0 |
        A2.0.0 | C2.0.0 | E2.0.0 | G2.0.0 |
        B1.0.0 | D1.0.0 | F1.0.0 | H1.0.0 );

    #[rustfmt::skip]
    pub const DARK_SQUARES: Bitboard = Bitboard::new(
        B8.0.0 | D8.0.0 | F8.0.0 | H8.0.0 |
        A7.0.0 | C7.0.0 | E7.0.0 | G7.0.0 |
        B6.0.0 | D6.0.0 | F6.0.0 | H6.0.0 |
        A5.0.0 | C5.0.0 | E5.0.0 | G5.0.0 |
        B4.0.0 | D4.0.0 | F4.0.0 | H4.0.0 |
        A3.0.0 | C3.0.0 | E3.0.0 | G3.0.0 |
        B2.0.0 | D2.0.0 | F2.0.0 | H2.0.0 |
        A1.0.0 | C1.0.0 | E1.0.0 | G1.0.0 );

    pub const INIT_WHITE_PAWNS: Bitboard = RANK_2;
    pub const INIT_WHITE_KNIGHTS: Bitboard = Bitboard::new(B1.0.0 | G1.0.0);
    pub const INIT_WHITE_BISHOPS: Bitboard = Bitboard::new(C1.0.0 | F1.0.0);
    pub const INIT_WHITE_ROOKS: Bitboard = Bitboard::new(A1.0.0 | H1.0.0);
    pub const INIT_WHITE_QUEEN: Bitboard = squares::INIT_WHITE_QUEEN.0;
    pub const INIT_WHITE_KING: Bitboard = squares::INIT_WHITE_KING.0;

    pub const INIT_BLACK_PAWNS: Bitboard = RANK_7;
    pub const INIT_BLACK_KNIGHTS: Bitboard = Bitboard::new(B8.0.0 | G8.0.0);
    pub const INIT_BLACK_BISHOPS: Bitboard = Bitboard::new(C8.0.0 | F8.0.0);
    pub const INIT_BLACK_ROOKS: Bitboard = Bitboard::new(A8.0.0 | H8.0.0);
    pub const INIT_BLACK_QUEEN: Bitboard = squares::INIT_BLACK_QUEEN.0;
    pub const INIT_BLACK_KING: Bitboard = squares::INIT_BLACK_KING.0;

    pub const WHITE_KINGSIDE_CASTLE_REQUIRED_EMPTY_AND_NOT_ATTACKED_SQUARES: Bitboard = Bitboard::new(F1.0.0 | G1.0.0);
    pub const BLACK_KINGSIDE_CASTLE_REQUIRED_EMPTY_AND_NOT_ATTACKED_SQUARES: Bitboard = Bitboard::new(F8.0.0 | G8.0.0);

    pub const WHITE_QUEENSIDE_CASTLE_REQUIRED_EMPTY_SQUARES: Bitboard = Bitboard::new(B1.0.0 | C1.0.0 | D1.0.0);
    pub const BLACK_QUEENSIDE_CASTLE_REQUIRED_EMPTY_SQUARES: Bitboard = Bitboard::new(B8.0.0 | C8.0.0 | D8.0.0);


    pub const WHITE_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Bitboard = Bitboard::new(C1.0.0 | D1.0.0);
    pub const BLACK_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Bitboard = Bitboard::new(C8.0.0 | D8.0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitboard_display() {
        let bitboard = Bitboard::EMPTY;
        let formatted_bitboard = format!("{bitboard}");

        assert_eq!(
            formatted_bitboard, ". . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . ."
        );
    }

    #[test]
    fn test_full_bitboard_display() {
        let bitboard = Bitboard::FULL;
        let formatted_bitboard = format!("{bitboard}");

        assert_eq!(
            formatted_bitboard, "* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *"
        );
    }

    #[test]
    fn bitboard_size() {
        assert_eq!(std::mem::size_of::<Bitboard>(), std::mem::size_of::<u64>());
    }
}
