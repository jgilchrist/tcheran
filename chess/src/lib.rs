#![allow(dead_code)]

pub mod bitboard;
pub mod board;
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
