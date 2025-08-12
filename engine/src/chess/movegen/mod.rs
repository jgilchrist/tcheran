mod attackers;
mod moves;
mod pins;
pub mod tables;

pub use attackers::{all_attackers_of, generate_attackers_of};
pub use moves::{MovegenCache, generate_captures, generate_legal_moves, generate_quiets};

pub fn init() {
    tables::init();
}
