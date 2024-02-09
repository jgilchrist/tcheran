use crate::chess;
use crate::chess::board::Board;
use crate::chess::player::Player;
use crate::chess::square::Square;
use crate::engine::eval::piece_square_tables::piece_contributions;
use crate::engine::eval::PhasedEval;

pub const EMPTY_KING_FILE_MALUS: PhasedEval = PhasedEval::new(-10, -10);

pub fn eval(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    let all_pieces = board.white_pieces().all() & board.black_pieces().all();

    let white_king = board.white_pieces().king().single();
    let black_king = board.black_pieces().king().single();

    let vertical_empty_file_by_white_king =
        (all_pieces & white_king.file().bitboard()).count() == 1;
    let vertical_empty_file_by_black_king =
        (all_pieces & black_king.file().bitboard()).count() == 1;

    if vertical_empty_file_by_white_king {
        eval += EMPTY_KING_FILE_MALUS;
    }

    if vertical_empty_file_by_black_king {
        eval -= EMPTY_KING_FILE_MALUS;
    }

    eval
}
