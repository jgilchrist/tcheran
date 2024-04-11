mod attackers;
mod legal_gen;
mod pins;
mod tables;

pub use attackers::generate_attackers_of;
pub use legal_gen::{generate_legal_moves, get_legal_moves};

pub fn init() {
    tables::init();
}
