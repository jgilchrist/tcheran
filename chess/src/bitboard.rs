use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(squares: u64) -> Bitboard {
        Bitboard(squares)
    }

    pub fn has_square(&self, square: &Square) -> bool {
        !(*self & Bitboard::from_square(square)).is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn from_square(square: &Square) -> Bitboard {
        use crate::square::File::*;
        use crate::square::Rank::*;

        let file_idx = match &square.0 {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            H => 7,
        };

        let rank_idx = match &square.1 {
            R1 => 0,
            R2 => 1,
            R3 => 2,
            R4 => 3,
            R5 => 4,
            R6 => 5,
            R7 => 6,
            R8 => 7,
        };

        let square_idx = rank_idx * 8 + file_idx;
        Bitboard(1 << square_idx)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
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
