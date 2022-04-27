use std::ops::BitAndAssign;

use crate::direction::Direction;
use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(squares: u64) -> Bitboard {
        Bitboard(squares)
    }

    pub const fn empty() -> Bitboard {
        Bitboard(0)
    }

    pub const fn full() -> Bitboard {
        Bitboard(u64::MAX)
    }

    pub fn has_square(&self, square: &Square) -> bool {
        !(*self & Bitboard::from_square(square)).is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn invert(&self) -> Bitboard {
        Bitboard(!self.0)
    }

    pub fn lsb(&self) -> Bitboard {
        Bitboard((1_u64).wrapping_shl(self.0.trailing_zeros()))
    }

    pub fn pop_lsb(&self) -> (Bitboard, Bitboard) {
        let lsb = self.lsb();
        let popped = Bitboard(self.0 & (self.0 - 1));
        (lsb, popped)
    }

    pub fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn bits(&self) -> Vec<Bitboard> {
        let mut bits: Vec<Bitboard> = Vec::with_capacity(self.count() as usize);
        let mut bitboard = Bitboard(self.0);

        loop {
            if bitboard.is_empty() {
                return bits;
            }

            let (next_bit, next_bitboard) = bitboard.pop_lsb();
            bitboard = next_bitboard;
            bits.push(next_bit);
        }
    }

    pub fn squares(&self) -> Vec<Square> {
        self.bits().iter().map(|b| b.to_square_definite()).collect()
    }

    #[inline(always)]
    pub fn from_square(square: &Square) -> Bitboard {
        Bitboard(1 << square.idx())
    }

    #[inline(always)]
    pub fn except_square(square: &Square) -> Bitboard {
        Bitboard::from_square(square).invert()
    }

    #[inline(always)]
    pub fn to_square_definite(&self) -> Square {
        self.to_square().expect("Expected single bit")
    }

    pub fn to_square(&self) -> Option<Square> {
        assert!(self.count() == 0 || self.count() == 1);

        match self.is_empty() {
            true => None,
            false => Some(Square::from_idx(self.0.trailing_zeros() as u8)),
        }
    }

    #[inline(always)]
    pub fn in_direction(&self, direction: Direction) -> Bitboard {
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
    pub fn north(&self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    #[inline(always)]
    pub fn south(&self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    #[inline(always)]
    pub fn east(&self) -> Bitboard {
        // If we go east and land on A, we wrapped around.
        Bitboard((self.0 << 1) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    pub fn north_east(&self) -> Bitboard {
        // If we go east and land on A, we wrapped around.
        Bitboard((self.0 << 9) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    pub fn south_east(&self) -> Bitboard {
        // If we go east and land on A, we wrapped around.
        Bitboard((self.0 >> 7) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    pub fn west(&self) -> Bitboard {
        // If we go west and land on H, we wrapped around.
        Bitboard((self.0 >> 1) & known::NOT_H_FILE.0)
    }

    #[inline(always)]
    pub fn south_west(&self) -> Bitboard {
        // If we go west and land on H, we wrapped around.
        Bitboard((self.0 >> 9) & known::NOT_H_FILE.0)
    }

    #[inline(always)]
    pub fn north_west(&self) -> Bitboard {
        // If we go west and land on H, we wrapped around.
        Bitboard((self.0 << 7) & known::NOT_H_FILE.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
            (0..8)
                .rev()
                .into_iter()
                .map(|rank| {
                    (0..8)
                        .into_iter()
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
                .into_iter()
                .map(|rank| {
                    (0..8)
                        .into_iter()
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

pub mod known {
    use super::Bitboard;

    pub const A_FILE: Bitboard = Bitboard::new(0x0101010101010101);
    pub const B_FILE: Bitboard = Bitboard::new(0x0202020202020202);
    pub const C_FILE: Bitboard = Bitboard::new(0x0404040404040404);
    pub const D_FILE: Bitboard = Bitboard::new(0x0808080808080808);
    pub const E_FILE: Bitboard = Bitboard::new(0x1010101010101010);
    pub const F_FILE: Bitboard = Bitboard::new(0x2020202020202020);
    pub const G_FILE: Bitboard = Bitboard::new(0x4040404040404040);
    pub const H_FILE: Bitboard = Bitboard::new(0x8080808080808080);

    pub const RANK_1: Bitboard = Bitboard::new(0x00000000000000FF);
    pub const RANK_2: Bitboard = Bitboard::new(0x000000000000FF00);
    pub const RANK_3: Bitboard = Bitboard::new(0x0000000000FF0000);
    pub const RANK_4: Bitboard = Bitboard::new(0x00000000FF000000);
    pub const RANK_5: Bitboard = Bitboard::new(0x000000FF00000000);
    pub const RANK_6: Bitboard = Bitboard::new(0x0000FF0000000000);
    pub const RANK_7: Bitboard = Bitboard::new(0x00FF000000000000);
    pub const RANK_8: Bitboard = Bitboard::new(0xFF00000000000000);

    pub const UP_DIAGONAL: Bitboard = Bitboard::new(0x8040201008040201);
    pub const DOWN_DIAGONAL: Bitboard = Bitboard::new(0x0102040810204080);
    pub const LIGHT_SQUARES: Bitboard = Bitboard::new(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES: Bitboard = Bitboard::new(0xAA55AA55AA55AA55);
    pub const EMPTY: Bitboard = Bitboard::new(0);

    pub const INIT_WHITE_PAWNS: Bitboard = RANK_2;
    pub const INIT_WHITE_KNIGHTS: Bitboard = Bitboard::new(1 << 1 | 1 << 6);
    pub const INIT_WHITE_BISHOPS: Bitboard = Bitboard::new(1 << 2 | 1 << 5);
    pub const INIT_WHITE_ROOKS: Bitboard = Bitboard::new(1 << 0 | 1 << 7);
    pub const INIT_WHITE_QUEEN: Bitboard = Bitboard::new(1 << 3);
    pub const INIT_WHITE_KING: Bitboard = Bitboard::new(1 << 4);

    pub const INIT_BLACK_PAWNS: Bitboard = RANK_7;
    pub const INIT_BLACK_KNIGHTS: Bitboard = Bitboard::new(1 << 57 | 1 << 62);
    pub const INIT_BLACK_BISHOPS: Bitboard = Bitboard::new(1 << 58 | 1 << 61);
    pub const INIT_BLACK_ROOKS: Bitboard = Bitboard::new(1 << 56 | 1 << 63);
    pub const INIT_BLACK_QUEEN: Bitboard = Bitboard::new(1 << 59);
    pub const INIT_BLACK_KING: Bitboard = Bitboard::new(1 << 60);

    pub const NOT_A_FILE: Bitboard = Bitboard::new(0xFEFEFEFEFEFEFEFE); // ~0x0101010101010101
    pub const NOT_H_FILE: Bitboard = Bitboard::new(0x7F7F7F7F7F7F7F7F); // ~0x8080808080808080
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitboard_display() {
        let bitboard = Bitboard::new(0);
        let formatted_bitboard = format!("{}", bitboard);

        assert_eq!(
            formatted_bitboard, ". . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . ."
        );
    }

    #[test]
    fn test_full_bitboard_display() {
        let bitboard = Bitboard::new(u64::MAX);
        let formatted_bitboard = format!("{}", bitboard);

        assert_eq!(
            formatted_bitboard, "* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *"
        );
    }
}
