#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bitboard;
pub mod board;
pub mod consts;
pub mod r#move;
pub mod square;

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
