mod attackers;
mod gen;
mod pins;
mod tables;

pub use attackers::generate_attackers_of;
pub use gen::{generate_captures, generate_legal_moves, generate_quiets, MovegenCache};

pub fn init() {
    tables::init();
}
