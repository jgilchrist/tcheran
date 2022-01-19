#![allow(dead_code)]

pub mod bitboard;

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
