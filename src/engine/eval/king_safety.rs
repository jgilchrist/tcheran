use crate::chess;
use crate::chess::board::Board;
use crate::chess::piece::PieceKind;
use crate::chess::player::Player;
use crate::engine::eval::{Eval, PhasedEval};

pub const EMPTY_KING_FILE_MALUS: PhasedEval = PhasedEval::new(-10, -10);

fn tropism(board: &Board, player: Player) -> i16 {
    let our_king = board.pieces(player).king().single();

    let their_pieces = board.pieces(!player);

    let their_dangerous_pieces = their_pieces.queens()
        | their_pieces.rooks()
        | their_pieces.bishops()
        | their_pieces.knights();

    let mut eval = 0;

    for sq in their_dangerous_pieces {
        let piece_danger = match board.piece_at(sq).unwrap().kind {
            PieceKind::Queen | PieceKind::Rook => 5,
            PieceKind::Bishop | PieceKind::Knight => 2,
            _ => unreachable!(),
        };

        eval += i16::from((14 - sq.manhattan_distance_to(our_king)) * piece_danger);
    }

    -eval
}

pub fn eval(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    let white_tropism = tropism(board, Player::White);
    let black_tropism = tropism(board, Player::Black);

    eval += PhasedEval::uniform(white_tropism);
    eval -= PhasedEval::uniform(black_tropism);

    // let all_pieces = board.white_pieces().all() & board.black_pieces().all();
    //
    // let white_king = board.white_pieces().king().single();
    // let black_king = board.black_pieces().king().single();
    //
    // let vertical_empty_file_by_white_king =
    //     (all_pieces & white_king.file().bitboard()).count() == 1;
    // let vertical_empty_file_by_black_king =
    //     (all_pieces & black_king.file().bitboard()).count() == 1;
    //
    // if vertical_empty_file_by_white_king {
    //     eval += EMPTY_KING_FILE_MALUS;
    // }
    //
    // if vertical_empty_file_by_black_king {
    //     eval -= EMPTY_KING_FILE_MALUS;
    // }

    eval
}
