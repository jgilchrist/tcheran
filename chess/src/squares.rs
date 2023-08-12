use crate::{
    bitboard::{self, Bitboard},
    direction::Direction,
    player::Player,
    square::{Square, File, Rank},
};

use self::all::*;

/// A set of squares on a chessboard.
///
/// In practice, a transparent wrapper for a bitboard.
/// However, the terminology and API are bitboard agnostic.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Squares(pub(super) Bitboard);

pub struct SquareIterator(Squares);

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.pop_inplace())
        }
    }
}

impl IntoIterator for Squares {
    type Item = Square;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator(self)
    }
}

impl<'a> IntoIterator for &'a Squares {
    type Item = Square;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator(*self)
    }
}

impl Squares {
    pub const N: usize = File::N * Rank::N;

    #[must_use]
    pub const fn from_bitboard(bitboard: Bitboard) -> Self {
        Self(bitboard)
    }

    #[must_use]
    pub const fn from_square(square: Square) -> Self {
        Self(square.0)
    }

    #[must_use]
    pub const fn none() -> Self {
        Self(Bitboard::empty())
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(Bitboard::full())
    }

    #[must_use]
    pub const fn all_except(square: Square) -> Self {
        Self(square.0).invert()
    }

    #[must_use]
    pub fn contains(&self, square: Square) -> bool {
        !(*self & square).is_empty()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub const fn count(&self) -> u8 {
        self.0.count()
    }

    #[must_use]
    pub const fn invert(&self) -> Self {
        Self(self.0.invert())
    }

    pub fn pop_inplace(&mut self) -> Square {
        let lsb = self.0.pop_lsb_inplace();
        Square(lsb)
    }

    #[must_use]
    pub const fn iter(&self) -> SquareIterator {
        SquareIterator(*self)
    }

    #[inline(always)]
    #[must_use]
    pub fn single(&self) -> Square {
        assert_eq!(self.count(), 1);
        Square(self.0)
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
        Self(self.0.north())
    }

    #[inline(always)]
    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0.south())
    }

    #[inline(always)]
    #[must_use]
    pub fn east(&self) -> Self {
        Self(self.0.east())
    }

    #[inline(always)]
    #[must_use]
    pub fn north_east(&self) -> Self {
        Self(self.0.north_east())
    }

    #[inline(always)]
    #[must_use]
    pub fn south_east(&self) -> Self {
        Self(self.0.south_east())
    }

    #[inline(always)]
    #[must_use]
    pub fn west(&self) -> Self {
        Self(self.0.west())
    }

    #[inline(always)]
    #[must_use]
    pub fn south_west(&self) -> Self {
        Self(self.0.south_west())
    }

    #[inline(always)]
    #[must_use]
    pub fn north_west(&self) -> Self {
        Self(self.0.north_west())
    }
}

impl std::ops::BitAnd for Squares {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Squares {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitAnd<Square> for Squares {
    type Output = Self;

    fn bitand(self, rhs: Square) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign<Square> for Squares {
    fn bitand_assign(&mut self, rhs: Square) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitOr for Squares {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Squares {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::ops::BitOr<Square> for Squares {
    type Output = Self;

    fn bitor(self, rhs: Square) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign<Square> for Squares {
    fn bitor_assign(&mut self, rhs: Square) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::fmt::Debug for Squares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
            (0..Rank::N)
                .rev()
                .map(|rank| {
                    (0..File::N)
                        .map(|file| {
                            if self.contains(Square::from_array_idxs(file, rank)) {
                                "*"
                            } else {
                                "."
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub mod all {
    use crate::square::{File, Rank, Square};

    // For convenience
    pub const A1: Square = Square::from_file_and_rank(File::A, Rank::R1);
    pub const A2: Square = Square::from_file_and_rank(File::A, Rank::R2);
    pub const A3: Square = Square::from_file_and_rank(File::A, Rank::R3);
    pub const A4: Square = Square::from_file_and_rank(File::A, Rank::R4);
    pub const A5: Square = Square::from_file_and_rank(File::A, Rank::R5);
    pub const A6: Square = Square::from_file_and_rank(File::A, Rank::R6);
    pub const A7: Square = Square::from_file_and_rank(File::A, Rank::R7);
    pub const A8: Square = Square::from_file_and_rank(File::A, Rank::R8);

    pub const B1: Square = Square::from_file_and_rank(File::B, Rank::R1);
    pub const B2: Square = Square::from_file_and_rank(File::B, Rank::R2);
    pub const B3: Square = Square::from_file_and_rank(File::B, Rank::R3);
    pub const B4: Square = Square::from_file_and_rank(File::B, Rank::R4);
    pub const B5: Square = Square::from_file_and_rank(File::B, Rank::R5);
    pub const B6: Square = Square::from_file_and_rank(File::B, Rank::R6);
    pub const B7: Square = Square::from_file_and_rank(File::B, Rank::R7);
    pub const B8: Square = Square::from_file_and_rank(File::B, Rank::R8);

    pub const C1: Square = Square::from_file_and_rank(File::C, Rank::R1);
    pub const C2: Square = Square::from_file_and_rank(File::C, Rank::R2);
    pub const C3: Square = Square::from_file_and_rank(File::C, Rank::R3);
    pub const C4: Square = Square::from_file_and_rank(File::C, Rank::R4);
    pub const C5: Square = Square::from_file_and_rank(File::C, Rank::R5);
    pub const C6: Square = Square::from_file_and_rank(File::C, Rank::R6);
    pub const C7: Square = Square::from_file_and_rank(File::C, Rank::R7);
    pub const C8: Square = Square::from_file_and_rank(File::C, Rank::R8);

    pub const D1: Square = Square::from_file_and_rank(File::D, Rank::R1);
    pub const D2: Square = Square::from_file_and_rank(File::D, Rank::R2);
    pub const D3: Square = Square::from_file_and_rank(File::D, Rank::R3);
    pub const D4: Square = Square::from_file_and_rank(File::D, Rank::R4);
    pub const D5: Square = Square::from_file_and_rank(File::D, Rank::R5);
    pub const D6: Square = Square::from_file_and_rank(File::D, Rank::R6);
    pub const D7: Square = Square::from_file_and_rank(File::D, Rank::R7);
    pub const D8: Square = Square::from_file_and_rank(File::D, Rank::R8);

    pub const E1: Square = Square::from_file_and_rank(File::E, Rank::R1);
    pub const E2: Square = Square::from_file_and_rank(File::E, Rank::R2);
    pub const E3: Square = Square::from_file_and_rank(File::E, Rank::R3);
    pub const E4: Square = Square::from_file_and_rank(File::E, Rank::R4);
    pub const E5: Square = Square::from_file_and_rank(File::E, Rank::R5);
    pub const E6: Square = Square::from_file_and_rank(File::E, Rank::R6);
    pub const E7: Square = Square::from_file_and_rank(File::E, Rank::R7);
    pub const E8: Square = Square::from_file_and_rank(File::E, Rank::R8);

    pub const F1: Square = Square::from_file_and_rank(File::F, Rank::R1);
    pub const F2: Square = Square::from_file_and_rank(File::F, Rank::R2);
    pub const F3: Square = Square::from_file_and_rank(File::F, Rank::R3);
    pub const F4: Square = Square::from_file_and_rank(File::F, Rank::R4);
    pub const F5: Square = Square::from_file_and_rank(File::F, Rank::R5);
    pub const F6: Square = Square::from_file_and_rank(File::F, Rank::R6);
    pub const F7: Square = Square::from_file_and_rank(File::F, Rank::R7);
    pub const F8: Square = Square::from_file_and_rank(File::F, Rank::R8);

    pub const G1: Square = Square::from_file_and_rank(File::G, Rank::R1);
    pub const G2: Square = Square::from_file_and_rank(File::G, Rank::R2);
    pub const G3: Square = Square::from_file_and_rank(File::G, Rank::R3);
    pub const G4: Square = Square::from_file_and_rank(File::G, Rank::R4);
    pub const G5: Square = Square::from_file_and_rank(File::G, Rank::R5);
    pub const G6: Square = Square::from_file_and_rank(File::G, Rank::R6);
    pub const G7: Square = Square::from_file_and_rank(File::G, Rank::R7);
    pub const G8: Square = Square::from_file_and_rank(File::G, Rank::R8);

    pub const H1: Square = Square::from_file_and_rank(File::H, Rank::R1);
    pub const H2: Square = Square::from_file_and_rank(File::H, Rank::R2);
    pub const H3: Square = Square::from_file_and_rank(File::H, Rank::R3);
    pub const H4: Square = Square::from_file_and_rank(File::H, Rank::R4);
    pub const H5: Square = Square::from_file_and_rank(File::H, Rank::R5);
    pub const H6: Square = Square::from_file_and_rank(File::H, Rank::R6);
    pub const H7: Square = Square::from_file_and_rank(File::H, Rank::R7);
    pub const H8: Square = Square::from_file_and_rank(File::H, Rank::R8);
}

#[must_use]
pub const fn king_start(player: Player) -> Square {
    match player {
        Player::White => INIT_WHITE_KING,
        Player::Black => INIT_BLACK_KING,
    }
}

#[must_use]
pub const fn kingside_rook_start(player: Player) -> Square {
    match player {
        Player::White => H1,
        Player::Black => H8,
    }
}

#[must_use]
pub const fn queenside_rook_start(player: Player) -> Square {
    match player {
        Player::White => A1,
        Player::Black => A8,
    }
}

#[must_use]
pub const fn kingside_castle_dest(player: Player) -> Square {
    match player {
        Player::White => WHITE_KINGSIDE_CASTLE_SQUARE,
        Player::Black => BLACK_KINGSIDE_CASTLE_SQUARE,
    }
}

#[must_use]
pub const fn queenside_castle_dest(player: Player) -> Square {
    match player {
        Player::White => WHITE_QUEENSIDE_CASTLE_SQUARE,
        Player::Black => BLACK_QUEENSIDE_CASTLE_SQUARE,
    }
}

#[must_use]
pub const fn kingside_required_not_attacked_squares(player: Player) -> Squares {
    match player {
        Player::White => WHITE_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
        Player::Black => BLACK_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
    }
}

#[must_use]
pub const fn queenside_required_not_attacked_squares(player: Player) -> Squares {
    match player {
        Player::White => WHITE_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
        Player::Black => BLACK_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES,
    }
}

pub const A_FILE: Squares = Squares::from_bitboard(bitboard::known::A_FILE);
pub const B_FILE: Squares = Squares::from_bitboard(bitboard::known::B_FILE);
pub const C_FILE: Squares = Squares::from_bitboard(bitboard::known::C_FILE);
pub const D_FILE: Squares = Squares::from_bitboard(bitboard::known::D_FILE);
pub const E_FILE: Squares = Squares::from_bitboard(bitboard::known::E_FILE);
pub const F_FILE: Squares = Squares::from_bitboard(bitboard::known::F_FILE);
pub const G_FILE: Squares = Squares::from_bitboard(bitboard::known::G_FILE);
pub const H_FILE: Squares = Squares::from_bitboard(bitboard::known::H_FILE);

pub const RANK_1: Squares = Squares::from_bitboard(bitboard::known::RANK_1);
pub const RANK_2: Squares = Squares::from_bitboard(bitboard::known::RANK_2);
pub const RANK_3: Squares = Squares::from_bitboard(bitboard::known::RANK_3);
pub const RANK_4: Squares = Squares::from_bitboard(bitboard::known::RANK_4);
pub const RANK_5: Squares = Squares::from_bitboard(bitboard::known::RANK_5);
pub const RANK_6: Squares = Squares::from_bitboard(bitboard::known::RANK_6);
pub const RANK_7: Squares = Squares::from_bitboard(bitboard::known::RANK_7);
pub const RANK_8: Squares = Squares::from_bitboard(bitboard::known::RANK_8);

pub const UP_DIAGONAL: Squares = Squares::from_bitboard(bitboard::known::UP_DIAGONAL);
pub const DOWN_DIAGONAL: Squares = Squares::from_bitboard(bitboard::known::DOWN_DIAGONAL);

pub const LIGHT_SQUARES: Squares = Squares::from_bitboard(bitboard::known::LIGHT_SQUARES);
pub const DARK_SQUARES: Squares = Squares::from_bitboard(bitboard::known::DARK_SQUARES);

pub const INIT_WHITE_PAWNS: Squares = Squares::from_bitboard(bitboard::known::INIT_WHITE_PAWNS);
pub const INIT_WHITE_KNIGHTS: Squares = Squares::from_bitboard(bitboard::known::INIT_WHITE_KNIGHTS);
pub const INIT_WHITE_BISHOPS: Squares = Squares::from_bitboard(bitboard::known::INIT_WHITE_BISHOPS);
pub const INIT_WHITE_ROOKS: Squares = Squares::from_bitboard(bitboard::known::INIT_WHITE_ROOKS);
pub const INIT_WHITE_QUEEN: Square = D1;
pub const INIT_WHITE_KING: Square = E1;

pub const WHITE_KINGSIDE_CASTLE_SQUARE: Square = G1;
pub const WHITE_QUEENSIDE_CASTLE_SQUARE: Square = C1;

pub const INIT_BLACK_PAWNS: Squares = Squares::from_bitboard(bitboard::known::INIT_BLACK_PAWNS);
pub const INIT_BLACK_KNIGHTS: Squares = Squares::from_bitboard(bitboard::known::INIT_BLACK_KNIGHTS);
pub const INIT_BLACK_BISHOPS: Squares = Squares::from_bitboard(bitboard::known::INIT_BLACK_BISHOPS);
pub const INIT_BLACK_ROOKS: Squares = Squares::from_bitboard(bitboard::known::INIT_BLACK_ROOKS);
pub const INIT_BLACK_QUEEN: Square = D8;
pub const INIT_BLACK_KING: Square = E8;

pub const BLACK_KINGSIDE_CASTLE_SQUARE: Square = G8;
pub const BLACK_QUEENSIDE_CASTLE_SQUARE: Square = C8;

pub const WHITE_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Squares =
    Squares::from_bitboard(bitboard::known::WHITE_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES);
pub const BLACK_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Squares =
    Squares::from_bitboard(bitboard::known::BLACK_KINGSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES);

pub const WHITE_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Squares =
    Squares::from_bitboard(bitboard::known::WHITE_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES);
pub const BLACK_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES: Squares =
    Squares::from_bitboard(bitboard::known::BLACK_QUEENSIDE_CASTLE_REQUIRED_NOT_ATTACKED_SQUARES);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_size() {
        assert_eq!(
            std::mem::size_of::<Squares>(),
            std::mem::size_of::<Bitboard>()
        );
    }
}
