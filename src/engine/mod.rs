pub mod eval;
pub mod options;
pub mod strategy;
pub mod uci;
pub mod util;

pub mod search;
pub mod transposition_table;

pub fn init() {
    eval::init();
}
