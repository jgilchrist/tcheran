#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

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
