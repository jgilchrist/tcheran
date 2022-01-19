pub struct Bitboard(u64);

impl Bitboard {
    pub const A_FILE: Self = Bitboard::new(0x0101010101010101);
    pub const H_FILE: Self = Bitboard::new(0x8080808080808080);
    pub const RANK_1: Self = Bitboard::new(0x00000000000000FF);
    pub const RANK_8: Self = Bitboard::new(0xFF00000000000000);
    pub const UP_DIAGONAL: Self = Bitboard::new(0x8040201008040201);
    pub const DOWN_DIAGONAL: Self = Bitboard::new(0x0102040810204080);
    pub const LIGHT_SQUARES: Self = Bitboard::new(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES: Self = Bitboard::new(0xAA55AA55AA55AA55);
    pub const EMPTY: Self = Bitboard::new(0);

    pub const fn new(squares: u64) -> Bitboard {
        Bitboard(squares)
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
                .map(|y| {
                    (0..8)
                        .into_iter()
                        .map(|x| match self.0 & (1 << (y * 8 + x)) {
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
                .map(|y| {
                    (0..8)
                        .into_iter()
                        .map(|x| match self.0 & (1 << (y * 8 + x)) {
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
        let bitboard = Bitboard::new(0);
        let formatted_bitboard = format!("{}", bitboard);

        println!("{}", formatted_bitboard);

        assert_eq!(
            formatted_bitboard, ". . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . .\n. . . . . . . ."
        );
    }

    #[test]
    fn test_full_bitboard_display() {
        let bitboard = Bitboard::new(u64::MAX);
        let formatted_bitboard = format!("{}", bitboard);

        println!("{}", formatted_bitboard);

        assert_eq!(
            formatted_bitboard, "* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *\n* * * * * * * *"
        );
    }
}
