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

pub mod movegen;
pub mod zobrist;

pub fn init() {
    movegen::init();
    zobrist::init();
}
