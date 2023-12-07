use crate::{bitboard::Bitboard, square::Square};

use super::attacks;

static mut ATTACKS_TABLE: [Bitboard; Square::N] = [Bitboard::EMPTY; Square::N];

pub fn king_attacks(s: Square) -> Bitboard {
    *unsafe { ATTACKS_TABLE.get_unchecked(s.array_idx()) }
}

pub fn init() {
    for s in Bitboard::FULL {
        let attacks = attacks::generate_king_attacks(s);

        unsafe {
            ATTACKS_TABLE[s.array_idx()] = attacks;
        }
    }
}
