mod attackers;
mod legal_gen;
mod pins;
mod pseudo_legal_gen;
mod tables;

pub use attackers::generate_attackers_of;
pub use legal_gen::{generate_legal_moves, get_legal_moves};

#[allow(unused)]
pub use pseudo_legal_gen::{generate_pseudo_legal_moves, get_pseudo_legal_moves};

pub fn init() {
    tables::init();
}
