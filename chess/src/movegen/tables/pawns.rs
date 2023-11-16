use crate::{bitboard::Bitboard, player::Player, square::Square};

use super::attacks;

static mut ATTACKS_TABLE: [[Bitboard; Square::N]; Player::N] =
    [[Bitboard::EMPTY; Square::N]; Player::N];

pub fn pawn_attacks(s: Square, player: Player) -> Bitboard {
    unsafe { ATTACKS_TABLE[player.array_idx()][s.idx() as usize] }
}

pub fn init() {
    for s in Bitboard::FULL {
        let white_attacks = attacks::generate_pawn_attacks(s, Player::White);
        let black_attacks = attacks::generate_pawn_attacks(s, Player::Black);

        unsafe {
            ATTACKS_TABLE[Player::White.array_idx()][s.idx() as usize] = white_attacks;
            ATTACKS_TABLE[Player::Black.array_idx()][s.idx() as usize] = black_attacks;
        }
    }
}
