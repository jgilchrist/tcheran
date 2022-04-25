use chess::game::Game;

const PAWN_VALUE: u64 = 100;
const KNIGHT_VALUE: u64 = 300;
const BISHOP_VALUE: u64 = 300;
const ROOK_VALUE: u64 = 500;
const QUEEN_VALUE: u64 = 800;

pub fn eval(game: &Game) -> i64 {
    white_piece_value(game) as i64 - black_piece_value(game) as i64
}

fn white_piece_value(game: &Game) -> u64 {
    let pieces = &game.board.white_pieces;

    pieces.pawns.count() as u64 * PAWN_VALUE
        + pieces.knights.count() as u64 * KNIGHT_VALUE
        + pieces.bishops.count() as u64 * BISHOP_VALUE
        + pieces.rooks.count() as u64 * ROOK_VALUE
        + pieces.queen.count() as u64 * QUEEN_VALUE
}

fn black_piece_value(game: &Game) -> u64 {
    let pieces = &game.board.black_pieces;

    pieces.pawns.count() as u64 * PAWN_VALUE
        + pieces.knights.count() as u64 * KNIGHT_VALUE
        + pieces.bishops.count() as u64 * BISHOP_VALUE
        + pieces.rooks.count() as u64 * ROOK_VALUE
        + pieces.queen.count() as u64 * QUEEN_VALUE
}
