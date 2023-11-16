mod gen;
mod tables;

pub use gen::{generate_attackers_of, generate_moves, MoveTypes};

pub fn init() {
    tables::init();
}
