#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always
)]

pub mod bitboard;
pub mod board;
pub mod direction;
pub mod fen;
pub mod game;
pub mod moves;
pub mod piece;
pub mod player;
pub mod square;
pub mod squares;

mod move_tables;
mod movegen;

pub fn init() {
    move_tables::init();
}
