#![allow(dead_code)]
#![allow(unused_variables)]

use chess::{
    board::Board,
    r#move::Move,
    square::{File, Rank},
};

pub mod uci;

fn run(board: &Board) -> Move {
    Move::new(
        chess::square::Square(File::E, Rank::R7),
        chess::square::Square(File::E, Rank::R5),
    )
}
