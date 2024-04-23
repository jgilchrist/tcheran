use crate::chess::board::Board;
use crate::chess::player::Player;
use crate::chess::square;
use crate::engine::eval::PhasedEval;

const DOUBLED_PAWN_MALUS: PhasedEval = PhasedEval::new(-5, -15);

fn doubled_pawn_malus(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    let white_pawns = board.pawns(Player::White);
    let black_pawns = board.pawns(Player::Black);

    for file in square::FILES {
        let bb = file.bitboard();

        if (white_pawns & bb).count() > 1 {
            eval += DOUBLED_PAWN_MALUS;
        }

        if (black_pawns & bb).count() > 1 {
            eval -= DOUBLED_PAWN_MALUS;
        }
    }

    eval
}

pub fn eval(board: &Board) -> PhasedEval {
    doubled_pawn_malus(board)
}
