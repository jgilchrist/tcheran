use chess::{
    board::PlayerPieces,
    game::Game,
    piece::PieceKind,
    square::{File, Rank},
    squares::Squares,
};

use super::Eval;

type PieceValueTableDefinition = [[i32; File::N]; Rank::N];
type PieceValueTable = [i32; Squares::N];
type PieceValueTables = [PieceValueTable; PieceKind::N];

#[rustfmt::skip]
mod tables {
    use super::*;

    // Piece square tables taken, for now, from https://www.chessprogramming.org/Simplified_Evaluation_Function

    pub const PAWNS: PieceValueTableDefinition = [
        [   0,   0,   0,   0,   0,   0,   0,   0],
        [  50,  50,  50,  50,  50,  50,  50,  50],
        [  10,  10,  20,  30,  30,  20,  10,  10],
        [   5,   5,  10,  25,  25,  10,   5,   5],
        [   0,   0,   0,  20,  20,   0,   0,   0],
        [   5,  -5, -10,   0,   0, -10,  -5,   5],
        [   5,  10,  10, -20, -20,  10,  10,   5],
        [   0,   0,   0,   0,   0,   0,   0,   0],
    ];

    pub const KNIGHTS: PieceValueTableDefinition = [
        [ -50, -40, -30, -30, -30, -30, -40, -50],
        [ -40, -20,   0,   0,   0,   0, -20, -40],
        [ -30,   0,  10,  15,  15,  10,   0, -30],
        [ -30,   5,  15,  20,  20,  15,   5, -30],
        [ -30,   0,  15,  20,  20,  15,   0, -30],
        [ -30,   5,  10,  15,  15,  10,   5, -30],
        [ -40, -20,   0,   5,   5,   0, -20, -40],
        [ -50, -40, -30, -30, -30, -30, -40, -50],
    ];

    pub const BISHOPS: PieceValueTableDefinition = [
        [ -20, -10, -10, -10, -10, -10, -10, -20],
        [ -10,   0,   0,   0,   0,   0,   0, -10],
        [ -10,   0,   5,  10,  10,   5,   0, -10],
        [ -10,   5,   5,  10,  10,   5,   5, -10],
        [ -10,   0,  10,  10,  10,  10,   0, -10],
        [ -10,  10,  10,  10,  10,  10,  10, -10],
        [ -10,   5,   0,   0,   0,   0,   5, -10],
        [ -20, -10, -10, -10, -10, -10, -10, -20],
    ];

    pub const ROOKS: PieceValueTableDefinition = [
        [   0,   0,   0,   0,   0,   0,   0,   0],
        [   5,  10,  10,  10,  10,  10,  10,   5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [   0,   0,   0,   5,   5,   0,   0,   0],
    ];

    pub const QUEENS: PieceValueTableDefinition = [
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -20, -30, -30, -40, -40, -30, -30, -20],
        [ -10, -20, -20, -20, -20, -20, -20, -10],
        [  20,  20,   0,   0,   0,   0,  20,  20],
        [  20,  30,  10,   0,   0,  10,  30,  20],
    ];

    pub const KINGS: PieceValueTableDefinition = [
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -20, -30, -30, -40, -40, -30, -30, -20],
        [ -10, -20, -20, -20, -20, -20, -20, -10],
        [  20,  20,   0,   0,   0,   0,  20,  20],
        [  20,  30,  10,   0,   0,  10,  30,  20],
    ];

    // These need to be initialised when we start up, since they can
    // be derived from the white tables.
    pub static mut WHITE_TABLES: PieceValueTables = [[0; Squares::N]; PieceKind::N];
    pub static mut BLACK_TABLES: PieceValueTables = [[0; Squares::N]; PieceKind::N];

    pub fn flip(t: PieceValueTableDefinition) -> PieceValueTableDefinition {
        let mut new_table: PieceValueTableDefinition = Default::default();

        for i in 0..Rank::N {
            new_table[i] = t[Rank::N - i - 1];
        }

        new_table
    }

    pub fn flatten(definition: PieceValueTableDefinition) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Squares::N];

        for i in 0..Squares::N {
            new_table[i] = definition[i / File::N][i % Rank::N];
        }

        new_table
    }
}

pub fn init() {
    use tables::*;

    unsafe {
        tables::WHITE_TABLES[PieceKind::Pawn.array_idx()] = flatten(tables::PAWNS);
        tables::WHITE_TABLES[PieceKind::Knight.array_idx()] = flatten(tables::KNIGHTS);
        tables::WHITE_TABLES[PieceKind::Bishop.array_idx()] = flatten(tables::BISHOPS);
        tables::WHITE_TABLES[PieceKind::Rook.array_idx()] = flatten(tables::ROOKS);
        tables::WHITE_TABLES[PieceKind::Queen.array_idx()] = flatten(tables::QUEENS);
        tables::WHITE_TABLES[PieceKind::King.array_idx()] = flatten(tables::KINGS);

        tables::BLACK_TABLES[PieceKind::Pawn.array_idx()] = flatten(flip(tables::PAWNS));
        tables::BLACK_TABLES[PieceKind::Knight.array_idx()] = flatten(flip(tables::KNIGHTS));
        tables::BLACK_TABLES[PieceKind::Bishop.array_idx()] = flatten(flip(tables::BISHOPS));
        tables::BLACK_TABLES[PieceKind::Rook.array_idx()] = flatten(flip(tables::ROOKS));
        tables::BLACK_TABLES[PieceKind::Queen.array_idx()] = flatten(flip(tables::QUEENS));
        tables::BLACK_TABLES[PieceKind::King.array_idx()] = flatten(flip(tables::KINGS));
    }
}

pub fn piece_square_tables(game: &Game) -> Eval {
    piece_square_tables_white(game) + piece_square_tables_black(game)
}

pub fn piece_square_tables_white(game: &Game) -> Eval {
    let eval = all_piece_contributions(&game.board.white_pieces, unsafe { &tables::WHITE_TABLES });
    Eval(eval)
}

pub fn piece_square_tables_black(game: &Game) -> Eval {
    let eval = all_piece_contributions(&game.board.black_pieces, unsafe { &tables::BLACK_TABLES });
    -Eval(eval)
}

fn all_piece_contributions(pieces: &PlayerPieces, tables: &PieceValueTables) -> i32 {
    let pawn_score = piece_contribution(pieces.pawns, tables[PieceKind::Pawn.array_idx()]);
    let knight_score = piece_contribution(pieces.knights, tables[PieceKind::Knight.array_idx()]);
    let bishops_score = piece_contribution(pieces.bishops, tables[PieceKind::Bishop.array_idx()]);
    let rook_score = piece_contribution(pieces.rooks, tables[PieceKind::Rook.array_idx()]);
    let queen_score = piece_contribution(pieces.queens, tables[PieceKind::Queen.array_idx()]);
    let king_score = piece_contribution(pieces.king, tables[PieceKind::King.array_idx()]);

    pawn_score + knight_score + bishops_score + rook_score + queen_score + king_score
}

fn piece_contribution(pieces: Squares, piece_table: PieceValueTable) -> i32 {
    pieces.iter().map(|p| piece_table[p.array_idx()]).sum()
}
