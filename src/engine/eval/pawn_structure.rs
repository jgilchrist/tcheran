use crate::chess::board::Board;
use crate::chess::square;
use crate::engine::eval::PhasedEval;

fn doubled_pawn_malus(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    for file in square::FILES {
        let white_pawns = file.bitboard() & board.white_pieces().pawns();
        let black_pawns = file.bitboard() & board.black_pieces().pawns();

        if white_pawns.count() > 1 {
            eval += PhasedEval::new(-20, -20);
        }

        if black_pawns.count() > 1 {
            eval += PhasedEval::new(20, 20);
        }
    }

    eval
}

pub fn eval(board: &Board) -> PhasedEval {
    doubled_pawn_malus(board)
}
