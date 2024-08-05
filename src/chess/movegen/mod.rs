mod attackers;
mod gen;
mod pins;
pub mod tables;

pub use attackers::{all_attackers_of, generate_attackers_of};
pub use gen::{generate_captures, generate_legal_moves, generate_quiets, MovegenCache};

pub fn init() {
    tables::init();
}
