#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::missing_const_for_fn,
    clippy::too_many_lines,
    clippy::cognitive_complexity
)]

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
pub mod squares;

mod move_tables;
mod movegen;
pub mod util;
pub mod zobrist;

pub fn init() {
    move_tables::init();
    zobrist::init();
}
