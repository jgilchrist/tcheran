use crate::chess::board::Board;
use crate::chess::square;
use crate::engine::eval::PhasedEval;

const DOUBLED_PAWN_MALUS: PhasedEval = PhasedEval::new(-20, -20);

fn doubled_pawn_malus(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    let white_pawns = board.white_pieces().pawns();
    let black_pawns = board.black_pieces().pawns();

    for file in square::FILES {
        if (white_pawns & file.bitboard()).count() > 1 {
            eval += DOUBLED_PAWN_MALUS;
        }

        if (black_pawns & file.bitboard()).count() > 1 {
            eval -= DOUBLED_PAWN_MALUS;
        }
    }

    eval
}

pub fn eval(board: &Board) -> PhasedEval {
    doubled_pawn_malus(board)
}
