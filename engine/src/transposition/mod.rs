use chess::zobrist;

pub mod transposition_table;

pub fn init() {
    zobrist::init();
}
