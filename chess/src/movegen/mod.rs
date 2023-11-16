pub(crate) mod attackers;
mod gen;
mod tables;

pub use attackers::generate_attackers_of;
pub use gen::{generate_moves, MoveTypes};

pub fn init() {
    tables::init();
}
