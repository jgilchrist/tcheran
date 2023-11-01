use crate::{bitboard::Bitboard, player::Player, square::Square};

use super::attacks;

static mut WHITE_ATTACKS_TABLE: [Bitboard; Square::N] = [Bitboard::EMPTY; Square::N];
static mut BLACK_ATTACKS_TABLE: [Bitboard; Square::N] = [Bitboard::EMPTY; Square::N];

pub fn pawn_attacks(s: Square, player: Player) -> Bitboard {
    match player {
        Player::White => unsafe { WHITE_ATTACKS_TABLE[s.idx() as usize] },
        Player::Black => unsafe { BLACK_ATTACKS_TABLE[s.idx() as usize] },
    }
}

pub fn init() {
    for s in Bitboard::FULL {
        let white_attacks = attacks::generate_pawn_attacks(s, Player::White);
        let black_attacks = attacks::generate_pawn_attacks(s, Player::Black);

        unsafe {
            WHITE_ATTACKS_TABLE[s.idx() as usize] = white_attacks;
            BLACK_ATTACKS_TABLE[s.idx() as usize] = black_attacks;
        }
    }
}
