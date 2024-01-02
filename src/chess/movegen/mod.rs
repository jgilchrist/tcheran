mod attackers;
mod gen;
mod pins;
mod tables;

pub use attackers::generate_attackers_of;
pub use gen::{generate_captures, generate_moves, generate_quiets};

pub fn init() {
    tables::init();
}
