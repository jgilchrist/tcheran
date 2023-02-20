use crate::direction::Direction;
use crate::square::Square;

// TODO: Try removing Copy so that clones have to be explicit
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    #[must_use]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn full() -> Self {
        Self(u64::MAX)
    }

    #[inline(always)]
    #[must_use]
    pub const fn has_square(&self, square: Square) -> bool {
        self.0 & square.0 .0 != 0
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
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

    #[must_use]
    pub fn pop_lsb_inplace(&mut self) -> Self {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn trailing_zeros(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    #[inline(always)]
    #[must_use]
    pub const fn in_direction(&self, direction: Direction) -> Self {
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
    pub const fn east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self((self.0 << 1) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn north_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self((self.0 << 9) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn south_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self((self.0 >> 7) & known::NOT_A_FILE.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self((self.0 >> 1) & known::NOT_H_FILE.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn south_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self((self.0 >> 9) & known::NOT_H_FILE.0)
    }

    #[inline(always)]
    #[must_use]
    pub const fn north_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self((self.0 << 7) & known::NOT_H_FILE.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
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

pub mod known {
    use super::Bitboard;

    pub const A_FILE: Bitboard = Bitboard::new(0x0101_0101_0101_0101);
    pub const B_FILE: Bitboard = Bitboard::new(0x0202_0202_0202_0202);
    pub const C_FILE: Bitboard = Bitboard::new(0x0404_0404_0404_0404);
    pub const D_FILE: Bitboard = Bitboard::new(0x0808_0808_0808_0808);
    pub const E_FILE: Bitboard = Bitboard::new(0x1010_1010_1010_1010);
    pub const F_FILE: Bitboard = Bitboard::new(0x2020_2020_2020_2020);
    pub const G_FILE: Bitboard = Bitboard::new(0x4040_4040_4040_4040);
    pub const H_FILE: Bitboard = Bitboard::new(0x8080_8080_8080_8080);

    pub const RANK_1: Bitboard = Bitboard::new(0x0000_0000_0000_00FF);
    pub const RANK_2: Bitboard = Bitboard::new(0x0000_0000_0000_FF00);
    pub const RANK_3: Bitboard = Bitboard::new(0x0000_0000_00FF_0000);
    pub const RANK_4: Bitboard = Bitboard::new(0x0000_0000_FF00_0000);
    pub const RANK_5: Bitboard = Bitboard::new(0x0000_00FF_0000_0000);
    pub const RANK_6: Bitboard = Bitboard::new(0x0000_FF00_0000_0000);
    pub const RANK_7: Bitboard = Bitboard::new(0x00FF_0000_0000_0000);
    pub const RANK_8: Bitboard = Bitboard::new(0xFF00_0000_0000_0000);

    pub const UP_DIAGONAL: Bitboard = Bitboard::new(0x8040_2010_0804_0201);
    pub const DOWN_DIAGONAL: Bitboard = Bitboard::new(0x0102_0408_1020_4080);
    pub const LIGHT_SQUARES: Bitboard = Bitboard::new(0x55AA_55AA_55AA_55AA);
    pub const DARK_SQUARES: Bitboard = Bitboard::new(0xAA55_AA55_AA55_AA55);

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

    pub const NOT_A_FILE: Bitboard = Bitboard::new(0xFEFE_FEFE_FEFE_FEFE); // ~0x0101010101010101
    pub const NOT_H_FILE: Bitboard = Bitboard::new(0x7F7F_7F7F_7F7F_7F7F); // ~0x8080808080808080
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitboard_display() {
        let bitboard = Bitboard::empty();
        let formatted_bitboard = format!("{bitboard}");

        assert_eq!(
            formatted_bitboard, ". . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . ."
        );
    }

    #[test]
    fn test_full_bitboard_display() {
        let bitboard = Bitboard::full();
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
