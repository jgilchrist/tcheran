use crate::chess::player::{Black, PlayerT, White};
use crate::chess::{bitboard::Bitboard, player::Player, square::Square};

use super::attacks;

static mut ATTACKS_TABLE: [[Bitboard; Square::N]; Player::N] =
    [[Bitboard::EMPTY; Square::N]; Player::N];

pub fn pawn_attacks<PLAYER: PlayerT>(s: Square) -> Bitboard {
    *unsafe {
        ATTACKS_TABLE
            .get_unchecked(PLAYER::IDX)
            .get_unchecked(s.array_idx())
    }
}

pub fn init() {
    for s in Bitboard::FULL {
        let white_attacks = attacks::generate_pawn_attacks::<White>(s);
        let black_attacks = attacks::generate_pawn_attacks::<Black>(s);

        unsafe {
            ATTACKS_TABLE[Player::White.array_idx()][s.array_idx()] = white_attacks;
            ATTACKS_TABLE[Player::Black.array_idx()][s.array_idx()] = black_attacks;
        }
    }
}
