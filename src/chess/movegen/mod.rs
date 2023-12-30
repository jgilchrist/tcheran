mod attackers;
mod gen;
mod pins;
mod tables;

pub use attackers::generate_attackers_of;
pub use gen::generate_moves;

pub fn init() {
    tables::init();
}
