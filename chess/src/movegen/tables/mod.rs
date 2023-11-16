mod attacks;
mod king;
mod knights;
mod magics;
mod occupancies;
mod pawns;

use crate::bitboard::Bitboard;
pub use king::king_attacks;
pub use knights::knight_attacks;
pub use magics::bishop_attacks;
pub use magics::rook_attacks;
pub use pawns::pawn_attacks;

use crate::square::Square;

#[inline]
pub fn queen_attacks(s: Square, blockers: Bitboard) -> Bitboard {
    rook_attacks(s, blockers) | bishop_attacks(s, blockers)
}

pub fn init() {
    magics::init();
    knights::init();
    king::init();
    pawns::init();
}
