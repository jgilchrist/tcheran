use crate::{bitboard::Bitboard, square::Square, squares::Squares};

use super::attacks;

static mut ATTACKS_TABLE: [Bitboard; 64] = [Bitboard::empty(); 64];

pub fn king_attacks(s: Square) -> Squares {
    Squares(unsafe { ATTACKS_TABLE[s.idx() as usize] })
}

pub fn init() {
    for s in Squares::all() {
        let attacks = attacks::generate_king_attacks(s);

        unsafe {
            ATTACKS_TABLE[s.idx() as usize] = attacks.0;
        }
    }
}
