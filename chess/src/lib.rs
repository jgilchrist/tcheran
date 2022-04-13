#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bitboard;
pub mod board;
pub mod consts;
pub mod r#move;
pub mod square;

pub enum Player {
    White,
    Black,
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
