#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always
)]

pub mod bitboard;
pub mod board;
pub mod consts;
pub mod direction;
pub mod game;
pub mod moves;
pub mod piece;
pub mod player;
pub mod square;
pub mod squares;

mod attacks;
pub mod fen;
mod movegen;
