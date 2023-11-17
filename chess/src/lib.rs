pub mod bitboard;
pub mod board;
pub mod direction;
pub mod fen;
pub mod game;
pub mod moves;
pub mod perft;
pub mod piece;
pub mod player;
pub mod square;

mod move_tables;
pub mod movegen;
pub mod util;
pub mod zobrist;

pub fn init() {
    move_tables::init();
    zobrist::init();
}
