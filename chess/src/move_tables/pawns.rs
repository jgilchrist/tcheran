use crate::{bitboard::Bitboard, player::Player, square::Square, squares::Squares};

use super::attacks;

static mut WHITE_ATTACKS_TABLE: [Bitboard; Squares::N] = [Bitboard::empty(); Squares::N];
static mut BLACK_ATTACKS_TABLE: [Bitboard; Squares::N] = [Bitboard::empty(); Squares::N];

pub fn pawn_attacks(s: Square, player: Player) -> Squares {
    Squares(match player {
        Player::White => unsafe { WHITE_ATTACKS_TABLE[s.idx() as usize] },
        Player::Black => unsafe { BLACK_ATTACKS_TABLE[s.idx() as usize] },
    })
}

pub fn init() {
    for s in Squares::all() {
        let white_attacks = attacks::generate_pawn_attacks(s, Player::White);
        let black_attacks = attacks::generate_pawn_attacks(s, Player::Black);

        unsafe {
            WHITE_ATTACKS_TABLE[s.idx() as usize] = white_attacks.0;
            BLACK_ATTACKS_TABLE[s.idx() as usize] = black_attacks.0;
        }
    }
}
