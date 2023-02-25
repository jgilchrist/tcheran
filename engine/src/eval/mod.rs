use chess::game::Game;

const PAWN_VALUE: u64 = 100;
const KNIGHT_VALUE: u64 = 300;
const BISHOP_VALUE: u64 = 300;
const ROOK_VALUE: u64 = 500;
const QUEEN_VALUE: u64 = 800;

#[allow(clippy::cast_possible_wrap)]
pub fn eval(game: &Game) -> i64 {
    white_piece_value(game) as i64 - black_piece_value(game) as i64
}

fn white_piece_value(game: &Game) -> u64 {
    let pieces = &game.board.white_pieces;

    u64::from(pieces.pawns.count()) * PAWN_VALUE
        + u64::from(pieces.knights.count()) * KNIGHT_VALUE
        + u64::from(pieces.bishops.count()) * BISHOP_VALUE
        + u64::from(pieces.rooks.count()) * ROOK_VALUE
        + u64::from(pieces.queens.count()) * QUEEN_VALUE
}

fn black_piece_value(game: &Game) -> u64 {
    let pieces = &game.board.black_pieces;

    u64::from(pieces.pawns.count()) * PAWN_VALUE
        + u64::from(pieces.knights.count()) * KNIGHT_VALUE
        + u64::from(pieces.bishops.count()) * BISHOP_VALUE
        + u64::from(pieces.rooks.count()) * ROOK_VALUE
        + u64::from(pieces.queens.count()) * QUEEN_VALUE
}
