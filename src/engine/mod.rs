pub mod eval;
pub mod options;
pub mod uci;
pub mod util;

pub mod see;

pub mod search;
pub mod transposition_table;

pub fn init() {
    eval::init();
    search::init();
}
