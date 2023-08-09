use chess::{game::Game, board::PlayerPieces};

use super::Eval;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 800;

pub fn material_diff(game: &Game) -> Eval {
    white_piece_value(game) + black_piece_value(game)
}

fn white_piece_value(game: &Game) -> Eval {
    count_piece_values(&game.board.white_pieces)
}

fn black_piece_value(game: &Game) -> Eval {
    -count_piece_values(&game.board.black_pieces)
}

fn count_piece_values(pieces: &PlayerPieces) -> Eval {
    Eval(
        i32::from(pieces.pawns.count()) * PAWN_VALUE
            + i32::from(pieces.knights.count()) * KNIGHT_VALUE
            + i32::from(pieces.bishops.count()) * BISHOP_VALUE
            + i32::from(pieces.rooks.count()) * ROOK_VALUE
            + i32::from(pieces.queens.count()) * QUEEN_VALUE,
    )
}
