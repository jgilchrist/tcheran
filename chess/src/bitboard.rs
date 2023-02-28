use crate::{direction::Direction, squares};

// TODO: Try removing Copy so that clones have to be explicit
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    const NOT_A_FILE: Self = squares::A_FILE.invert().0;
    const NOT_H_FILE: Self = squares::H_FILE.invert().0;

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
        Self(self.0 << 1) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub const fn north_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self(self.0 << 9) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub const fn south_east(&self) -> Self {
        // If we go east and land on A, we wrapped around.
        Self(self.0 >> 7) & Self::NOT_A_FILE
    }

    #[inline(always)]
    #[must_use]
    pub const fn west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 >> 1) & Self::NOT_H_FILE
    }

    #[inline(always)]
    #[must_use]
    pub const fn south_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 >> 9) & Self::NOT_H_FILE
    }

    #[inline(always)]
    #[must_use]
    pub const fn north_west(&self) -> Self {
        // If we go west and land on H, we wrapped around.
        Self(self.0 << 7) & Self::NOT_H_FILE
    }
}

impl const std::ops::BitAnd for Bitboard {
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

impl const std::ops::BitOr for Bitboard {
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
