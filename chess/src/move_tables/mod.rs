mod attacks;
mod magics;
mod occupancies;

pub use magics::bishop_attacks;
pub use magics::rook_attacks;

use crate::square::Square;
use crate::squares::Squares;

pub fn queen_attacks(s: Square, blockers: Squares) -> Squares {
    rook_attacks(s, blockers) | bishop_attacks(s, blockers)
}

pub fn init() {
    magics::init();
}
