use crate::{bitboard::Bitboard, square::Square, squares::Squares};

use super::attacks;

static mut ATTACKS_TABLE: [Bitboard; Squares::N] = [Bitboard::EMPTY; Squares::N];

pub fn knight_attacks(s: Square) -> Squares {
    Squares(unsafe { ATTACKS_TABLE[s.idx() as usize] })
}

pub fn init() {
    for s in Squares::ALL {
        let attacks = attacks::generate_knight_attacks(s);

        unsafe {
            ATTACKS_TABLE[s.idx() as usize] = attacks.0;
        }
    }
}
