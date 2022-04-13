#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bitboard;
pub mod board;
pub mod consts;
pub mod square;

enum Player {
    White,
    Black,
}

enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
