use chess::{board::Board, r#move::Move, square::Square};

pub mod uci;

fn run(_board: &Board) -> Move {
    Move::new(Square::E7, Square::E5)
}
