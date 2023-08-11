// Piece square tables taken, for now, from https://www.chessprogramming.org/Simplified_Evaluation_Function

use chess::{game::Game, squares::Squares};

use super::Eval;

type PieceValueTable = [i32; 64];

#[rustfmt::skip]
mod tables {
    use super::*;

    pub const PAWN_TABLE_WHITE: PieceValueTable = [
        0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
        5,  5, 10, 25, 25, 10,  5,  5,
        0,  0,  0, 20, 20,  0,  0,  0,
        5, -5,-10,  0,  0,-10, -5,  5,
        5, 10, 10,-20,-20, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0,
    ];

    // TODO: Find a way to pre-process this based on flipping PAWN_TABLE_WHITE
    pub const PAWN_TABLE_BLACK: PieceValueTable = [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10,-20,-20, 10, 10,  5,
        5, -5,-10,  0,  0,-10, -5,  5,
        0,  0,  0, 20, 20,  0,  0,  0,
        5,  5, 10, 25, 25, 10,  5,  5,
        10, 10, 20, 30, 30, 20, 10, 10,
        50, 50, 50, 50, 50, 50, 50, 50,
        0,  0,  0,  0,  0,  0,  0,  0,
    ];

    pub const KNIGHT_TABLE_WHITE: PieceValueTable = [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ];

    pub const KNIGHT_TABLE_BLACK: PieceValueTable = [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ];

    pub const BISHOP_TABLE_WHITE: PieceValueTable = [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];

    pub const BISHOP_TABLE_BLACK: PieceValueTable = [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];

    pub const ROOK_TABLE_WHITE: PieceValueTable = [
        0,  0,  0,  0,  0,  0,  0,  0,
        5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  5,  5,  0,  0,  0
    ];

    pub const ROOK_TABLE_BLACK: PieceValueTable = [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        5, 10, 10, 10, 10, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0,
    ];

    pub const QUEEN_TABLE_WHITE: PieceValueTable = [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        20, 20,  0,  0,  0,  0, 20, 20,
        20, 30, 10,  0,  0, 10, 30, 20
    ];

    pub const QUEEN_TABLE_BLACK: PieceValueTable = [
        20, 30, 10,  0,  0, 10, 30, 20,
        20, 20,  0,  0,  0,  0, 20, 20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
    ];

    pub const KING_TABLE_WHITE: PieceValueTable = [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        20, 20,  0,  0,  0,  0, 20, 20,
        20, 30, 10,  0,  0, 10, 30, 20
    ];

    pub const KING_TABLE_BLACK: PieceValueTable = [
        20, 30, 10,  0,  0, 10, 30, 20,
        20, 20,  0,  0,  0,  0, 20, 20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
    ];
}


pub fn piece_square_tables(game: &Game) -> Eval {
    piece_square_tables_white(game) + piece_square_tables_black(game)
}

fn piece_contribution(pieces: Squares, piece_table: &PieceValueTable) -> i32 {
    pieces.iter()
        .map(|p| piece_table[p.array_idx()])
        .sum()
}

fn piece_square_tables_white(game: &Game) -> Eval {
    let pawn_score = piece_contribution(game.board.white_pieces.pawns, &tables::PAWN_TABLE_WHITE);
    let knight_score = piece_contribution(game.board.white_pieces.knights, &tables::KNIGHT_TABLE_WHITE);
    let bishops_score = piece_contribution(game.board.white_pieces.bishops, &tables::BISHOP_TABLE_WHITE);
    let rook_score = piece_contribution(game.board.white_pieces.rooks, &tables::ROOK_TABLE_WHITE);
    let queen_score = piece_contribution(game.board.white_pieces.queens, &tables::QUEEN_TABLE_WHITE);
    let king_score = piece_contribution(game.board.white_pieces.king, &tables::KING_TABLE_WHITE);

    Eval(
        pawn_score
            + knight_score
            + bishops_score
            + rook_score
            + queen_score
            + king_score
    )
}

fn piece_square_tables_black(game: &Game) -> Eval {
    // Piece contributions for black are reversed
    // Whereas for white, +50 would mean 'we are winning' and -50 would mean 'we are losing', for black
    // +50 should mean 'white is winning' and -50 should mean 'black is winning'.
    let pawn_score = -piece_contribution(game.board.black_pieces.pawns, &tables::PAWN_TABLE_BLACK);
    let knight_score = -piece_contribution(game.board.black_pieces.knights, &tables::KNIGHT_TABLE_BLACK);
    let bishops_score = -piece_contribution(game.board.black_pieces.bishops, &tables::BISHOP_TABLE_BLACK);
    let rook_score = -piece_contribution(game.board.black_pieces.rooks, &tables::ROOK_TABLE_BLACK);
    let queen_score = -piece_contribution(game.board.black_pieces.queens, &tables::QUEEN_TABLE_BLACK);
    let king_score = -piece_contribution(game.board.black_pieces.king, &tables::KING_TABLE_BLACK);

    -Eval(
        pawn_score
            + knight_score
            + bishops_score
            + rook_score
            + queen_score
            + king_score
    )
}
