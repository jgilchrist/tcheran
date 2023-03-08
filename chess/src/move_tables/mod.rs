mod magics;
mod occupancies;

pub use magics::bishop_attacks;
pub use magics::rook_attacks;

pub fn init() {
    magics::init();
}
